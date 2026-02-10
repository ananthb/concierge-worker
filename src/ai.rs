use serde::{Deserialize, Serialize};
use worker::*;

use crate::types::ExtractedEvent;

const MODEL: &str = "@cf/meta/llama-3.1-8b-instruct";
const MIN_CONFIDENCE: f32 = 0.6;

#[derive(Serialize)]
struct AiRequest {
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct AiResponse {
    response: Option<String>,
}

// ============================================================================
// Responder AI Generation
// ============================================================================

/// Generate an AI response for form/booking responders using Cloudflare Workers AI
///
/// The system_prompt is treated as instructions for the AI, and the form data
/// is provided as context for generating the response.
pub async fn generate_response(
    env: &Env,
    system_prompt: &str,
    fields_data: &serde_json::Map<String, serde_json::Value>,
) -> Result<String> {
    // Build context from form data
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
        "Form submission data:\n{}\n\nGenerate an appropriate response based on this submission.",
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

    let response = run_ai_model(env, &request).await?;
    Ok(response)
}

// ============================================================================
// Instagram Event Extraction
// ============================================================================

/// Extract event details from an Instagram caption using Cloudflare AI
pub async fn extract_event_from_caption(
    env: &Env,
    caption: &str,
    timezone: &str,
    today_date: &str,
) -> Result<(ExtractedEvent, String)> {
    let system_prompt = format!(
        r#"You are an event extraction assistant. Analyze Instagram captions to identify event announcements.

Today's date is {today_date}. The calendar timezone is {timezone}.

Extract event details and respond with ONLY a JSON object (no markdown, no explanation):
{{
  "has_event": boolean,
  "is_cancellation": boolean,
  "title": string or null,
  "date": "YYYY-MM-DD" or null,
  "start_time": "HH:MM" (24-hour) or null,
  "end_time": "HH:MM" (24-hour) or null,
  "description": string or null,
  "confidence": number between 0 and 1
}}

Guidelines:
- Set has_event=true only if the post clearly announces a specific event with at least a date
- For relative dates like "this Saturday" or "next Friday", calculate the actual date from today
- If only a date is given without time, leave start_time and end_time as null
- If cancellation words appear ("cancelled", "postponed", "called off"), set is_cancellation=true
- Set confidence based on how clearly the post describes an event (0.0 to 1.0)
- For event descriptions, include relevant details but keep it concise
- If no event is found, set has_event=false and confidence=0"#,
        today_date = today_date,
        timezone = timezone
    );

    let user_prompt = format!(
        "Extract event details from this Instagram caption:\n\n{}",
        caption
    );

    let request = AiRequest {
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: user_prompt,
            },
        ],
    };

    let ai_response = run_ai_model(env, &request).await?;
    let event = parse_ai_response(&ai_response)?;

    Ok((event, ai_response))
}

// ============================================================================
// Contact/Lead Extraction from Instagram
// ============================================================================

