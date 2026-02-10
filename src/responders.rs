use base64::Engine;
use worker::*;

pub async fn send_twilio_message(env: &Env, to: &str, from: &str, body: &str) -> Result<()> {
    let sid = env.secret("TWILIO_SID")?.to_string();
    let token = env.secret("TWILIO_TOKEN")?.to_string();

    let url = format!(
        "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
        sid
    );
    let auth = base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", sid, token));

    let form_body = format!(
        "To={}&From={}&Body={}",
        urlencoding::encode(to),
        urlencoding::encode(from),
        urlencoding::encode(body)
    );

    let headers = Headers::new();
    headers.set("Authorization", &format!("Basic {}", auth))?;
    headers.set("Content-Type", "application/x-www-form-urlencoded")?;

    let request = Request::new_with_init(
        &url,
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(wasm_bindgen::JsValue::from_str(&form_body))),
    )?;

    Fetch::Request(request).send().await?;
    Ok(())
}

pub async fn send_twilio_email(
    env: &Env,
    to: &str,
    from: &str,
    subject: &str,
    body: &str,
) -> Result<()> {
    let api_key = env.secret("SENDGRID_API_KEY")?.to_string();

    let payload = serde_json::json!({
        "personalizations": [{"to": [{"email": to}]}],
        "from": {"email": from},
        "subject": subject,
        "content": [{"type": "text/plain", "value": body}]
    });

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", api_key))?;
    headers.set("Content-Type", "application/json")?;

    let request = Request::new_with_init(
        "https://api.sendgrid.com/v3/mail/send",
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(wasm_bindgen::JsValue::from_str(&payload.to_string()))),
    )?;

    Fetch::Request(request).send().await?;
    Ok(())
}

pub async fn send_resend_email(
    env: &Env,
    to: &str,
    from: &str,
    subject: &str,
    body: &str,
) -> Result<()> {
    let api_key = env.secret("RESEND_API_KEY")?.to_string();

    let payload = serde_json::json!({
        "from": from,
        "to": to,
        "subject": subject,
        "text": body
    });

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", api_key))?;
    headers.set("Content-Type", "application/json")?;

    let request = Request::new_with_init(
        "https://api.resend.com/emails",
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(wasm_bindgen::JsValue::from_str(&payload.to_string()))),
    )?;

    Fetch::Request(request).send().await?;
    Ok(())
}
