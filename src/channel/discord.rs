//! Outbound replies for the Discord channel.
//!
//! Posts a plain message into the originating channel via
//! `botrelay`'s DiscordBot. The originating channel id is read from the
//! inbound message's raw_metadata.

use botrelay::discord::{CreateMessage, DiscordBot};
use worker::*;

pub async fn send_reply(
    env: &Env,
    metadata: &serde_json::Value,
    _sender: &str,
    body: &str,
) -> Result<()> {
    let token = env
        .secret("DISCORD_BOT_TOKEN")
        .map(|s| s.to_string())
        .unwrap_or_default();
    if token.is_empty() {
        return Err(Error::from("DISCORD_BOT_TOKEN secret not set"));
    }
    let app_id = env
        .secret("DISCORD_APP_ID")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let public_key = env
        .secret("DISCORD_PUBLIC_KEY")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let bot = DiscordBot::new(token, app_id, public_key);

    let channel_id = metadata
        .get("channel_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::from("Discord reply: missing channel_id in metadata"))?;

    let params = CreateMessage {
        content: body.to_string(),
        ..Default::default()
    };
    bot.create_message(channel_id, params)
        .await
        .map(|_| ())
        .map_err(|e| Error::from(format!("Discord create_message: {e:?}")))
}
