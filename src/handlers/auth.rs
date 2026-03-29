//! Authentication handlers - Google OAuth

use serde::Deserialize;
use worker::*;

use super::get_base_url;
use crate::helpers::*;
use crate::storage::*;
use crate::templates::auth_login_html;
use crate::types::*;

const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";

const SESSION_TTL_SECONDS: u64 = 7 * 24 * 60 * 60; // 7 days

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct UserInfo {
    email: String,
    name: Option<String>,
}

/// Handle auth routes (/auth/*)
pub async fn handle_auth(req: Request, env: Env, path: &str, method: Method) -> Result<Response> {
    let base_url = get_base_url(&req);

    match (method, path) {
        (Method::Get, "/auth/login") => {
            let client_id = env
                .secret("GOOGLE_OAUTH_CLIENT_ID")
                .map(|s| s.to_string())
                .unwrap_or_default();

            if client_id.is_empty() {
                return Response::error(
                    "Google OAuth not configured. Set GOOGLE_OAUTH_CLIENT_ID secret.",
                    500,
                );
            }

            let html = auth_login_html(&base_url, &client_id);
            Response::from_html(html)
        }

        (Method::Get, "/auth/callback") => {
            let url = req.url()?;
            let query: std::collections::HashMap<_, _> = url.query_pairs().collect();

            let code = match query.get("code") {
                Some(c) => c.to_string(),
                None => {
                    let error = query
                        .get("error")
                        .map(|e| e.to_string())
                        .unwrap_or_default();
                    return Response::error(format!("OAuth error: {}", error), 400);
                }
            };

            let client_id = env.secret("GOOGLE_OAUTH_CLIENT_ID")?.to_string();
            let client_secret = env.secret("GOOGLE_OAUTH_CLIENT_SECRET")?.to_string();
            let redirect_uri = format!("{}/auth/callback", base_url);

            // Exchange code for access token
            let token_body = format!(
                "code={}&client_id={}&client_secret={}&redirect_uri={}&grant_type=authorization_code",
                urlencoding::encode(&code),
                urlencoding::encode(&client_id),
                urlencoding::encode(&client_secret),
                urlencoding::encode(&redirect_uri),
            );

            let headers = Headers::new();
            headers.set("Content-Type", "application/x-www-form-urlencoded")?;

            let mut init = RequestInit::new();
            init.with_method(Method::Post)
                .with_headers(headers)
                .with_body(Some(wasm_bindgen::JsValue::from_str(&token_body)));

            let token_req = Request::new_with_init(GOOGLE_TOKEN_URL, &init)?;
            let mut token_resp = Fetch::Request(token_req).send().await?;
            let token_text = token_resp.text().await?;

            if token_resp.status_code() != 200 {
                return Response::error(format!("Token exchange failed: {}", token_text), 500);
            }

            let tokens: TokenResponse = serde_json::from_str(&token_text)
                .map_err(|e| Error::from(format!("Failed to parse token response: {}", e)))?;

            // Get user info
            let headers = Headers::new();
            headers.set("Authorization", &format!("Bearer {}", tokens.access_token))?;

            let mut init = RequestInit::new();
            init.with_method(Method::Get).with_headers(headers);

            let userinfo_req = Request::new_with_init(GOOGLE_USERINFO_URL, &init)?;
            let mut userinfo_resp = Fetch::Request(userinfo_req).send().await?;
            let userinfo_text = userinfo_resp.text().await?;

            if userinfo_resp.status_code() != 200 {
                return Response::error("Failed to get user info", 500);
            }

            let user: UserInfo = serde_json::from_str(&userinfo_text)
                .map_err(|e| Error::from(format!("Failed to parse user info: {}", e)))?;

            let kv = env.kv("CALENDARS_KV")?;

            // Find or create tenant
            let tenant = match get_tenant_by_email(&kv, &user.email).await? {
                Some(t) => t,
                None => {
                    let now = now_iso();
                    let tenant = Tenant {
                        id: generate_id(),
                        email: user.email.clone(),
                        name: user.name,
                        plan: "free".to_string(),
                        created_at: now.clone(),
                        updated_at: now,
                    };
                    save_tenant(&kv, &tenant).await?;
                    tenant
                }
            };

            // Create session
            let session_token = generate_token();
            save_session(&kv, &session_token, &tenant.id, SESSION_TTL_SECONDS).await?;

            // Set cookie and redirect to admin
            let headers = Headers::new();
            headers.set("Location", "/admin")?;
            headers.set(
                "Set-Cookie",
                &format!(
                    "session={}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={}",
                    session_token, SESSION_TTL_SECONDS
                ),
            )?;

            Ok(Response::empty()?.with_status(302).with_headers(headers))
        }

        (Method::Get, "/auth/logout") => {
            // Clear session from KV if exists
            if let Some(session_token) = get_session_cookie(&req) {
                let kv = env.kv("CALENDARS_KV")?;
                delete_session(&kv, &session_token).await?;
            }

            let headers = Headers::new();
            headers.set("Location", "/auth/login")?;
            headers.set(
                "Set-Cookie",
                "session=; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=0",
            )?;

            Ok(Response::empty()?.with_status(302).with_headers(headers))
        }

        _ => Response::error("Not Found", 404),
    }
}

/// Extract session cookie from request
pub fn get_session_cookie(req: &Request) -> Option<String> {
    let cookie_header = req.headers().get("Cookie").ok()??;
    for part in cookie_header.split(';') {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("session=") {
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

/// Resolve tenant_id from session cookie, returns None if not authenticated
pub async fn resolve_tenant_id(req: &Request, kv: &kv::KvStore) -> Option<String> {
    let token = get_session_cookie(req)?;
    get_session(kv, &token).await.ok()?
}
