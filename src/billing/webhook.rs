//! Razorpay webhook handler — processes payment events.

use worker::*;

use super::razorpay;

/// Handle POST /webhook/razorpay
pub async fn handle_razorpay_webhook(mut req: Request, env: Env) -> Result<Response> {
    let signature = req
        .headers()
        .get("X-Razorpay-Signature")?
        .unwrap_or_default();
    let body = req.text().await?;

    let webhook_secret = env
        .secret("RAZORPAY_WEBHOOK_SECRET")
        .map(|s| s.to_string())
        .unwrap_or_default();

    if !webhook_secret.is_empty()
        && !razorpay::verify_webhook_signature(&body, &signature, &webhook_secret)
    {
        console_log!("Invalid Razorpay webhook signature");
        return Response::error("Invalid signature", 401);
    }

    let payload: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| Error::from(format!("JSON parse error: {e}")))?;

    let event = payload
        .get("event")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    match event {
        "payment.captured" => {
            let payment = match payload.pointer("/payload/payment/entity") {
                Some(p) => p,
                None => return Response::ok("OK"),
            };

            let payment_id = payment.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let tenant_id = payment
                .pointer("/notes/tenant_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let credits = payment
                .pointer("/notes/credits")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0);

            if !tenant_id.is_empty() && credits > 0 {
                let kv = env.kv("CALENDARS_KV")?;
                super::grant_replies(&kv, tenant_id, credits).await?;
                console_log!("Webhook: granted {credits} replies to {tenant_id} (payment {payment_id})");
            }

            Response::ok("OK")
        }
        "payment.failed" => {
            console_log!("Razorpay payment failed: {:?}", payload.get("payload"));
            Response::ok("OK")
        }
        _ => {
            console_log!("Unhandled Razorpay event: {event}");
            Response::ok("OK")
        }
    }
}
