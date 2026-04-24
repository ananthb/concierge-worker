//! Discord management page — install CTA + channel picker + uninstall.

use crate::helpers::html_escape;
use crate::types::DiscordConfig;

use super::base::{app_shell, base_html};
use super::HASH;

/// Render a "Connect Discord" CTA for tenants that have no `DiscordConfig`.
pub fn install_cta_html(from: &str, base_url: &str) -> String {
    let install_href = if from.is_empty() {
        format!("{base_url}/admin/discord/install")
    } else {
        format!(
            "{base_url}/admin/discord/install?from={}",
            urlencoding::encode(from)
        )
    };
    let back_href = back_href_for(from, base_url);

    let content = format!(
        r##"<div class="page-pad">
  <p><a href="{back_href}" class="btn ghost sm">&larr; Back</a></p>
  <h1 class="display-sm m-0 mt-8 mb-16">Discord</h1>
  <div class="card p-22">
    <div class="row gap-16">
      <div class="admin-mark icon-chip">{icon}</div>
      <div class="flex-1">
        <div class="fw-600">Install the Concierge bot</div>
        <p class="muted m-0 mt-4 fs-14">Pick a server and approve the bot. We use Discord for AI approvals, digests, and email relay.</p>
      </div>
      <a href="{install_href}" class="btn primary">Install &rarr;</a>
    </div>
  </div>
</div>"##,
        back_href = back_href,
        install_href = install_href,
        icon = discord_icon(),
    );

    let page = app_shell(&content, "Settings", base_url);
    base_html("Connect Discord - Concierge", &page)
}

/// Render the installed-management page: show guild, pick channels, uninstall.
pub fn manage_html(
    cfg: &DiscordConfig,
    channels: &[crate::discord::api::GuildChannel],
    from: &str,
    base_url: &str,
) -> String {
    let back_href = back_href_for(from, base_url);
    let guild = cfg.guild_name.as_deref().unwrap_or("Discord server");

    let options_html = |selected: Option<&str>| -> String {
        let mut out = String::from(r#"<option value="">— none —</option>"#);
        for ch in channels {
            let sel = if Some(ch.id.as_str()) == selected {
                " selected"
            } else {
                ""
            };
            out.push_str(&format!(
                r#"<option value="{id}"{sel}>#{name}</option>"#,
                id = html_escape(&ch.id),
                sel = sel,
                name = html_escape(&ch.name),
            ));
        }
        out
    };

    let approval_opts = options_html(cfg.approval_channel_id.as_deref());
    let digest_opts = options_html(cfg.digest_channel_id.as_deref());
    let relay_opts = options_html(cfg.relay_channel_id.as_deref());

    let empty_note = if channels.is_empty() {
        r#"<p class="muted mt-8 fs-13">No text channels found. The bot may not have access yet — check the server's permissions.</p>"#
    } else {
        ""
    };

    let content = format!(
        r##"<div class="page-pad">
  <p><a href="{back_href}" class="btn ghost sm">&larr; Back</a></p>
  <div class="between m-0 mt-8 mb-16">
    <div>
      <h1 class="display-sm m-0">Discord</h1>
      <p class="muted m-0 mt-4 fs-13">Connected to <strong>{guild}</strong></p>
    </div>
    <div class="row gap-8">
      <a href="{base_url}/admin/discord/install?from={from_enc}" class="btn ghost sm">Re-install</a>
      <button class="btn ghost sm text-warn" hx-delete="{base_url}/admin/discord" hx-confirm="Uninstall the bot and forget these channels?">Uninstall</button>
    </div>
  </div>

  <form class="card p-22" hx-put="{base_url}/admin/discord/config" hx-ext="json-enc" hx-target="{hash}dc-toast" hx-swap="innerHTML">
    <p class="muted m-0 mb-16 fs-14">Pick where we should post to. Per-rule overrides still win over these defaults.</p>

    <div class="form-group">
      <label class="eyebrow lbl">Approvals channel</label>
      <select class="select" name="approval_channel_id">{approval_opts}</select>
      <small class="muted fs-12">AI drafts land here for you to approve or reject.</small>
    </div>

    <div class="form-group">
      <label class="eyebrow lbl">Digests channel</label>
      <select class="select" name="digest_channel_id">{digest_opts}</select>
      <small class="muted fs-12">Periodic activity summaries go here.</small>
    </div>

    <div class="form-group">
      <label class="eyebrow lbl">Email relay channel</label>
      <select class="select" name="relay_channel_id">{relay_opts}</select>
      <small class="muted fs-12">Default for <code>forward_discord</code> rules when none is set on the rule.</small>
    </div>

    {empty_note}

    <div class="row gap-8 mt-16" style="justify-content:flex-end">
      <button type="submit" class="btn primary">Save</button>
    </div>
    <div id="dc-toast" class="mt-8"></div>
  </form>
</div>"##,
        back_href = back_href,
        base_url = base_url,
        guild = html_escape(guild),
        from_enc = urlencoding::encode(from),
        approval_opts = approval_opts,
        digest_opts = digest_opts,
        relay_opts = relay_opts,
        empty_note = empty_note,
        hash = HASH,
    );

    let page = app_shell(&content, "Settings", base_url);
    base_html("Discord - Concierge", &page)
}

fn back_href_for(from: &str, base_url: &str) -> String {
    match from {
        "wizard_channels" => format!("{base_url}/admin/wizard/channels"),
        "wizard_heads_up" => format!("{base_url}/admin/wizard/notifications"),
        _ => format!("{base_url}/admin/settings"),
    }
}

/// Discord's squircle SVG, reused from onboarding.rs's channel_icon.
fn discord_icon() -> &'static str {
    r#"<svg width="22" height="22" viewBox="0 0 24 24" fill="none"><path d="M7 7c1.4-.7 3-1 5-1s3.6.3 5 1l1 1 1.5 4.5c.2 2-.3 3.8-1.5 5.5-1 .3-2 .5-3 .5l-1-1.5c.5-.2 1-.4 1.5-.8-.3-.2-.8-.4-1.2-.5-2 .7-4.6.7-6.6 0-.4.1-.9.3-1.2.5.5.4 1 .6 1.5.8L6 17.5c-1 0-2-.2-3-.5-1.2-1.7-1.7-3.5-1.5-5.5L3 7l1-1z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/></svg>"#
}
