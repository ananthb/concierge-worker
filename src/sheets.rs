use base64::Engine;
use serde::{Deserialize, Serialize};
use worker::*;

use crate::types::FormField;

/// Google Service Account credentials
#[derive(Debug, Deserialize)]
struct ServiceAccountCredentials {
    client_email: String,
    private_key: String,
    token_uri: String,
}

/// JWT Header
#[derive(Serialize)]
struct JwtHeader {
    alg: String,
    typ: String,
}

/// JWT Claims for Google OAuth
#[derive(Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    exp: u64,
    iat: u64,
}

/// OAuth token response
#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

/// Append a row to a Google Sheet
pub async fn append_to_sheet(
    env: &Env,
    sheet_url: &str,
    fields: &[FormField],
    fields_data: &serde_json::Map<String, serde_json::Value>,
) -> Result<()> {
    // Parse sheet ID from URL
    let sheet_id = parse_sheet_id(sheet_url).ok_or_else(|| {
        Error::from("Invalid Google Sheet URL")
    })?;

    // Get service account credentials
    let creds_json = env.secret("GOOGLE_SERVICE_ACCOUNT_JSON")?.to_string();
    let creds: ServiceAccountCredentials = serde_json::from_str(&creds_json)
        .map_err(|e| Error::from(format!("Invalid service account JSON: {}", e)))?;

    // Get OAuth access token
    let access_token = get_access_token(&creds).await?;

    // Build row data from fields
    let row: Vec<String> = fields
        .iter()
        .map(|field| {
            fields_data
                .get(&field.id)
                .map(|v| match v {
                    serde_json::Value::String(s) => s.clone(),
                    _ => v.to_string(),
                })
                .unwrap_or_default()
        })
        .collect();

    // Add timestamp as last column
    let timestamp = js_sys::Date::new_0()
        .to_iso_string()
        .as_string()
        .unwrap_or_default();
    let mut row_with_timestamp = row;
    row_with_timestamp.push(timestamp);

    // Append to sheet
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/Sheet1:append?valueInputOption=USER_ENTERED&insertDataOption=INSERT_ROWS",
        sheet_id
    );

    let payload = serde_json::json!({
        "values": [row_with_timestamp]
    });

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", access_token))?;
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
        console_log!("Google Sheets API error: status {}", response.status_code());
        return Err(Error::from(format!(
            "Google Sheets API error: {}",
            response.status_code()
        )));
    }

    console_log!("Successfully appended row to Google Sheet");
    Ok(())
}

/// Parse Google Sheet ID from various URL formats
fn parse_sheet_id(url: &str) -> Option<String> {
    // Format: https://docs.google.com/spreadsheets/d/{SHEET_ID}/...
    if url.contains("docs.google.com/spreadsheets/d/") {
        let parts: Vec<&str> = url.split("/d/").collect();
        if parts.len() > 1 {
            let id_part = parts[1];
            let id = id_part.split('/').next()?;
            return Some(id.to_string());
        }
    }

    // If it looks like just an ID (alphanumeric with dashes/underscores)
    if url
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Some(url.to_string());
    }

    None
}

/// Get OAuth2 access token using service account credentials
async fn get_access_token(creds: &ServiceAccountCredentials) -> Result<String> {
    let now = (js_sys::Date::now() / 1000.0) as u64;

    let header = JwtHeader {
        alg: "RS256".to_string(),
        typ: "JWT".to_string(),
    };

    let claims = JwtClaims {
        iss: creds.client_email.clone(),
        scope: "https://www.googleapis.com/auth/spreadsheets".to_string(),
        aud: creds.token_uri.clone(),
        exp: now + 3600,
        iat: now,
    };

    // Encode header and claims
    let header_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(serde_json::to_string(&header).unwrap());
    let claims_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(serde_json::to_string(&claims).unwrap());

    let signing_input = format!("{}.{}", header_b64, claims_b64);

    // Sign with RSA private key
    let signature = sign_rs256(&signing_input, &creds.private_key).await?;
    let signature_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&signature);

    let jwt = format!("{}.{}", signing_input, signature_b64);

    // Exchange JWT for access token
    let form_body = format!(
        "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={}",
        jwt
    );

    let headers = Headers::new();
    headers.set("Content-Type", "application/x-www-form-urlencoded")?;

    let request = Request::new_with_init(
        &creds.token_uri,
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(wasm_bindgen::JsValue::from_str(&form_body))),
    )?;

    let mut response = Fetch::Request(request).send().await?;

    if !response.status_code().to_string().starts_with('2') {
        let error_text = response.text().await.unwrap_or_default();
        console_log!("OAuth token error: {} - {}", response.status_code(), error_text);
        return Err(Error::from(format!(
            "Failed to get OAuth token: {}",
            response.status_code()
        )));
    }

    let token_response: TokenResponse = response.json().await?;
    Ok(token_response.access_token)
}

