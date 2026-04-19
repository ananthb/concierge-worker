//! AI-powered email reply generation using Workers AI

use serde::Serialize;
use worker::*;

fn get_model(env: &Env) -> String {
    env.var("AI_MODEL")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "@cf/meta/llama-4-scout-17b-16e-instruct".to_string())
}

#[derive(Serialize)]
struct AiRequest {
    messages: Vec<AiMessage>,
}

#[derive(Serialize)]
struct AiMessage {
    role: String,
    content: String,
}

const DEFAULT_SYSTEM_PROMPT: &str = "You are an email assistant. Draft a polite, professional reply to the email below. Keep it concise. Do not include subject line or headers, just the reply body.";

/// Generate an AI reply draft for an incoming email.
/// Returns Err if prompt injection is detected in the email body.
pub async fn generate_email_reply(
    env: &Env,
    system_prompt: Option<&str>,
    from: &str,
    subject: &str,
    body: &str,
) -> Result<String> {
    // Scan email body for prompt injection before generating reply
    let scan_text = format!("{subject}\n\n{body}");
    if crate::ai::is_prompt_injection(env, &scan_text).await {
        return Err(Error::from(
            "Prompt injection detected in email, skipping AI reply",
        ));
    }

    let prompt = system_prompt.unwrap_or(DEFAULT_SYSTEM_PROMPT);

    let user_message = format!("From: {from}\nSubject: {subject}\n\n{body}");

    let request = AiRequest {
        messages: vec![
            AiMessage {
                role: "system".into(),
                content: prompt.to_string(),
            },
            AiMessage {
                role: "user".into(),
                content: user_message,
            },
        ],
    };

    let request_json = serde_json::to_string(&request)
        .map_err(|e| Error::from(format!("Failed to serialize AI request: {e}")))?;

    let ai = env.ai("AI")?;

    let input: serde_json::Value = serde_json::from_str(&request_json)
        .map_err(|e| Error::from(format!("Failed to parse request: {e}")))?;

    let model = get_model(env);
    let response: serde_json::Value = ai
        .run(&model, input)
        .await
        .map_err(|e| Error::from(format!("AI model error: {e:?}")))?;

    let reply = response
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| {
            response
                .get("response")
                .and_then(|r| r.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "Thank you for your email. I will get back to you shortly.".into());

    Ok(reply)
}
