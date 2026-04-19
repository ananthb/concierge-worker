pub mod email;
pub mod instagram;
pub mod whatsapp;

use worker::*;

use crate::types::{Channel, InboundMessage};

/// Dispatch a reply to the correct channel adapter.
pub async fn send_reply(
    channel: &Channel,
    env: &Env,
    metadata: &serde_json::Value,
    to: &str,
    body: &str,
    subject: Option<&str>,
) -> Result<()> {
    match channel {
        Channel::WhatsApp => whatsapp::send_reply(env, metadata, to, body).await,
        Channel::Instagram => instagram::send_reply(env, metadata, to, body).await,
        Channel::Email => {
            email::send_reply(env, metadata, to, body, subject).await?;
            Ok(())
        }
        Channel::Discord => Ok(()), // Discord replies handled by interaction handler
    }
}
