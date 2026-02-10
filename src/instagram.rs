use wasm_bindgen::JsValue;
use worker::*;

use crate::types::{InstagramMedia, InstagramToken};

const INSTAGRAM_API_URL: &str = "https://graph.instagram.com";
const INSTAGRAM_OAUTH_URL: &str = "https://api.instagram.com/oauth";

/// Instagram API client
pub struct InstagramClient {
    access_token: String,
}

impl InstagramClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }

    /// Fetch recent media (up to 25 posts) from the user's Instagram account
    pub async fn get_recent_media(&self) -> Result<Vec<InstagramMedia>> {
        let url = format!(
            "{}/me/media?fields=id,caption,media_type,permalink,timestamp&limit=25&access_token={}",
            INSTAGRAM_API_URL, self.access_token
        );

        let mut init = RequestInit::new();
        init.with_method(Method::Get);

        let request = Request::new_with_init(&url, &init)?;
        let mut response = Fetch::Request(request).send().await?;

        if response.status_code() != 200 {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::from(format!(
                "Instagram API error {}: {}",
                response.status_code(),
                error_text
            )));
        }

        let body: serde_json::Value = response.json().await?;

        let media = body
            .get("data")
            .and_then(|d| d.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| serde_json::from_value(item.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();

        Ok(media)
    }

    /// Get user profile info
    pub async fn get_user_info(&self) -> Result<(String, String)> {
        let url = format!(
            "{}/me?fields=id,username&access_token={}",
            INSTAGRAM_API_URL, self.access_token
        );

        let mut init = RequestInit::new();
        init.with_method(Method::Get);

        let request = Request::new_with_init(&url, &init)?;
        let mut response = Fetch::Request(request).send().await?;

        if response.status_code() != 200 {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::from(format!(
                "Instagram API error {}: {}",
                response.status_code(),
                error_text
            )));
        }

        let body: serde_json::Value = response.json().await?;

        let user_id = body
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::from("Missing user id"))?
            .to_string();

        let username = body
            .get("username")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::from("Missing username"))?
            .to_string();

        Ok((user_id, username))
    }
}

/// Exchange short-lived token for long-lived token (60 days)
pub async fn exchange_for_long_lived_token(
    short_lived_token: &str,
    app_secret: &str,
) -> Result<InstagramToken> {
    let url = format!(
        "{}/access_token?grant_type=ig_exchange_token&client_secret={}&access_token={}",
        INSTAGRAM_API_URL, app_secret, short_lived_token
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

    let access_token = body
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::from("Missing access_token"))?
        .to_string();

    let expires_in = body
        .get("expires_in")
        .and_then(|v| v.as_i64())
        .unwrap_or(5184000); // Default to 60 days

    // Get user ID from the token
    let client = InstagramClient::new(access_token.clone());
    let (user_id, _) = client.get_user_info().await?;

    // Calculate expiration timestamp
    let expires_at = calculate_expiry(expires_in);

    Ok(InstagramToken {
        access_token,
        expires_at,
        user_id,
    })
}

/// Refresh a long-lived token before it expires
pub async fn refresh_token(current_token: &str) -> Result<InstagramToken> {
    let url = format!(
        "{}/refresh_access_token?grant_type=ig_refresh_token&access_token={}",
        INSTAGRAM_API_URL, current_token
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

    // Get user ID
    let client = InstagramClient::new(access_token.clone());
    let (user_id, _) = client.get_user_info().await?;

    let expires_at = calculate_expiry(expires_in);

    Ok(InstagramToken {
        access_token,
        expires_at,
        user_id,
    })
}

/// Generate OAuth authorization URL
pub fn get_auth_url(app_id: &str, redirect_uri: &str, state: &str) -> String {
    format!(
        "{}/authorize?client_id={}&redirect_uri={}&scope=user_profile,user_media&response_type=code&state={}",
        INSTAGRAM_OAUTH_URL, app_id, urlencoding_encode(redirect_uri), state
    )
}

/// Exchange authorization code for short-lived access token
pub async fn exchange_code_for_token(
    code: &str,
    app_id: &str,
    app_secret: &str,
    redirect_uri: &str,
) -> Result<String> {
    let url = format!("{}/access_token", INSTAGRAM_OAUTH_URL);

    let body = format!(
        "client_id={}&client_secret={}&grant_type=authorization_code&redirect_uri={}&code={}",
        app_id,
        app_secret,
        urlencoding_encode(redirect_uri),
        code
    );

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_body(Some(JsValue::from_str(&body)));

    let mut headers = Headers::new();
    headers.set("Content-Type", "application/x-www-form-urlencoded")?;
    init.with_headers(headers);

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        let error_text = response.text().await.unwrap_or_default();
        return Err(Error::from(format!(
            "Code exchange error {}: {}",
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

    Ok(access_token)
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
