//! Instagram OAuth handlers

use worker::*;

use super::get_base_url;
use crate::crypto;
use crate::helpers::*;
use crate::instagram;
use crate::storage::*;
use crate::types::*;

/// Handle Instagram OAuth routes (/instagram/*)
pub async fn handle_instagram(req: Request, env: Env, path: &str, method: Method) -> Result<Response> {
    let base_url = get_base_url(&req);
    let kv = env.kv("CALENDARS_KV")?;

    let app_id = env
        .secret("INSTAGRAM_APP_ID")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let app_secret = env
        .secret("INSTAGRAM_APP_SECRET")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let encryption_key = env
        .secret("ENCRYPTION_KEY")
        .map(|s| s.to_string())
        .unwrap_or_default();

    let path_parts: Vec<&str> = path
        .strip_prefix("/instagram/")
        .unwrap_or("")
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    match (method, path_parts.as_slice()) {
        (Method::Get, ["auth", calendar_id]) => {
            if app_id.is_empty() || app_secret.is_empty() {
                return Response::error("Instagram integration not configured", 500);
            }

            if get_calendar(&kv, calendar_id).await?.is_none() {
                return Response::error("Calendar not found", 404);
            }

            let state = format!("{}:{}", calendar_id, generate_token());
            let redirect_uri = format!("{}/instagram/callback", base_url);

            kv.put(&format!("instagram_oauth_state:{}", state), calendar_id)?
                .expiration_ttl(600)
                .execute()
                .await?;

            let auth_url = instagram::get_auth_url(&app_id, &redirect_uri, &state);

            let headers = Headers::new();
            headers.set("Location", &auth_url)?;
            Ok(Response::empty()?.with_status(302).with_headers(headers))
        }

        (Method::Get, ["callback"]) => {
            let url = req.url()?;
            let query_pairs: std::collections::HashMap<_, _> = url.query_pairs().collect();

            if let Some(error) = query_pairs.get("error") {
                return Response::error(format!("Instagram error: {}", error), 400);
            }

            let code = query_pairs
                .get("code")
                .ok_or_else(|| Error::from("Missing authorization code"))?;
            let state = query_pairs
                .get("state")
                .ok_or_else(|| Error::from("Missing state parameter"))?;

            let state_key = format!("instagram_oauth_state:{}", state);
            let calendar_id = kv
                .get(&state_key)
                .text()
                .await?
                .ok_or_else(|| Error::from("Invalid or expired state"))?;

            kv.delete(&state_key).await?;

            let redirect_uri = format!("{}/instagram/callback", base_url);
            let short_token = instagram::exchange_code_for_token(code, &app_id, &app_secret, &redirect_uri).await?;
            let token = instagram::exchange_for_long_lived_token(&short_token, &app_secret).await?;

            let client = instagram::InstagramClient::new(token.access_token.clone());
            let (user_id, username) = client.get_user_info().await?;

            let source_id = generate_id();
            let source = InstagramSource {
                id: source_id.clone(),
                instagram_user_id: user_id.clone(),
                instagram_username: username.clone(),
                enabled: true,
                last_synced_at: None,
                created_at: now_iso(),
            };

            let mut calendar = get_calendar(&kv, &calendar_id)
                .await?
                .ok_or_else(|| Error::from("Calendar not found"))?;

            if calendar
                .instagram_sources
                .iter()
                .any(|s| s.instagram_user_id == user_id)
            {
                let headers = Headers::new();
                headers.set(
                    "Location",
                    &format!("{}/admin/calendars/{}?error=instagram_already_connected", base_url, calendar_id),
                )?;
                return Ok(Response::empty()?.with_status(302).with_headers(headers));
            }

            calendar.instagram_sources.push(source);
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;

            let encrypted = crypto::encrypt_token(&token, &encryption_key).await?;
            kv.put(
                &format!("instagram_token:{}:{}", calendar_id, source_id),
                encrypted,
            )?
            .execute()
            .await?;

            let headers = Headers::new();
            headers.set(
                "Location",
                &format!("{}/admin/calendars/{}?success=instagram_connected", base_url, calendar_id),
            )?;
            Ok(Response::empty()?.with_status(302).with_headers(headers))
        }

        (Method::Delete, ["disconnect", calendar_id, source_id]) => {
            let mut calendar = get_calendar(&kv, calendar_id)
                .await?
                .ok_or_else(|| Error::from("Calendar not found"))?;

            calendar.instagram_sources.retain(|s| s.id != *source_id);
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;

            kv.delete(&format!("instagram_token:{}:{}", calendar_id, source_id))
                .await?;

            Response::empty()
        }

        _ => Response::error("Not Found", 404),
    }
}
