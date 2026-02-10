//! Webhook handlers for WhatsApp and other integrations

use worker::*;

use crate::storage::*;
use crate::types::*;
use crate::whatsapp;

/// Handle webhook routes (/webhook/*)
pub async fn handle_webhook(
    req: Request,
    env: Env,
    path: &str,
    method: Method,
) -> Result<Response> {
    if path == "/webhook/whatsapp" || path == "/webhook/whatsapp/" {
        return handle_whatsapp_webhook(req, env, method).await;
    }

    if path.starts_with("/webhook/whatsapp/") {
        let slug = path.strip_prefix("/webhook/whatsapp/").unwrap_or("");
        if !slug.is_empty() && !slug.contains('/') {
            return handle_whatsapp_webhook_for_form(req, env, method, slug).await;
        }
    }

    Response::error("Unknown webhook endpoint", 404)
}

async fn handle_whatsapp_webhook(req: Request, env: Env, method: Method) -> Result<Response> {
    match method {
        Method::Get => whatsapp::verify_webhook(&req, &env),
        Method::Post => {
            let kv = env.kv("FORMS_KV")?;
            let forms = list_forms(&kv).await?;

            if forms.is_empty() {
                console_log!("No forms configured for WhatsApp webhook");
                return Response::ok("OK");
            }

            let form = &forms[0];

            let mut req = req;
            let payload: WhatsAppWebhook = match req.json().await {
                Ok(p) => p,
                Err(e) => {
                    console_log!("Failed to parse WhatsApp webhook: {:?}", e);
                    return Response::ok("OK");
                }
            };

            let messages = whatsapp::parse_webhook(&payload);
            for message in messages {
                if let Err(e) = whatsapp::process_whatsapp_message(&env, form, &message).await {
                    console_log!("Error processing WhatsApp message: {:?}", e);
                }
            }

            Response::ok("OK")
        }
        _ => Response::error("Method not allowed", 405),
    }
}

async fn handle_whatsapp_webhook_for_form(
    req: Request,
    env: Env,
    method: Method,
    slug: &str,
) -> Result<Response> {
    match method {
        Method::Get => whatsapp::verify_webhook(&req, &env),
        Method::Post => {
            let kv = env.kv("FORMS_KV")?;
            let form = match get_form(&kv, slug).await? {
                Some(f) => f,
                None => {
                    console_log!("Form not found for WhatsApp webhook: {}", slug);
                    return Response::ok("OK");
                }
            };

            let mut req = req;
            let payload: WhatsAppWebhook = match req.json().await {
                Ok(p) => p,
                Err(e) => {
                    console_log!("Failed to parse WhatsApp webhook: {:?}", e);
                    return Response::ok("OK");
                }
            };

            let messages = whatsapp::parse_webhook(&payload);
            for message in messages {
                if let Err(e) = whatsapp::process_whatsapp_message(&env, &form, &message).await {
                    console_log!("Error processing WhatsApp message: {:?}", e);
                }
            }

            Response::ok("OK")
        }
        _ => Response::error("Method not allowed", 405),
    }
}