/// Extract contact/lead details from an Instagram caption using Cloudflare AI
pub async fn extract_contact_from_caption(
    env: &Env,
    caption: &str,
    _form_fields: &[crate::types::FormField],
) -> Result<(serde_json::Map<String, serde_json::Value>, String)> {
    let system_prompt = r#"You are a contact extraction assistant. Analyze Instagram captions to identify contact information or lead details.

Extract contact details and respond with ONLY a JSON object (no markdown, no explanation):
{
  "has_contact": boolean,
  "name": string or null,
  "email": string or null,
  "phone": string or null,
  "message": string or null,
  "confidence": number between 0 and 1
}

Guidelines:
- Set has_contact=true if the post contains contact information or appears to be a lead/inquiry
- Extract any names, email addresses, or phone numbers mentioned
- For message, summarize the main content or inquiry
- Set confidence based on how clearly the post contains contact information (0.0 to 1.0)
- If no contact info is found, set has_contact=false and confidence=0"#;

    let user_prompt = format!(
        "Extract contact details from this Instagram caption:\n\n{}",
        caption
    );

    let request = AiRequest {
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_prompt,
            },
        ],
    };

    let ai_response = run_ai_model(env, &request).await?;

    // Parse the response into fields_data
    let json_str = extract_json_from_response(&ai_response);
    let parsed: serde_json::Value = serde_json::from_str(&json_str).map_err(|e| {
        Error::from(format!(
            "Failed to parse AI JSON: {} - Response: {}",
            e, ai_response
        ))
    })?;

    let has_contact = parsed
        .get("has_contact")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let confidence = parsed
        .get("confidence")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let mut fields_data = serde_json::Map::new();

    if has_contact && confidence >= 0.5 {
        if let Some(name) = parsed.get("name").and_then(|v| v.as_str()) {
            fields_data.insert(
                "name".to_string(),
                serde_json::Value::String(name.to_string()),
            );
        }
        if let Some(email) = parsed.get("email").and_then(|v| v.as_str()) {
            fields_data.insert(
                "email".to_string(),
                serde_json::Value::String(email.to_string()),
            );
        }
        if let Some(phone) = parsed.get("phone").and_then(|v| v.as_str()) {
            fields_data.insert(
                "phone".to_string(),
                serde_json::Value::String(phone.to_string()),
            );
        }
        if let Some(message) = parsed.get("message").and_then(|v| v.as_str()) {
            fields_data.insert(
                "message".to_string(),
                serde_json::Value::String(message.to_string()),
            );
        }
    }

    Ok((fields_data, ai_response))
}

// ============================================================================
// Helper Functions
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
        .unwrap_or_else(|| "Thank you for your submission.".to_string());

    Ok(response_str)
}

fn parse_ai_response(response: &str) -> Result<ExtractedEvent> {
    let json_str = extract_json_from_response(response);

    let parsed: serde_json::Value = serde_json::from_str(&json_str).map_err(|e| {
        Error::from(format!(
            "Failed to parse AI JSON: {} - Response: {}",
            e, response
        ))
    })?;

    let has_event = parsed
        .get("has_event")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !has_event {
        return Ok(ExtractedEvent::default());
    }

    let confidence = parsed
        .get("confidence")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32)
        .unwrap_or(0.0);

    if confidence < MIN_CONFIDENCE {
        return Ok(ExtractedEvent {
            confidence,
            ..Default::default()
        });
    }

    let is_cancellation = parsed
        .get("is_cancellation")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let title = parsed
        .get("title")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let date = parsed
        .get("date")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let start_time = parsed
        .get("start_time")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let end_time = parsed
        .get("end_time")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let description = parsed
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(ExtractedEvent {
        title,
        date,
        start_time,
        end_time,
        description,
        is_cancellation,
        confidence,
    })
}

fn extract_json_from_response(response: &str) -> String {
    let trimmed = response.trim();

    // If it starts with {, assume it's JSON
    if trimmed.starts_with('{') {
        let mut depth = 0;
        let mut end = 0;
        for (i, c) in trimmed.char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        if end > 0 {
            return trimmed[..end].to_string();
        }
    }

    // Try to find JSON block in markdown code fence
    if let Some(start) = trimmed.find("```json") {
        if let Some(end) = trimmed[start + 7..].find("```") {
            return trimmed[start + 7..start + 7 + end].trim().to_string();
        }
    }

    // Try to find any JSON object
    if let Some(start) = trimmed.find('{') {
        if let Some(end) = trimmed.rfind('}') {
            return trimmed[start..=end].to_string();
        }
    }

    trimmed.to_string()
}

/// Check if an extracted event meets the minimum quality threshold
pub fn event_is_valid(event: &ExtractedEvent) -> bool {
    event.confidence >= MIN_CONFIDENCE && event.title.is_some() && event.date.is_some()
}

/// Generate a signature for deduplication
pub fn generate_event_signature(event: &ExtractedEvent) -> Option<String> {
    let title = event.title.as_ref()?;
    let date = event.date.as_ref()?;

    let normalized_title = normalize_title(title);
    let start_time = event.start_time.as_deref().unwrap_or("");

    let signature_input = format!("{}|{}|{}", normalized_title, date, start_time);

    Some(crate::crypto::sha256_hex(&signature_input))
}

fn normalize_title(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
