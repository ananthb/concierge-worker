use worker::*;

use crate::ai;
use crate::helpers::interpolate_template;
use crate::responders::{send_resend_email, send_twilio_email, send_twilio_message};
use crate::storage::save_submission;
use crate::types::{FormConfig, IncomingMessage, ResponderChannel, WhatsAppWebhook};

/// Parse incoming WhatsApp webhook and extract messages
pub fn parse_webhook(payload: &WhatsAppWebhook) -> Vec<IncomingMessage> {
    let mut messages = Vec::new();

    for entry in &payload.entry {
        for change in &entry.changes {
            if change.field != "messages" {
                continue;
            }

            let value = &change.value;
            let contacts: std::collections::HashMap<String, String> = value
                .contacts
                .iter()
                .map(|c| (c.wa_id.clone(), c.profile.name.clone()))
                .collect();

            for msg in &value.messages {
                if msg.message_type == "text" {
                    if let Some(text) = &msg.text {
                        let sender_name = contacts
                            .get(&msg.from)
                            .cloned()
                            .unwrap_or_else(|| msg.from.clone());

                        messages.push(IncomingMessage {
                            from: msg.from.clone(),
                            sender_name,
                            text: text.body.clone(),
                            message_id: msg.id.clone(),
                            timestamp: msg.timestamp.clone(),
                        });
                    }
                }
            }
        }
    }

    messages
}

/// Send a WhatsApp message via Meta Graph API
pub async fn send_whatsapp_message(env: &Env, to: &str, text: &str) -> Result<()> {
    let access_token = env.secret("WHATSAPP_ACCESS_TOKEN")?.to_string();
    let phone_number_id = env.secret("WHATSAPP_PHONE_NUMBER_ID")?.to_string();

    let url = format!(
        "https://graph.facebook.com/v18.0/{}/messages",
        phone_number_id
    );

    let payload = serde_json::json!({
        "messaging_product": "whatsapp",
        "to": to,
        "type": "text",
        "text": {
            "body": text
        }
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
        console_log!("WhatsApp API error: status {}", response.status_code());
    }

    Ok(())
}

/// Process incoming WhatsApp message: store as submission and trigger responders
pub async fn process_whatsapp_message(
    env: &Env,
    form: &FormConfig,
    message: &IncomingMessage,
) -> Result<()> {
    let db = env.d1("DB")?;

    // Build fields_data from the message
    let mut fields_data: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

    // Try to map to existing form fields
    for field in &form.fields {
        let value = match field.id.as_str() {
            "name" => Some(message.sender_name.clone()),
            "message" | "text" | "body" | "content" => Some(message.text.clone()),
            "phone" | "mobile" | "whatsapp" => Some(message.from.clone()),
            _ => None,
        };

        if let Some(v) = value {
            fields_data.insert(field.id.clone(), serde_json::Value::String(v));
        }
    }

    // Always store the raw message data
    fields_data.insert(
        "_whatsapp_from".to_string(),
        serde_json::Value::String(message.from.clone()),
    );
    fields_data.insert(
        "_whatsapp_name".to_string(),
        serde_json::Value::String(message.sender_name.clone()),
    );
    fields_data.insert(
        "_whatsapp_message".to_string(),
        serde_json::Value::String(message.text.clone()),
    );
    fields_data.insert(
        "_whatsapp_message_id".to_string(),
        serde_json::Value::String(message.message_id.clone()),
    );

    let fields_json = serde_json::to_string(&fields_data).unwrap_or_else(|_| "{}".into());

    // Store in database
    save_submission(&db, &form.slug, &fields_json, None).await?;

    // Trigger responders
    for responder in &form.responders {
        if !responder.enabled {
            continue;
        }

        // For MetaWhatsapp responders, respond back to the sender
        if matches!(responder.channel, ResponderChannel::MetaWhatsapp) {
            let body = if responder.use_ai {
                match ai::generate_response(env, &responder.body, &fields_data).await {
                    Ok(response) => response,
                    Err(e) => {
                        console_log!("AI generation error: {:?}", e);
                        interpolate_template(&responder.body, &fields_data)
                    }
                }
            } else {
                interpolate_template(&responder.body, &fields_data)
            };

            if let Err(e) = send_whatsapp_message(env, &message.from, &body).await {
                console_log!("WhatsApp responder error: {:?}", e);
            }
            continue;
        }

        // Handle other responder types
        let Some(serde_json::Value::String(target)) = fields_data.get(&responder.target_field)
        else {
            continue;
        };

        let body = if responder.use_ai {
            match ai::generate_response(env, &responder.body, &fields_data).await {
                Ok(response) => response,
                Err(e) => {
                    console_log!("AI generation error: {:?}", e);
                    interpolate_template(&responder.body, &fields_data)
                }
            }
        } else {
            interpolate_template(&responder.body, &fields_data)
        };

        let result = match responder.channel {
            ResponderChannel::TwilioSms => {
                if let Ok(from) = env.secret("TWILIO_FROM_SMS") {
                    send_twilio_message(env, target, &from.to_string(), &body).await
                } else {
                    continue;
                }
            }
            ResponderChannel::TwilioRcs => {
                if let Ok(from) = env.secret("TWILIO_FROM_SMS") {
                    send_twilio_message(env, target, &from.to_string(), &body).await
                } else {
                    continue;
                }
            }
            ResponderChannel::TwilioWhatsapp => {
                if let Ok(from) = env.secret("TWILIO_FROM_WHATSAPP") {
                    send_twilio_message(
                        env,
                        &format!("whatsapp:{}", target),
                        &from.to_string(),
                        &body,
                    )
                    .await
                } else {
                    continue;
                }
            }
            ResponderChannel::TwilioEmail => {
                if let Ok(from) = env.secret("TWILIO_FROM_EMAIL") {
                    let subject = interpolate_template(&responder.subject, &fields_data);
                    send_twilio_email(env, target, &from.to_string(), &subject, &body).await
                } else {
                    continue;
                }
            }
            ResponderChannel::ResendEmail => {
                if let Ok(from) = env.secret("RESEND_FROM") {
                    let subject = interpolate_template(&responder.subject, &fields_data);
                    send_resend_email(env, target, &from.to_string(), &subject, &body).await
                } else {
                    continue;
                }
            }
            ResponderChannel::MetaWhatsapp => {
                // Already handled above
                continue;
            }
        };

        if let Err(e) = result {
            console_log!("Responder error for {}: {:?}", responder.name, e);
        }
    }

    Ok(())
}

/// Handle WhatsApp webhook verification (GET request)
pub fn verify_webhook(req: &Request, env: &Env) -> Result<Response> {
    let url = req.url()?;
    let query: std::collections::HashMap<_, _> = url.query_pairs().collect();

    let mode = query.get("hub.mode").map(|s| s.as_ref());
    let token = query.get("hub.verify_token").map(|s| s.as_ref());
    let challenge = query.get("hub.challenge").map(|s| s.as_ref());

    let verify_token = env
        .secret("WHATSAPP_VERIFY_TOKEN")
        .map(|s| s.to_string())
        .unwrap_or_default();

    if mode == Some("subscribe") && token == Some(&verify_token) {
        if let Some(challenge) = challenge {
            console_log!("WhatsApp webhook verified successfully");
            return Response::ok(challenge);
        }
    }

    console_log!("WhatsApp webhook verification failed");
    Response::error("Verification failed", 403)
}
