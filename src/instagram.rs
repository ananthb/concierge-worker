use wasm_bindgen::JsValue;
use worker::*;

use crate::types::InstagramToken;

const GRAPH_API_URL: &str = "https://graph.facebook.com/v21.0";
const FACEBOOK_OAUTH_URL: &str = "https://www.facebook.com/v21.0/dialog/oauth";

/// Generate Facebook Login OAuth authorization URL
pub fn get_auth_url(app_id: &str, redirect_uri: &str, state: &str) -> String {
    let scopes = "instagram_basic,instagram_manage_messages,pages_manage_metadata,pages_messaging";
    format!(
        "{}?client_id={}&redirect_uri={}&scope={}&response_type=code&state={}",
        FACEBOOK_OAUTH_URL,
        app_id,
        urlencoding_encode(redirect_uri),
        urlencoding_encode(scopes),
        state
    )
}

/// Exchange authorization code for access token via Facebook Login
pub async fn exchange_code_for_token(
    code: &str,
    app_id: &str,
    app_secret: &str,
    redirect_uri: &str,
) -> Result<String> {
    let url = format!(
        "{}/oauth/access_token?client_id={}&redirect_uri={}&client_secret={}&code={}",
        GRAPH_API_URL,
        app_id,
        urlencoding_encode(redirect_uri),
        app_secret,
        code
    );

    let mut init = RequestInit::new();
    init.with_method(Method::Get);

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        let error_text = response.text().await.unwrap_or_default();
        return Err(Error::from(format!(
            "Token exchange error {}: {}",
            response.status_code(),
            error_text
        )));
    }

    let body: serde_json::Value = response.json().await?;

    body.get("access_token")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| Error::from("Missing access_token"))
}

/// Exchange short-lived token for long-lived token (60 days)
pub async fn exchange_for_long_lived_token(
    short_lived_token: &str,
    app_id: &str,
    app_secret: &str,
) -> Result<InstagramToken> {
    let url = format!(
        "{}/oauth/access_token?grant_type=fb_exchange_token&client_id={}&client_secret={}&fb_exchange_token={}",
        GRAPH_API_URL, app_id, app_secret, short_lived_token
    );

    let mut init = RequestInit::new();
    init.with_method(Method::Get);

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        let error_text = response.text().await.unwrap_or_default();
        return Err(Error::from(format!(
            "Long-lived token exchange error {}: {}",
            response.status_code(),
            error_text
        )));
    }

    let body: serde_json::Value = response.json().await?;

    let access_token = body
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::from("Missing access_token"))?
        .to_string();

    let expires_in = body
        .get("expires_in")
        .and_then(|v| v.as_i64())
        .unwrap_or(5184000);

    let expires_at = calculate_expiry(expires_in);

    Ok(InstagramToken {
        access_token,
        expires_at,
        user_id: String::new(), // Will be set by caller
    })
}

/// Get user's Facebook Pages
pub async fn get_user_pages(access_token: &str) -> Result<Vec<(String, String, String)>> {
    let url = format!(
        "{}/me/accounts?fields=id,name,access_token&access_token={}",
        GRAPH_API_URL, access_token
    );

    let mut init = RequestInit::new();
    init.with_method(Method::Get);

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        let error_text = response.text().await.unwrap_or_default();
        return Err(Error::from(format!(
            "Pages API error {}: {}",
            response.status_code(),
            error_text
        )));
    }

    let body: serde_json::Value = response.json().await?;
    let pages = body
        .get("data")
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let id = item.get("id")?.as_str()?.to_string();
                    let name = item.get("name")?.as_str()?.to_string();
                    let token = item.get("access_token")?.as_str()?.to_string();
                    Some((id, name, token))
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(pages)
}

/// Get the Instagram Business Account connected to a Facebook Page
pub async fn get_instagram_business_account(
    page_id: &str,
    page_access_token: &str,
) -> Result<Option<(String, String)>> {
    let url = format!(
        "{}/{}?fields=instagram_business_account{{id,username}}&access_token={}",
        GRAPH_API_URL, page_id, page_access_token
    );

    let mut init = RequestInit::new();
    init.with_method(Method::Get);

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        return Ok(None);
    }

    let body: serde_json::Value = response.json().await?;

    let ig = match body.get("instagram_business_account") {
        Some(ig) => ig,
        None => return Ok(None),
    };

    let ig_id = ig
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let ig_username = ig
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if ig_id.is_empty() {
        return Ok(None);
    }

    Ok(Some((ig_id, ig_username)))
}

/// Send an Instagram DM via the Pages API
pub async fn send_instagram_dm(
    page_access_token: &str,
    page_id: &str,
    recipient_id: &str,
    text: &str,
) -> Result<()> {
    let url = format!("{}/{}/messages", GRAPH_API_URL, page_id);

    let payload = serde_json::json!({
        "recipient": {"id": recipient_id},
        "message": {"text": text}
    });

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", page_access_token))?;
    headers.set("Content-Type", "application/json")?;

    let request = Request::new_with_init(
        &url,
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(wasm_bindgen::JsValue::from_str(&payload.to_string()))),
    )?;

    let response = Fetch::Request(request).send().await?;

    if !response.status_code().to_string().starts_with('2') {
        console_log!("Instagram DM API error: status {}", response.status_code());
    }

    Ok(())
}

/// Refresh a long-lived token
pub async fn refresh_token(current_token: &str) -> Result<InstagramToken> {
    let url = format!(
        "{}/oauth/access_token?grant_type=fb_exchange_token&fb_exchange_token={}",
        GRAPH_API_URL, current_token
    );

    let mut init = RequestInit::new();
    init.with_method(Method::Get);

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        let error_text = response.text().await.unwrap_or_default();
        return Err(Error::from(format!(
            "Token refresh error {}: {}",
            response.status_code(),
            error_text
        )));
    }

    let body: serde_json::Value = response.json().await?;

    let access_token = body
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::from("Missing access_token"))?
        .to_string();

    let expires_in = body
        .get("expires_in")
        .and_then(|v| v.as_i64())
        .unwrap_or(5184000);

    let expires_at = calculate_expiry(expires_in);

    Ok(InstagramToken {
        access_token,
        expires_at,
        user_id: String::new(),
    })
}

/// Check if token needs refresh (within 7 days of expiry)
pub fn token_needs_refresh(token: &InstagramToken) -> bool {
    let now = js_sys::Date::now() as i64;
    let expires = parse_iso_timestamp(&token.expires_at).unwrap_or(0);
    let seven_days_ms = 7 * 24 * 60 * 60 * 1000;
    expires - now < seven_days_ms
}

/// Check if token is expired
pub fn token_is_expired(token: &InstagramToken) -> bool {
    let now = js_sys::Date::now() as i64;
    let expires = parse_iso_timestamp(&token.expires_at).unwrap_or(0);
    expires < now
}

// ============================================================================
// Helper functions
// ============================================================================

fn calculate_expiry(expires_in_seconds: i64) -> String {
    let now_ms = js_sys::Date::now() as i64;
    let expiry_ms = now_ms + (expires_in_seconds * 1000);
    let date = js_sys::Date::new(&JsValue::from_f64(expiry_ms as f64));
    date.to_iso_string()
        .as_string()
        .unwrap_or_else(|| String::from("1970-01-01T00:00:00.000Z"))
}

fn parse_iso_timestamp(iso: &str) -> Option<i64> {
    let date = js_sys::Date::new(&JsValue::from_str(iso));
    let time = date.get_time();
    if time.is_nan() {
        None
    } else {
        Some(time as i64)
    }
}

fn urlencoding_encode(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);
    for c in s.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            _ => {
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}