/// Sign data with RS256 (RSA-SHA256) using Web Crypto API
async fn sign_rs256(data: &str, private_key_pem: &str) -> Result<Vec<u8>> {
    use js_sys::{ArrayBuffer, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    // Get the crypto subtle interface via globalThis (works in Workers)
    let global = js_sys::global();
    let crypto = Reflect::get(&global, &"crypto".into())
        .map_err(|_| Error::from("No crypto available"))?;
    let subtle = Reflect::get(&crypto, &"subtle".into())
        .map_err(|_| Error::from("No subtle crypto available"))?;

    // Parse PEM to DER
    let der = pem_to_der(private_key_pem)?;

    // Import the private key
    let algorithm = Object::new();
    Reflect::set(&algorithm, &"name".into(), &"RSASSA-PKCS1-v1_5".into())
        .map_err(|_| Error::from("Failed to set algorithm name"))?;
    Reflect::set(&algorithm, &"hash".into(), &"SHA-256".into())
        .map_err(|_| Error::from("Failed to set hash"))?;

    let key_data = Uint8Array::from(der.as_slice());
    let key_usages = js_sys::Array::new();
    key_usages.push(&"sign".into());

    // Call subtle.importKey
    let import_key_fn = Reflect::get(&subtle, &"importKey".into())
        .map_err(|_| Error::from("No importKey function"))?;
    let import_key_fn: js_sys::Function = import_key_fn.dyn_into()
        .map_err(|_| Error::from("importKey is not a function"))?;

    let import_args = js_sys::Array::new();
    import_args.push(&"pkcs8".into());
    import_args.push(&key_data.buffer());
    import_args.push(&algorithm);
    import_args.push(&false.into());
    import_args.push(&key_usages);

    let import_promise = import_key_fn.apply(&subtle, &import_args)
        .map_err(|_| Error::from("Failed to call importKey"))?;
    let import_promise: js_sys::Promise = import_promise.dyn_into()
        .map_err(|_| Error::from("importKey did not return a promise"))?;

    let key = JsFuture::from(import_promise)
        .await
        .map_err(|e| Error::from(format!("Key import failed: {:?}", e)))?;

    // Sign the data
    let data_bytes = data.as_bytes();
    let data_array = Uint8Array::from(data_bytes);

    // Call subtle.sign
    let sign_fn = Reflect::get(&subtle, &"sign".into())
        .map_err(|_| Error::from("No sign function"))?;
    let sign_fn: js_sys::Function = sign_fn.dyn_into()
        .map_err(|_| Error::from("sign is not a function"))?;

    let sign_args = js_sys::Array::new();
    sign_args.push(&algorithm);
    sign_args.push(&key);
    sign_args.push(&data_array);

    let sign_promise = sign_fn.apply(&subtle, &sign_args)
        .map_err(|_| Error::from("Failed to call sign"))?;
    let sign_promise: js_sys::Promise = sign_promise.dyn_into()
        .map_err(|_| Error::from("sign did not return a promise"))?;

    let signature_buffer: ArrayBuffer = JsFuture::from(sign_promise)
        .await
        .map_err(|e| Error::from(format!("Signing failed: {:?}", e)))?
        .dyn_into()
        .map_err(|_| Error::from("Invalid signature type"))?;

    let signature_array = Uint8Array::new(&signature_buffer);
    Ok(signature_array.to_vec())
}

/// Convert PEM-encoded private key to DER format
fn pem_to_der(pem: &str) -> Result<Vec<u8>> {
    // Remove PEM headers and whitespace
    let pem_clean = pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<Vec<_>>()
        .join("");

    base64::engine::general_purpose::STANDARD
        .decode(&pem_clean)
        .map_err(|e| Error::from(format!("Failed to decode PEM: {}", e)))
}
