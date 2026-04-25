//! Unified message processing pipeline.
//! All inbound messages from any channel flow through here.

use worker::*;

use crate::ai;
use crate::billing;
use crate::channel;
use crate::helpers::generate_id;
use crate::storage::*;
use crate::types::*;

/// Process an inbound message from WhatsApp, Instagram, or Discord.
///
/// Routes through the ReplyBufferDO so quick-fire messages from the same
/// sender batch into one AI call. wait_seconds=0 (or DO unreachable) falls
/// back to immediate processing.
pub async fn process_inbound(msg: &InboundMessage, env: &Env) -> Result<()> {
    let kv = env.kv("KV")?;
    let db = env.d1("DB")?;

    // 1. Log inbound to unified messages table
    if let Err(e) = save_inbound_message(&db, msg, None).await {
        console_log!("Failed to log inbound message: {:?}", e);
    }

    let wait = lookup_wait_seconds(&kv, msg).await.unwrap_or(0);
    if wait == 0 {
        process_inbound_immediate(msg, env).await?;
    } else if let Err(e) = forward_to_buffer(env, msg, wait).await {
        console_log!("buffer route failed, falling back to immediate: {:?}", e);
        process_inbound_immediate(msg, env).await?;
    }

    Ok(())
}

/// Process a single (possibly already-batched) message immediately, no
/// further buffering. Called both from `process_inbound` (when wait=0)
/// and from `ReplyBufferDO::alarm` after the wait window closes.
pub async fn process_inbound_immediate(msg: &InboundMessage, env: &Env) -> Result<()> {
    let kv = env.kv("KV")?;
    let db = env.d1("DB")?;
    handle_auto_reply(msg, &kv, &db, env).await
}

async fn lookup_wait_seconds(kv: &kv::KvStore, msg: &InboundMessage) -> Result<u32> {
    let cfg = match msg.channel {
        Channel::WhatsApp => get_whatsapp_account(kv, &msg.channel_account_id)
            .await?
            .map(|a| a.auto_reply),
        Channel::Instagram => get_instagram_account(kv, &msg.channel_account_id)
            .await?
            .map(|a| a.auto_reply),
        Channel::Discord => get_discord_config_by_tenant(kv, &msg.tenant_id)
            .await?
            .map(|c| c.auto_reply),
        Channel::Email => get_email_address(kv, &msg.tenant_id, &msg.channel_account_id)
            .await?
            .map(|a| a.auto_reply),
    };
    Ok(cfg.map(|c| c.wait_seconds).unwrap_or(0))
}

async fn forward_to_buffer(env: &Env, msg: &InboundMessage, wait_seconds: u32) -> Result<()> {
    let ns = env.durable_object("REPLY_BUFFER")?;
    // One DO per conversation: tenant + channel + sender.
    let id_name = format!("{}:{}:{}", msg.tenant_id, msg.channel.as_str(), msg.sender);
    let stub = ns.id_from_name(&id_name)?.get_stub()?;

    let payload = serde_json::json!({
        "msg": msg,
        "wait_seconds": wait_seconds,
    });
    let body = serde_json::to_string(&payload)?;

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(wasm_bindgen::JsValue::from_str(&body)));
    let req = Request::new_with_init("https://buffer.do/push", &init)?;
    let _ = stub.fetch_with_request(req).await?;
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
            account.filter(|a| a.enabled).map(|a| a.auto_reply)
        }
        Channel::Email => {
            let addr = get_email_address(kv, &msg.tenant_id, &msg.channel_account_id).await?;
            addr.map(|a| a.auto_reply)
        }
        Channel::Discord => get_discord_config_by_tenant(kv, &msg.tenant_id)
            .await?
            .map(|c| c.auto_reply),
    };

    let config = match config {
        Some(c) if c.enabled => c,
        _ => return Ok(()),
    };

    // Only AI replies cost a credit. Static auto-replies are free, so we
    // skip the deduction entirely on that branch.
    let is_ai = matches!(config.mode, AutoReplyMode::Ai);
    if is_ai && !billing::try_deduct(&db, &msg.tenant_id).await? {
        console_log!("Tenant {} out of AI-reply credits, skipping", msg.tenant_id);
        return Ok(());
    }

    // Generate reply
    let reply = match config.mode {
        AutoReplyMode::Static => config.prompt.clone(),
        AutoReplyMode::Ai => {
            // Truncate to limit prompt injection surface
            let safe_body: String = msg.body.chars().take(1000).collect();

            // Scan for prompt injection before generating AI response
            if ai::is_prompt_injection(env, &safe_body).await {
                console_log!(
                    "Prompt injection detected from {} in tenant {}, skipping AI reply",
                    msg.sender,
                    msg.tenant_id
                );
                if let Err(e) = billing::restore_credit(&db, &msg.tenant_id).await {
                    console_log!("Failed to restore credit: {:?}", e);
                }
                return Ok(());
            }

            let mut context = serde_json::Map::new();
            if let Some(ref name) = msg.sender_name {
                let safe_name: String = name.chars().take(100).collect();
                context.insert("sender_name".into(), serde_json::Value::String(safe_name));
            }
            context.insert("message".into(), serde_json::Value::String(safe_body));
            match ai::generate_response(env, &config.prompt, &context).await {
                Ok(r) => r,
                Err(e) => {
                    console_log!("AI auto-reply error: {:?}", e);
                    // Restore credit since we didn't send
                    if let Err(re) = billing::restore_credit(&db, &msg.tenant_id).await {
                        console_log!("Failed to restore credit: {:?}", re);
                    }
                    return Ok(());
                }
            }
        }
    };

    if reply.is_empty() {
        // Restore credit if we deducted one (AI path only).
        if is_ai {
            if let Err(e) = billing::restore_credit(&db, &msg.tenant_id).await {
                console_log!("Failed to restore credit: {:?}", e);
            }
        }
        return Ok(());
    }

    // Send reply via channel adapter
    if let Err(e) = channel::send_reply(
        &msg.channel,
        env,
        &msg.raw_metadata,
        &msg.sender,
        &reply,
        None,
    )
    .await
    {
        console_log!("Auto-reply send error: {:?}", e);
        if is_ai {
            if let Err(re) = billing::restore_credit(&db, &msg.tenant_id).await {
                console_log!("Failed to restore credit: {:?}", re);
            }
        }
        return Ok(());
    }

    // Log outbound
    if let Err(e) = save_message(
        db,
        &generate_id(),
        &msg.channel,
        "outbound",
        &msg.recipient,
        &msg.sender,
        &msg.tenant_id,
        &msg.channel_account_id,
        Some("auto_reply"),
    )
    .await
    {
        console_log!("Failed to log outbound message: {:?}", e);
    }

    Ok(())
}
