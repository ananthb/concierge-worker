//! Unified message processing pipeline.
//! All inbound messages from any channel flow through here.

use worker::*;

use crate::ai;
use crate::channel;
use crate::helpers::generate_id;
use crate::storage::*;
use crate::types::*;

/// Process an inbound message from WhatsApp or Instagram.
/// Email has its own routing engine and calls into this only for Discord forwarding.
pub async fn process_inbound(msg: &InboundMessage, env: &Env) -> Result<()> {
    let kv = env.kv("CALENDARS_KV")?;
    let db = env.d1("DB")?;

    // 1. Log inbound to unified messages table
    let _ = save_inbound_message(&db, msg, None).await;

    // 2. Channel-specific routing
    match msg.channel {
        Channel::WhatsApp | Channel::Instagram => {
            handle_auto_reply(msg, &kv, &db, env).await?;
        }
        Channel::Email => {
            // Email uses its own routing engine in email::handler
            // This function is not called for email's main path
        }
        Channel::Discord => {
            // Discord inbound = operator interaction, handled by discord::components
        }
    }

    Ok(())
}

/// Handle simple auto-reply for WhatsApp/Instagram.
async fn handle_auto_reply(
    msg: &InboundMessage,
    kv: &kv::KvStore,
    db: &D1Database,
    env: &Env,
) -> Result<()> {
    let config = match msg.channel {
        Channel::WhatsApp => {
            let account = get_whatsapp_account(kv, &msg.channel_account_id).await?;
            account.map(|a| a.auto_reply)
        }
        Channel::Instagram => {
            let account = get_instagram_account(kv, &msg.channel_account_id).await?;
            account
                .filter(|a| a.enabled)
                .map(|a| a.auto_reply)
        }
        _ => None,
    };

    let config = match config {
        Some(c) if c.enabled => c,
        _ => return Ok(()),
    };

    // Generate reply
    let reply = match config.mode {
        AutoReplyMode::Static => config.prompt.clone(),
        AutoReplyMode::Ai => {
            let mut context = serde_json::Map::new();
            if let Some(ref name) = msg.sender_name {
                context.insert(
                    "sender_name".into(),
                    serde_json::Value::String(name.clone()),
                );
            }
            context.insert(
                "message".into(),
                serde_json::Value::String(msg.body.clone()),
            );
            match ai::generate_response(env, &config.prompt, &context).await {
                Ok(r) => r,
                Err(e) => {
                    console_log!("AI auto-reply error: {:?}", e);
                    return Ok(());
                }
            }
        }
    };

    if reply.is_empty() {
        return Ok(());
    }

    // Send reply via channel adapter
    if let Err(e) =
        channel::send_reply(&msg.channel, env, &msg.raw_metadata, &msg.sender, &reply, None).await
    {
        console_log!("Auto-reply send error: {:?}", e);
        return Ok(());
    }

    // Log outbound
    let _ = save_message(
        db,
        &generate_id(),
        &msg.channel,
        "outbound",
        &msg.recipient,
        &msg.sender,
        &reply,
        None,
        &msg.tenant_id,
        &msg.channel_account_id,
        Some("auto_reply"),
        None,
    )
    .await;

    Ok(())
}
