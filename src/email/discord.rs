//! Discord bot integration for email forwarding (uses the `botrelay` crate).

use botrelay::discord::{CreateMessage, DiscordBot, Embed, EmbedField, EmbedFooter};
use worker::*;

/// Post an email summary to a Discord channel as an embed.
pub async fn post_email_to_discord(
    bot_token: &str,
    channel_id: &str,
    from: &str,
    to: &str,
    subject: &str,
    body_preview: &str,
    rule_name: &str,
) -> Result<()> {
    let body_truncated = if body_preview.len() > 1000 {
        format!("{}...", &body_preview[..997])
    } else {
        body_preview.to_string()
    };

    let message = CreateMessage {
        embeds: vec![Embed {
            title: Some(if subject.is_empty() {
                "(no subject)".into()
            } else {
                subject.to_string()
            }),
            description: Some(body_truncated),
            color: Some(0xF38020), // Cloudflare orange
            fields: vec![
                EmbedField {
                    name: "From".into(),
                    value: from.to_string(),
                    inline: true,
                },
                EmbedField {
                    name: "To".into(),
                    value: to.to_string(),
                    inline: true,
                },
            ],
            footer: Some(EmbedFooter {
                text: format!("Rule: {rule_name}"),
            }),
        }],
        ..Default::default()
    };

    // `application_id` and `public_key` are not needed for `create_message`;
    // pass empty placeholders.
    let bot = DiscordBot::new(bot_token, "", "");
    if let Err(e) = bot.create_message(channel_id, message).await {
        console_log!("Discord API error: channel={channel_id} err={e}");
    }
    Ok(())
}

/// Post an AI draft reply to Discord for approval.
pub async fn post_ai_draft_for_approval(
    bot_token: &str,
    channel_id: &str,
    from: &str,
    to: &str,
    subject: &str,
    original_body: &str,
    ai_draft: &str,
    rule_name: &str,
) -> Result<()> {
    let original_truncated = if original_body.len() > 500 {
        format!("{}...", &original_body[..497])
    } else {
        original_body.to_string()
    };

    let message = CreateMessage {
        content: "**AI Draft Reply**: React with ✅ to approve, ❌ to discard.".into(),
        embeds: vec![
            Embed {
                title: Some(format!("Re: {subject}")),
                description: Some(format!("**Draft reply:**\n{ai_draft}")),
                color: Some(0x5865F2), // Discord blurple
                fields: vec![
                    EmbedField {
                        name: "From".into(),
                        value: from.to_string(),
                        inline: true,
                    },
                    EmbedField {
                        name: "To".into(),
                        value: to.to_string(),
                        inline: true,
                    },
                ],
                footer: Some(EmbedFooter {
                    text: format!("Rule: {rule_name}"),
                }),
            },
            Embed {
                title: Some("Original message".into()),
                description: Some(original_truncated),
                color: Some(0x99AAB5),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let bot = DiscordBot::new(bot_token, "", "");
    if let Err(e) = bot.create_message(channel_id, message).await {
        console_log!("Discord API error (AI draft): channel={channel_id} err={e}");
    }
    Ok(())
}
