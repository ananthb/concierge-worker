//! Google Calendar API client using service account authentication
//!
//! Requires secrets:
//! - GOOGLE_SERVICE_ACCOUNT_EMAIL: Service account email
//! - GOOGLE_PRIVATE_KEY: RSA private key in PEM format
//!
//! The Google Calendar must be shared with the service account email.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use worker::*;

use crate::types::CalendarEvent;

const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_CALENDAR_API: &str = "https://www.googleapis.com/calendar/v3";
const SCOPE: &str = "https://www.googleapis.com/auth/calendar";

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Serialize)]
struct GoogleEvent {
    summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    start: GoogleDateTime,
    end: GoogleDateTime,
}

#[derive(Serialize, Deserialize)]
struct GoogleDateTime {
    #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
    date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
}

#[derive(Deserialize)]
struct GoogleEventResponse {
    id: String,
    summary: Option<String>,
    description: Option<String>,
    start: GoogleDateTime,
    end: GoogleDateTime,
    created: Option<String>,
    updated: Option<String>,
}

#[derive(Deserialize)]
struct GoogleEventsListResponse {
    items: Option<Vec<GoogleEventResponse>>,
}

/// Get an access token using service account JWT assertion with a custom scope
pub async fn get_access_token_with_scope(
    service_account_email: &str,
    private_key: &str,
    scope: &str,
) -> Result<String> {
    let service_email = service_account_email;
    let private_key_pem = private_key;

    let now = (js_sys::Date::now() / 1000.0) as u64;
    let exp = now + 3600;

    // Build JWT header and claims
    let header = serde_json::json!({"alg": "RS256", "typ": "JWT"});
    let claims = serde_json::json!({
        "iss": service_email,
        "scope": scope,
        "aud": GOOGLE_TOKEN_URL,
        "iat": now,
        "exp": exp,
    });

    let header_b64 = base64_url_encode(&serde_json::to_vec(&header).unwrap());
    let claims_b64 = base64_url_encode(&serde_json::to_vec(&claims).unwrap());
    let signing_input = format!("{}.{}", header_b64, claims_b64);

    // Sign with RSA-SHA256
    let signature = rsa_sign(private_key_pem, signing_input.as_bytes()).await?;
    let signature_b64 = base64_url_encode(&signature);

    let jwt = format!("{}.{}", signing_input, signature_b64);

    // Exchange JWT for access token
    let body = format!(
        "grant_type={}&assertion={}",
        urlencoding::encode("urn:ietf:params:oauth:grant-type:jwt-bearer"),
        urlencoding::encode(&jwt)
    );

    let headers = Headers::new();
    headers.set("Content-Type", "application/x-www-form-urlencoded")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(JsValue::from_str(&body)));

    let request = Request::new_with_init(GOOGLE_TOKEN_URL, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    let text = response.text().await?;

    if response.status_code() != 200 {
        return Err(Error::from(format!(
            "Google token exchange failed ({}): {}",
            response.status_code(),
            text
        )));
    }

    let token_response: TokenResponse = serde_json::from_str(&text)
        .map_err(|e| Error::from(format!("Failed to parse token response: {}", e)))?;

    Ok(token_response.access_token)
}

/// List events from a Google Calendar
pub async fn list_events(
    service_account_email: &str,
    private_key: &str,
    google_calendar_id: &str,
    time_min: &str,
    time_max: &str,
    timezone: &str,
) -> Result<Vec<CalendarEvent>> {
    let token = get_access_token_with_scope(service_account_email, private_key, SCOPE).await?;
    let calendar_id_encoded = urlencoding::encode(google_calendar_id);

    let url = format!(
        "{}/calendars/{}/events?timeMin={}&timeMax={}&timeZone={}&singleEvents=true&orderBy=startTime&maxResults=250",
        GOOGLE_CALENDAR_API,
        calendar_id_encoded,
        urlencoding::encode(time_min),
        urlencoding::encode(time_max),
        urlencoding::encode(timezone),
    );

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", token))?;

    let mut init = RequestInit::new();
    init.with_method(Method::Get).with_headers(headers);

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    let text = response.text().await?;

    if response.status_code() != 200 {
        return Err(Error::from(format!(
            "Google Calendar list events failed ({}): {}",
            response.status_code(),
            text
        )));
    }

    let list_response: GoogleEventsListResponse = serde_json::from_str(&text)
        .map_err(|e| Error::from(format!("Failed to parse events response: {}", e)))?;

    let events = list_response
        .items
        .unwrap_or_default()
        .into_iter()
        .map(|ge| {
            let all_day = ge.start.date_time.is_none();
            CalendarEvent {
                id: ge.id,
                calendar_id: google_calendar_id.to_string(),
                title: ge.summary.unwrap_or_default(),
                description: ge.description,
                start_time: ge
                    .start
                    .date_time
                    .or(ge.start.date.map(|d| format!("{}T00:00:00", d)))
                    .unwrap_or_default(),
                end_time: ge
                    .end
                    .date_time
                    .or(ge.end.date.map(|d| format!("{}T23:59:59", d)))
                    .unwrap_or_default(),
                all_day,
                recurrence_rule: None,
                created_at: ge.created.unwrap_or_default(),
                updated_at: ge.updated.unwrap_or_default(),
            }
        })
        .collect();

    Ok(events)
}

