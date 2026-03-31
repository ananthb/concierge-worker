use serde::Serialize;
use worker::*;

const MODEL: &str = "@cf/meta/llama-3.1-8b-instruct";

#[derive(Serialize)]
struct AiRequest {
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

// ============================================================================
// AI Response Generation
// ============================================================================

/// Generate an AI response using Cloudflare Workers AI
pub async fn generate_response(
    env: &Env,
    system_prompt: &str,
    fields_data: &serde_json::Map<String, serde_json::Value>,
) -> Result<String> {
    let form_context: String = fields_data
        .iter()
        .map(|(key, value)| {
            let val = match value {
                serde_json::Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            format!("{}: {}", key, val)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let user_message = format!(
        "Context:\n{}\n\nGenerate an appropriate response.",
        form_context
    );

    let request = AiRequest {
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_message,
            },
        ],
    };

    run_ai_model(env, &request).await
}

// ============================================================================
// Internal
// ============================================================================

async fn run_ai_model(env: &Env, request: &AiRequest) -> Result<String> {
    let request_json = serde_json::to_string(request)
        .map_err(|e| Error::from(format!("Failed to serialize AI request: {}", e)))?;

    let ai = env.ai("AI")?;

    let input: serde_json::Value = serde_json::from_str(&request_json)
        .map_err(|e| Error::from(format!("Failed to parse request: {}", e)))?;

    let response: serde_json::Value = ai
        .run(MODEL, input)
        .await
        .map_err(|e| Error::from(format!("AI model error: {:?}", e)))?;

    let response_str = response
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| {
            response
                .get("response")
                .and_then(|r| r.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "Thank you for your message.".to_string());

    Ok(response_str)
}
