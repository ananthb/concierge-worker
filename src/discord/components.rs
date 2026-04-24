//! Discord message component (button) and modal submission handlers.
//! Implements the cross-channel relay: Reply button → modal → send back.

use botrelay::discord::{ActionRow, Component, Interaction, InteractionResponse};
use worker::*;

use crate::channel;
use crate::helpers::generate_id;
use crate::storage::*;
use crate::types::*;

/// Handle MESSAGE_COMPONENT interactions (button clicks).
pub async fn handle_component(interaction: &Interaction, env: &Env) -> Result<Response> {
    let custom_id = interaction
        .data
        .as_ref()
        .and_then(|d| d.custom_id.as_deref())
        .unwrap_or("");

    if let Some(ctx_id) = custom_id.strip_prefix("reply:") {
        return show_reply_modal(ctx_id);
    }
    if let Some(ctx_id) = custom_id.strip_prefix("approve:") {
        return handle_approve(ctx_id, env).await;
    }
    if let Some(ctx_id) = custom_id.strip_prefix("reject:") {
        return handle_reject(ctx_id, env).await;
    }
    if let Some(ctx_id) = custom_id.strip_prefix("drop:") {
        return handle_drop(ctx_id, env).await;
    }

    ephemeral("Unknown action")
}

/// Handle MODAL_SUBMIT interactions (reply text submitted).
pub async fn handle_modal_submit(interaction: &Interaction, env: &Env) -> Result<Response> {
    let custom_id = interaction
        .data
        .as_ref()
        .and_then(|d| d.custom_id.as_deref())
        .unwrap_or("");

    if let Some(ctx_id) = custom_id.strip_prefix("reply_modal:") {
        let reply_text = interaction.modal_text("reply_text").unwrap_or("");
        if reply_text.is_empty() {
            return ephemeral("Reply cannot be empty.");
        }
        return send_relay_reply(ctx_id, reply_text, env).await;
    }

    ephemeral("Unknown modal")
}

/// Show the reply modal with a text input.
fn show_reply_modal(ctx_id: &str) -> Result<Response> {
    let resp = InteractionResponse::modal(
        format!("reply_modal:{ctx_id}"),
        "Reply",
        vec![ActionRow::new(vec![Component::paragraph_input(
            "reply_text",
            "Your reply",
        )])],
    );
    Response::from_json(&resp)
}

/// Send a relay reply back through the originating channel.
async fn send_relay_reply(ctx_id: &str, reply_text: &str, env: &Env) -> Result<Response> {
    let kv = env.kv("KV")?;
    let db = env.d1("DB")?;

    let ctx = match get_conversation_context(&kv, ctx_id).await? {
        Some(c) => c,
        None => return ephemeral("Conversation expired or not found."),
    };

    let subject = if ctx.origin_channel == Channel::Email {
        Some("Re: your message")
    } else {
        None
    };

    match channel::send_reply(
        &ctx.origin_channel,
        env,
        &ctx.reply_metadata,
        &ctx.origin_sender,
        reply_text,
        subject,
    )
    .await
    {
        Ok(()) => {}
        Err(e) => {
            console_log!("Relay reply error: {:?}", e);
            return ephemeral(&format!("Failed to send reply: {e}"));
        }
    }

    let _ = save_message(
        &db,
        &generate_id(),
        &ctx.origin_channel,
        "relay",
        &ctx.origin_recipient,
        &ctx.origin_sender,
        &ctx.tenant_id,
        &ctx.channel_account_id,
        Some("relay"),
    )
    .await;

    ephemeral(&format!(
        "Reply sent to {} via {}.",
        ctx.origin_sender,
        ctx.origin_channel.as_str()
    ))
}

/// Approve an AI-generated draft and send it.
async fn handle_approve(ctx_id: &str, env: &Env) -> Result<Response> {
    let kv = env.kv("KV")?;
    let db = env.d1("DB")?;

    let ctx = match get_conversation_context(&kv, ctx_id).await? {
        Some(c) => c,
        None => return ephemeral("Conversation expired."),
    };

    let draft = match &ctx.ai_draft {
        Some(d) => d.clone(),
        None => return ephemeral("No draft found."),
    };

    let subject = if ctx.origin_channel == Channel::Email {
        Some("Re: your message")
    } else {
        None
    };

    match channel::send_reply(
        &ctx.origin_channel,
        env,
        &ctx.reply_metadata,
        &ctx.origin_sender,
        &draft,
        subject,
    )
    .await
    {
        Ok(()) => {}
        Err(e) => return ephemeral(&format!("Failed to send: {e}")),
    }

    let _ = save_message(
        &db,
        &generate_id(),
        &ctx.origin_channel,
        "outbound",
        &ctx.origin_recipient,
        &ctx.origin_sender,
        &ctx.tenant_id,
        &ctx.channel_account_id,
        Some("ai_approved"),
    )
    .await;

    let _ = delete_conversation_context(&kv, ctx_id).await;

    ephemeral(&format!(
        "Approved and sent to {} via {}.",
        ctx.origin_sender,
        ctx.origin_channel.as_str()
    ))
}

/// Reject an AI draft.
async fn handle_reject(ctx_id: &str, env: &Env) -> Result<Response> {
    let kv = env.kv("KV")?;
    let _ = delete_conversation_context(&kv, ctx_id).await;
    ephemeral("Draft rejected and discarded.")
}

/// Drop/dismiss a forwarded message.
async fn handle_drop(ctx_id: &str, env: &Env) -> Result<Response> {
    let kv = env.kv("KV")?;
    let _ = delete_conversation_context(&kv, ctx_id).await;
    ephemeral("Message dropped.")
}

fn ephemeral(content: &str) -> Result<Response> {
    Response::from_json(&InteractionResponse::ephemeral_message(content))
}

#[cfg(test)]
mod tests {
    use botrelay::discord::parse_interaction;

    #[test]
    fn parse_button_click_has_custom_id() {
        let body = br#"{
            "id": "1",
            "token": "t",
            "type": 3,
            "channel_id": "c",
            "data": {"custom_id": "reply:ctx-123"}
        }"#;
        let i = parse_interaction(body).unwrap();
        assert!(i.is_component_click());
        assert_eq!(
            i.data.as_ref().unwrap().custom_id.as_deref(),
            Some("reply:ctx-123")
        );
    }

    #[test]
    fn modal_text_extracts_reply() {
        let body = br#"{
            "id": "1",
            "token": "t",
            "type": 5,
            "data": {
                "custom_id": "reply_modal:ctx-1",
                "components": [
                    {"components": [{"custom_id": "reply_text", "value": "Hello back!"}]}
                ]
            }
        }"#;
        let i = parse_interaction(body).unwrap();
        assert!(i.is_modal_submit());
        assert_eq!(i.modal_text("reply_text"), Some("Hello back!"));
        assert_eq!(i.modal_text("wrong_id"), None);
    }

    #[test]
    fn custom_id_prefix_parsing() {
        assert_eq!(
            "reply:abc-123-def".strip_prefix("reply:"),
            Some("abc-123-def")
        );
        assert_eq!("approve:xyz".strip_prefix("approve:"), Some("xyz"));
        assert_eq!("reject:xyz".strip_prefix("reject:"), Some("xyz"));
        assert_eq!("drop:xyz".strip_prefix("drop:"), Some("xyz"));
        assert!("unknown:xyz".strip_prefix("reply:").is_none());
    }
}