/// Create an event in a Google Calendar
#[allow(clippy::too_many_arguments)]
pub async fn create_event(
    service_account_email: &str,
    private_key: &str,
    google_calendar_id: &str,
    title: &str,
    description: Option<&str>,
    start_time: &str,
    end_time: &str,
    timezone: &str,
) -> Result<String> {
    let token = get_access_token_with_scope(service_account_email, private_key, SCOPE).await?;
    let calendar_id_encoded = urlencoding::encode(google_calendar_id);

    let event = GoogleEvent {
        summary: title.to_string(),
        description: description.map(String::from),
        start: GoogleDateTime {
            date_time: Some(start_time.to_string()),
            date: None,
            time_zone: Some(timezone.to_string()),
        },
        end: GoogleDateTime {
            date_time: Some(end_time.to_string()),
            date: None,
            time_zone: Some(timezone.to_string()),
        },
    };

    let body = serde_json::to_string(&event)
        .map_err(|e| Error::from(format!("Failed to serialize event: {}", e)))?;

    let url = format!(
        "{}/calendars/{}/events",
        GOOGLE_CALENDAR_API, calendar_id_encoded
    );

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", token))?;
    headers.set("Content-Type", "application/json")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(JsValue::from_str(&body)));

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    let text = response.text().await?;

    if response.status_code() != 200 {
        return Err(Error::from(format!(
            "Google Calendar create event failed ({}): {}",
            response.status_code(),
            text
        )));
    }

    let created: GoogleEventResponse = serde_json::from_str(&text)
        .map_err(|e| Error::from(format!("Failed to parse created event: {}", e)))?;

    Ok(created.id)
}

// ============================================================================
// Crypto helpers for JWT signing
// ============================================================================

fn base64_url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// Parse PEM-encoded RSA private key and sign data with RS256
async fn rsa_sign(pem: &str, data: &[u8]) -> Result<Vec<u8>> {
    // Extract DER from PEM
    let der = pem_to_der(pem)?;

    // Import key using Web Crypto API
    let crypto = get_subtle_crypto()?;

    let algorithm = js_sys::Object::new();
    js_sys::Reflect::set(
        &algorithm,
        &JsValue::from_str("name"),
        &JsValue::from_str("RSASSA-PKCS1-v1_5"),
    )
    .map_err(|_| Error::from("Failed to set algorithm name"))?;
    js_sys::Reflect::set(
        &algorithm,
        &JsValue::from_str("hash"),
        &JsValue::from_str("SHA-256"),
    )
    .map_err(|_| Error::from("Failed to set hash"))?;

    let key_usages = js_sys::Array::new();
    key_usages.push(&JsValue::from_str("sign"));

    let key_data = js_sys::Uint8Array::from(der.as_slice());

    let promise = crypto
        .import_key_with_object("pkcs8", &key_data.buffer(), &algorithm, false, &key_usages)
        .map_err(|e| Error::from(format!("Failed to import RSA key: {:?}", e)))?;

    let crypto_key: web_sys::CryptoKey = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| Error::from(format!("RSA key import failed: {:?}", e)))?
        .dyn_into()
        .map_err(|_| Error::from("Failed to cast to CryptoKey"))?;

    // Sign
    let sign_algorithm = js_sys::Object::new();
    js_sys::Reflect::set(
        &sign_algorithm,
        &JsValue::from_str("name"),
        &JsValue::from_str("RSASSA-PKCS1-v1_5"),
    )
    .map_err(|_| Error::from("Failed to set sign algorithm name"))?;

    let data_array = js_sys::Uint8Array::from(data);

    let promise = crypto
        .sign_with_object_and_buffer_source(&sign_algorithm, &crypto_key, &data_array)
        .map_err(|e| Error::from(format!("Failed to sign: {:?}", e)))?;

    let result = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| Error::from(format!("Signing failed: {:?}", e)))?;

    let array_buffer: js_sys::ArrayBuffer = result
        .dyn_into()
        .map_err(|_| Error::from("Failed to cast to ArrayBuffer"))?;

    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    Ok(uint8_array.to_vec())
}

fn get_subtle_crypto() -> Result<web_sys::SubtleCrypto> {
    let global = js_sys::global();
    let crypto = js_sys::Reflect::get(&global, &JsValue::from_str("crypto"))
        .map_err(|_| Error::from("Failed to get crypto"))?;
    let crypto: web_sys::Crypto = crypto
        .dyn_into()
        .map_err(|_| Error::from("Failed to cast to Crypto"))?;
    Ok(crypto.subtle())
}

fn pem_to_der(pem: &str) -> Result<Vec<u8>> {
    use base64::Engine;
    let pem = pem
        .replace("-----BEGIN PRIVATE KEY-----", "")
        .replace("-----END PRIVATE KEY-----", "")
        .replace("-----BEGIN RSA PRIVATE KEY-----", "")
        .replace("-----END RSA PRIVATE KEY-----", "")
        .replace(['\n', '\r', ' '], "");

    base64::engine::general_purpose::STANDARD
        .decode(&pem)
        .map_err(|e| Error::from(format!("Failed to decode PEM: {}", e)))
}
