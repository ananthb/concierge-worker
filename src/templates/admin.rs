//! Admin dashboard and calendar management templates

use crate::helpers::*;
use crate::types::*;

use super::base::{base_html, timezone_options};
use super::HASH;

pub fn auth_login_html(base_url: &str, client_id: &str) -> String {
    let redirect_uri = format!("{}/auth/callback", base_url);
    let google_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=online&prompt=select_account",
        urlencoding::encode(client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode("openid email profile"),
    );

    let style = CalendarStyle::default();
    let content = format!(
        "<div style=\"max-width: 400px; margin: 4rem auto; text-align: center;\">
            <h1 style=\"margin-bottom: 0.5rem;\">Concierge</h1>
            <p style=\"color: {hash}666; margin-bottom: 2rem;\">Sign in to manage your calendars, bookings, and forms.</p>
            <a href=\"{google_url}\" class=\"btn\" style=\"display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; font-size: 1rem;\">
                <svg width=\"18\" height=\"18\" viewBox=\"0 0 18 18\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M17.64 9.2c0-.637-.057-1.251-.164-1.84H9v3.481h4.844a4.14 4.14 0 01-1.796 2.716v2.259h2.908c1.702-1.567 2.684-3.875 2.684-6.615z\" fill=\"#4285F4\"/><path d=\"M9 18c2.43 0 4.467-.806 5.956-2.18l-2.908-2.259c-.806.54-1.837.86-3.048.86-2.344 0-4.328-1.584-5.036-3.711H.957v2.332A8.997 8.997 0 009 18z\" fill=\"#34A853\"/><path d=\"M3.964 10.71A5.41 5.41 0 013.682 9c0-.593.102-1.17.282-1.71V4.958H.957A8.996 8.996 0 000 9c0 1.452.348 2.827.957 4.042l3.007-2.332z\" fill=\"#FBBC05\"/><path d=\"M9 3.58c1.321 0 2.508.454 3.44 1.345l2.582-2.58C13.463.891 11.426 0 9 0A8.997 8.997 0 00.957 4.958L3.964 7.29C4.672 5.163 6.656 3.58 9 3.58z\" fill=\"#EA4335\"/></svg>
                Sign in with Google
            </a>
        </div>",
        google_url = html_escape(&google_url),
        hash = HASH,
    );

    base_html("Sign In - Concierge", &content, &style)
}

pub fn admin_settings_html(creds: &TenantCredentials, base_url: &str) -> String {
    let style = CalendarStyle::default();
    let content = format!(
        "<p><a href=\"{base_url}/admin\">&larr; Back to Dashboard</a></p>
        <h1>Settings</h1>

        <div class=\"card\">
            <h2>Google Integration</h2>
            <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">Service account credentials for Google Calendar and Google Forms.</p>
            <form hx-put=\"{base_url}/admin/settings\" hx-swap=\"none\"
                  hx-on::before-request=\"this.querySelector('button[type=submit]').disabled=true\"
                  hx-on::after-request=\"this.querySelector('button[type=submit]').disabled=false\">
                <div class=\"form-group\">
                    <label>Service Account Email</label>
                    <input type=\"email\" name=\"google_service_account_email\" value=\"{google_email}\" placeholder=\"concierge@project.iam.gserviceaccount.com\" style=\"font-family: monospace;\">
                </div>
                <div class=\"form-group\">
                    <label>Private Key</label>
                    <textarea name=\"google_private_key\" rows=\"4\" placeholder=\"-----BEGIN PRIVATE KEY-----\" style=\"font-family: monospace; font-size: 0.8rem;\">{google_key}</textarea>
                    <small class=\"text-muted\">The private_key field from your service account JSON key file.</small>
                </div>
                <div style=\"display: flex; justify-content: flex-end; margin-top: 1rem;\">
                    <button type=\"submit\" class=\"btn\">Save Credentials</button>
                </div>
            </form>
        </div>

        <div class=\"card\">
            <h2>Account</h2>
            <p><a href=\"{base_url}/auth/logout\" class=\"btn btn-secondary\">Sign Out</a></p>
        </div>",
        base_url = base_url,
        google_email = html_escape(creds.google_service_account_email.as_deref().unwrap_or("")),
        google_key = html_escape(creds.google_private_key.as_deref().unwrap_or("")),
    );

    base_html("Settings - Concierge", &content, &style)
}

pub fn admin_dashboard_html(
    calendars: &[CalendarConfig],
    whatsapp_accounts: &[WhatsAppAccount],
    form_resources: &[GoogleFormResource],
    instagram_accounts: &[InstagramAccount],
    base_url: &str,
) -> String {
    // Split calendars into active and archived
    let active_calendars: Vec<_> = calendars.iter().filter(|c| !c.archived).collect();
    let archived_calendars: Vec<_> = calendars.iter().filter(|c| c.archived).collect();

    let calendar_rows: String = active_calendars
        .iter()
        .map(|cal| {
            let bc = cal.booking_links.len();
            let vc = cal.view_links.len();
            format!(
                "<tr>
                    <td><a href=\"{base_url}/admin/calendars/{id}\">{name}</a></td>
                    <td>{bc} booking {bc_s}</td>
                    <td>{vc} view {vc_s}</td>
                    <td>{updated}</td>
                    <td>
                        <a href=\"{base_url}/admin/calendars/{id}\" class=\"btn btn-sm\">Edit</a>
                        <button class=\"btn btn-sm btn-secondary\"
                                hx-post=\"{base_url}/admin/calendars/{id}/archive\"
                                hx-confirm=\"Archive this calendar? It will become read-only.\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Archive</button>
                    </td>
                </tr>",
                base_url = base_url,
                id = html_escape(&cal.id),
                name = html_escape(&cal.name),
                bc = bc,
                bc_s = if bc == 1 { "link" } else { "links" },
                vc = vc,
                vc_s = if vc == 1 { "link" } else { "links" },
                updated = html_escape(cal.updated_at.split('T').next().unwrap_or("")),
            )
        })
        .collect();

    // Archived calendars section
    let archived_calendar_rows: String = archived_calendars
        .iter()
        .map(|cal| {
            let bc = cal.booking_links.len();
            let vc = cal.view_links.len();
            format!(
                "<tr>
                    <td>{name} <span class=\"text-muted\" style=\"font-size:0.85em;\">(archived)</span></td>
                    <td>{bc} booking {bc_s}</td>
                    <td>{vc} view {vc_s}</td>
                    <td>{updated}</td>
                    <td>
                        <a href=\"{base_url}/admin/calendars/{id}\" class=\"btn btn-sm\">View</a>
                        <button class=\"btn btn-sm\"
                                hx-post=\"{base_url}/admin/calendars/{id}/unarchive\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Unarchive</button>
                    </td>
                </tr>",
                base_url = base_url,
                id = html_escape(&cal.id),
                name = html_escape(&cal.name),
                bc = bc,
                bc_s = if bc == 1 { "link" } else { "links" },
                vc = vc,
                vc_s = if vc == 1 { "link" } else { "links" },
                updated = html_escape(cal.updated_at.split('T').next().unwrap_or("")),
            )
        })
        .collect();

    // Build archived sections HTML
    let archived_calendars_section = if archived_calendars.is_empty() {
        String::new()
    } else {
        format!(
            "<details style=\"margin-top: 1rem;\">
                <summary class=\"text-muted\" style=\"cursor: pointer;\">Archived Calendars ({count})</summary>
                <table style=\"margin-top: 0.5rem;\">
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Booking Links</th>
                            <th>View Links</th>
                            <th>Updated</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody>{rows}</tbody>
                </table>
            </details>",
            count = archived_calendars.len(),
            rows = archived_calendar_rows
        )
    };

    let content = format!(
        "<h1 style=\"display: flex; align-items: center; gap: 0.5rem;\">
            <img src=\"/logo.svg\" alt=\"\" style=\"width: 32px; height: 32px;\">
            Concierge Admin
        </h1>
        <p style=\"margin-bottom: 1rem;\"><a href=\"{base_url}/admin/settings\" class=\"btn btn-secondary\">Settings</a></p>

        <div style=\"display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 1rem; margin: 1.5rem 0;\">
            <a href=\"{base_url}/admin/whatsapp\" class=\"card\" style=\"text-decoration: none; text-align: center; padding: 1rem;\">
                <div style=\"font-size: 1.5rem; font-weight: bold;\">{wa_count}</div>
                <div class=\"text-muted\">WhatsApp Account{wa_s}</div>
            </a>
            <a href=\"{base_url}/admin/forms\" class=\"card\" style=\"text-decoration: none; text-align: center; padding: 1rem;\">
                <div style=\"font-size: 1.5rem; font-weight: bold;\">{form_count}</div>
                <div class=\"text-muted\">Form{form_s}</div>
            </a>
            <a href=\"{base_url}/admin/instagram\" class=\"card\" style=\"text-decoration: none; text-align: center; padding: 1rem;\">
                <div style=\"font-size: 1.5rem; font-weight: bold;\">{ig_count}</div>
                <div class=\"text-muted\">Instagram Account{ig_s}</div>
            </a>
        </div>

        <h2 style=\"margin-top: 2rem;\">Calendars</h2>
        <p style=\"margin: 1rem 0;\">
            <button class=\"btn\" hx-post=\"{base_url}/admin/calendars\" hx-target=\"{hash}calendar-list tbody\" hx-swap=\"beforeend\">
                + New Calendar
            </button>
        </p>
        <table id=\"calendar-list\">
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Booking Links</th>
                    <th>View Links</th>
                    <th>Updated</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {calendar_rows}
            </tbody>
        </table>
        {archived_calendars_section}
        <script>
        function copyLink(path, btn) {{
            const url = window.location.origin + path;
            navigator.clipboard.writeText(url).then(() => {{
                const orig = btn.textContent;
                btn.textContent = 'Copied!';
                btn.style.background = '#28a745';
                setTimeout(() => {{
                    btn.textContent = orig;
                    btn.style.background = '';
                }}, 1500);
            }});
        }}
        </script>",
        base_url = base_url,
        wa_count = whatsapp_accounts.len(),
        wa_s = if whatsapp_accounts.len() == 1 { "" } else { "s" },
        form_count = form_resources.len(),
        form_s = if form_resources.len() == 1 { "" } else { "s" },
        ig_count = instagram_accounts.len(),
        ig_s = if instagram_accounts.len() == 1 { "" } else { "s" },
        calendar_rows = if calendar_rows.is_empty() {
            "<tr><td colspan=\"5\" class=\"text-muted\" style=\"text-align:center;\">No calendars yet.</td></tr>".to_string()
        } else {
            calendar_rows
        },
        archived_calendars_section = archived_calendars_section,
        hash = HASH,
    );

    base_html("Concierge Admin", &content, &CalendarStyle::default())
}

pub fn admin_calendar_html(
    calendar: &CalendarConfig,
    time_slots: &[TimeSlot],
    base_url: &str,
) -> String {
    // Build time slots summary
    let time_slots_summary = if time_slots.is_empty() {
        "<p class=\"text-muted\">No time slots configured. Bookings won't be available until you configure availability.</p>".to_string()
    } else {
        let days = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];
        let mut summary_html = String::from("<table style=\"width: 100%; margin-top: 0.5rem;\"><thead><tr><th>Day</th><th>Hours</th><th>Duration</th><th>Capacity</th></tr></thead><tbody>");

        // Group slots by day of week
        for (dow, day_name) in days.iter().enumerate() {
            let day_slots: Vec<_> = time_slots
                .iter()
                .filter(|s| s.day_of_week == Some(dow as i32))
                .collect();

            if !day_slots.is_empty() {
                for slot in day_slots {
                    summary_html.push_str(&format!(
                        "<tr><td>{}</td><td>{} - {}</td><td>{} min</td><td>{}</td></tr>",
                        day_name,
                        format_time(&slot.start_time),
                        format_time(&slot.end_time),
                        slot.slot_duration,
                        slot.max_bookings
                    ));
                }
            }
        }

        // Add specific date slots
        let date_slots: Vec<_> = time_slots
            .iter()
            .filter(|s| s.specific_date.is_some())
            .collect();
        for slot in date_slots {
            if let Some(date) = &slot.specific_date {
                summary_html.push_str(&format!(
                    "<tr><td>{}</td><td>{} - {}</td><td>{} min</td><td>{}</td></tr>",
                    html_escape(date),
                    format_time(&slot.start_time),
                    format_time(&slot.end_time),
                    slot.slot_duration,
                    slot.max_bookings
                ));
            }
        }

        summary_html.push_str("</tbody></table>");
        summary_html
    };

    // WhatsApp is the only supported channel
    let channel_options = [("whatsapp", "WhatsApp")];
    let default_channel = "whatsapp";

    let js_channel_options: String = channel_options
        .iter()
        .map(|(value, label)| {
            format!(
                "<option value=\"{}\" ${{r.channel==='{}'?'selected':''}}>{}</option>",
                value, value, label
            )
        })
        .collect::<Vec<_>>()
        .join("\\n                                ");

    let digest_json = serde_json::to_string(&calendar.digest)
        .unwrap_or_else(|_| r#"{"frequency":"none","responders":[]}"#.into());

    let booking_links_html: String = calendar
        .booking_links
        .iter()
        .map(|link| {
            let url = format!("{}/book/{}/{}", base_url, calendar.id, link.slug);
            format!(
                "<tr>
                    <td>{name}</td>
                    <td><div class=\"url-cell\"><code>{url}</code><button class=\"btn btn-copy\" onclick=\"copyUrl(this, '{url}')\">Copy</button></div></td>
                    <td>{duration} min</td>
                    <td>{status}</td>
                    <td>
                        <a href=\"{base_url}/admin/calendars/{cal_id}/booking/{link_id}\" class=\"btn btn-sm\">Edit</a>
                        <button class=\"btn btn-sm btn-danger\"
                                hx-delete=\"{base_url}/admin/calendars/{cal_id}/booking/{link_id}\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Delete</button>
                    </td>
                </tr>",
                base_url = base_url,
                cal_id = html_escape(&calendar.id),
                link_id = html_escape(&link.id),
                url = html_escape(&url),
                name = html_escape(&link.name),
                duration = link.duration,
                status = if link.enabled { "Enabled" } else { "Disabled" },
            )
        })
        .collect();

    let view_links_html: String = calendar
        .view_links
        .iter()
        .map(|link| {
            let url = format!("{}/view/{}/{}", base_url, calendar.id, link.slug);
            format!(
                "<tr>
                    <td>{name}</td>
                    <td><div class=\"url-cell\"><code>{url}</code><button class=\"btn btn-copy\" onclick=\"copyUrl(this, '{url}')\">Copy</button></div></td>
                    <td>{view_type:?}</td>
                    <td>{status}</td>
                    <td>
                        <a href=\"{base_url}/admin/calendars/{cal_id}/view/{link_id}\" class=\"btn btn-sm\">Edit</a>
                        <button class=\"btn btn-sm btn-danger\"
                                hx-delete=\"{base_url}/admin/calendars/{cal_id}/view/{link_id}\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Delete</button>
                    </td>
                </tr>",
                base_url = base_url,
                cal_id = html_escape(&calendar.id),
                link_id = html_escape(&link.id),
                url = html_escape(&url),
                name = html_escape(&link.name),
                view_type = link.view_type,
                status = if link.enabled { "Enabled" } else { "Disabled" },
            )
        })
        .collect();

    let form_links_html: String = calendar
        .form_links
        .iter()
        .map(|link| {
            let url = format!("{}/form/{}/{}", base_url, calendar.id, link.slug);
            let form_display = if link.google_form_url.is_empty() {
                "Not set".to_string()
            } else {
                "Configured".to_string()
            };
            format!(
                "<tr>
                    <td>{name}</td>
                    <td><div class=\"url-cell\"><code>{url}</code><button class=\"btn btn-copy\" onclick=\"copyUrl(this, '{url}')\">Copy</button></div></td>
                    <td>{form_display}</td>
                    <td>{status}</td>
                    <td>
                        <a href=\"{base_url}/admin/calendars/{cal_id}/form/{link_id}\" class=\"btn btn-sm\">Edit</a>
                        <button class=\"btn btn-sm btn-danger\"
                                hx-delete=\"{base_url}/admin/calendars/{cal_id}/form/{link_id}\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Delete</button>
                    </td>
                </tr>",
                base_url = base_url,
                cal_id = html_escape(&calendar.id),
                link_id = html_escape(&link.id),
                url = html_escape(&url),
                name = html_escape(&link.name),
                form_display = form_display,
                status = if link.enabled { "Enabled" } else { "Disabled" },
            )
        })
        .collect();

    let instagram_sources_html: String = calendar
        .instagram_sources
        .iter()
        .map(|source| {
            let last_synced = source
                .last_synced_at
                .as_ref()
                .map(|s| s.split('T').next().unwrap_or("Never"))
                .unwrap_or("Never");
            format!(
                "<tr>
                    <td>@{username}</td>
                    <td>{last_synced}</td>
                    <td>{status}</td>
                    <td>
                        <button class=\"btn btn-sm btn-danger\"
                                hx-delete=\"{base_url}/instagram/disconnect/{cal_id}/{source_id}\"
                                hx-confirm=\"Disconnect this Instagram account?\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Disconnect</button>
                    </td>
                </tr>",
                base_url = base_url,
                cal_id = html_escape(&calendar.id),
                source_id = html_escape(&source.id),
                username = html_escape(&source.instagram_username),
                last_synced = last_synced,
                status = if source.enabled {
                    "Enabled"
                } else {
                    "Disabled"
                },
            )
        })
        .collect();

    let content = format!(
        "<style>
            .tabs {{ display: flex; gap: 0.5rem; margin-bottom: 1.5rem; flex-wrap: wrap; }}
            .tab {{ padding: 0.5rem 1rem; background: var(--bg-muted); border-radius: 4px; cursor: pointer; border: none; font-size: 1rem; text-decoration: none; color: var(--cal-text); }}
            .tab.active {{ background: var(--cal-primary); color: white; }}
            .tab-content {{ display: none; }}
            .tab-content.active {{ display: block; }}
            .form-actions {{ display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem; }}
        </style>

        <p><a href=\"{base_url}/admin\">&larr; Back to Dashboard</a></p>
        <h1>{name}</h1>
        {archived_notice}

        <div class=\"tabs\">
            <a href=\"#settings\" class=\"tab active\" onclick=\"showTab('settings', this)\">Settings</a>
            <a href=\"#google-calendar\" class=\"tab\" onclick=\"showTab('google-calendar', this)\">Google Calendar</a>
            <a href=\"#bookings\" class=\"tab\" onclick=\"showTab('bookings', this)\">Bookings</a>
            {digest_tab}
        </div>

        <div id=\"tab-settings\" class=\"tab-content active\">
            <div class=\"card\">
                <h2>Calendar Settings</h2>
                <form hx-put=\"{base_url}/admin/calendars/{id}\" hx-swap=\"none\" hx-on::before-request=\"this.querySelector('button[type=submit]').disabled=true;this.querySelector('button[type=submit]').textContent='Saving...'\" hx-on::after-request=\"this.querySelector('button[type=submit]').disabled=false;this.querySelector('button[type=submit]').textContent='Save Settings'\">
                    <div class=\"form-group\">
                        <label>Name</label>
                        <input type=\"text\" name=\"name\" value=\"{name}\" required>
                    </div>
                    <div class=\"form-group\">
                        <label>Description</label>
                        <textarea name=\"description\" rows=\"2\">{description}</textarea>
                    </div>
                    <div class=\"form-group\">
                        <label>Timezone</label>
                        <select name=\"timezone\">
                            {timezone_options}
                        </select>
                    </div>
                    <div class=\"form-group\">
                        <label>Allowed Domains (for embedding)</label>
                        <textarea name=\"allowed_origins\" rows=\"3\" placeholder=\"https://example.com&#10;https://another-site.com&#10;(leave empty to allow all)\">{allowed_origins}</textarea>
                        <small class=\"text-muted\">One domain per line. Leave empty to allow embedding from any domain.</small>
                    </div>
                    <div class=\"form-group\">
                        <label>Google Calendar ID</label>
                        <input type=\"text\" name=\"google_calendar_id\" value=\"{google_calendar_id}\" placeholder=\"example@group.calendar.google.com\" style=\"font-family: monospace;\">
                        <small class=\"text-muted\">Share your Google Calendar with the service account email, then paste the Calendar ID here.</small>
                    </div>
                    <div class=\"form-group\">
                        <label>Custom CSS</label>
                        <textarea name=\"custom_css\" rows=\"5\" style=\"font-family: monospace; font-size: 0.9rem;\" placeholder=\"/* Custom styles */&#10;.booking-container {{ max-width: 500px; }}\">{custom_css}</textarea>
                        <small class=\"text-muted\">CSS variables: <code>--cal-primary</code>, <code>--cal-text</code>, <code>--cal-bg</code>, <code>--cal-border-radius</code>, <code>--cal-font</code></small>
                    </div>
                    <div class=\"form-actions\">
                        <button type=\"submit\" class=\"btn\">Save Settings</button>
                    </div>
                </form>
            </div>

            <div class=\"card\">
                <h2>Form Links (Google Forms)</h2>
                <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">Embed Google Forms that can be shared or embedded on your site.</p>
                <button class=\"btn\" hx-post=\"{base_url}/admin/calendars/{id}/form\"
                        hx-target=\"{hash}form-links tbody\" hx-swap=\"beforeend\">+ Add Form Link</button>
                <table id=\"form-links\" style=\"margin-top: 1rem;\">
                    <thead><tr><th>Name</th><th>URL</th><th>Google Form</th><th>Status</th><th>Actions</th></tr></thead>
                    <tbody>{form_links_html}</tbody>
                </table>
            </div>

            <div class=\"card\">
                <h2>View Links</h2>
                <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">Public calendar views that can be embedded or shared.</p>
                <button class=\"btn\" hx-post=\"{base_url}/admin/calendars/{id}/view\"
                        hx-target=\"{hash}view-links tbody\" hx-swap=\"beforeend\">+ Add View Link</button>
                <table id=\"view-links\" style=\"margin-top: 1rem;\">
                    <thead><tr><th>Name</th><th>URL</th><th>Type</th><th>Status</th><th>Actions</th></tr></thead>
                    <tbody>{view_links_html}</tbody>
                </table>
            </div>

            <div class=\"card danger\">
                <h2>Danger Zone</h2>
                <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">Permanently delete this calendar and all its data.</p>
                <button class=\"btn btn-danger\"
                        hx-delete=\"{base_url}/admin/calendars/{id}\"
                        hx-confirm=\"Are you sure you want to permanently delete this calendar? This action cannot be undone.\"
                        hx-on::after-request=\"if(event.detail.successful) window.location.href='/admin'\">Delete Calendar</button>
            </div>
        </div>

        <div id=\"tab-google-calendar\" class=\"tab-content\">
            <div class=\"card\">
                <h2>Google Calendar Integration</h2>
                <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">Connect a Google Calendar to display events and automatically create events for bookings. Share the calendar with your service account email.</p>
                <div class=\"form-group\">
                    <label>Google Calendar ID</label>
                    <input type=\"text\" id=\"google-calendar-id\" value=\"{google_calendar_id}\" placeholder=\"example@group.calendar.google.com\" style=\"font-family: monospace;\">
                    <small class=\"text-muted\">Found in Google Calendar Settings &rarr; Integrate calendar. Share the calendar with your service account email.</small>
                </div>
                <p class=\"text-muted\"><strong>Status:</strong> {gcal_status}</p>
                {gcal_link_html}
            </div>

            <div class=\"card\">
                <h2>Instagram Sources</h2>
                <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">Connect Instagram accounts to automatically import events from posts using AI.</p>
                <a href=\"{base_url}/instagram/auth/{id}\" class=\"btn\">Connect Instagram Account</a>
                <table id=\"instagram-sources\" style=\"margin-top: 1rem;\">
                    <thead><tr><th>Account</th><th>Last Synced</th><th>Status</th><th>Actions</th></tr></thead>
                    <tbody>{instagram_sources_html}</tbody>
                </table>
            </div>
        </div>

        <div id=\"tab-bookings\" class=\"tab-content\">
            <div class=\"card\">
                <h2>Time Slots</h2>
                {time_slots_summary}
                <p style=\"margin-top: 1rem;\"><a href=\"{base_url}/admin/calendars/{id}/slots\" class=\"btn\">Configure Available Slots</a></p>
            </div>

            <div class=\"card\">
                <h2>Booking Links</h2>
                <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">Public booking pages that customers can use to schedule appointments.</p>
                <button class=\"btn\" hx-post=\"{base_url}/admin/calendars/{id}/booking\"
                        hx-target=\"{hash}booking-links tbody\" hx-swap=\"beforeend\">+ Add Booking Link</button>
                <table id=\"booking-links\" style=\"margin-top: 1rem;\">
                    <thead><tr><th>Name</th><th>URL</th><th>Duration</th><th>Status</th><th>Actions</th></tr></thead>
                    <tbody>{booking_links_html}</tbody>
                </table>
            </div>

            <div class=\"card\">
                <h2>All Bookings</h2>
                <p><a href=\"{base_url}/admin/calendars/{id}/bookings\" class=\"btn\">View All Bookings</a></p>
            </div>
        </div>

        {digest_content}

        <script>
            function showTab(name, el) {{
                document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
                document.querySelectorAll('.tab-content').forEach(t => t.classList.remove('active'));
                document.getElementById('tab-' + name).classList.add('active');
                if (el) el.classList.add('active');
            }}
            // Handle initial hash on page load
            (function() {{
                const hash = window.location.hash.slice(1);
                if (hash && document.getElementById('tab-' + hash)) {{
                    showTab(hash, document.querySelector('a[href=\"#' + hash + '\"]'));
                }}
            }})();
            // Handle browser back/forward
            window.addEventListener('hashchange', function() {{
                const hash = window.location.hash.slice(1);
                if (hash && document.getElementById('tab-' + hash)) {{
                    showTab(hash, document.querySelector('a[href=\"#' + hash + '\"]'));
                }}
            }});

            {digest_script}
        </script>",
        base_url = base_url,
        id = html_escape(&calendar.id),
        name = html_escape(&calendar.name),
        description = html_escape(calendar.description.as_deref().unwrap_or("")),
        timezone_options = timezone_options(&calendar.timezone),
        allowed_origins = html_escape(&calendar.allowed_origins.join("\n")),
        custom_css = html_escape(&calendar.style.custom_css),
        google_calendar_id = html_escape(calendar.google_calendar_id.as_deref().unwrap_or("")),
        gcal_status = if calendar.google_calendar_id.is_some() { "Connected" } else { "Not configured" },
        gcal_link_html = if let Some(ref gcal_id) = calendar.google_calendar_id {
            format!(
                "<p style=\"margin-top: 0.5rem;\"><a href=\"https://calendar.google.com/calendar/embed?src={}\" target=\"_blank\">Open in Google Calendar</a></p>",
                html_escape(gcal_id)
            )
        } else {
            String::new()
        },
        booking_links_html = booking_links_html,
        form_links_html = form_links_html,
        view_links_html = view_links_html,
        instagram_sources_html = instagram_sources_html,
        time_slots_summary = time_slots_summary,
        archived_notice = if calendar.archived {
            "<div class=\"card warning\" style=\"margin-bottom: 1rem;\">
                <p style=\"margin: 0;\"><strong>This calendar is archived.</strong> It is read-only. Unarchive from the dashboard to make changes.</p>
            </div>"
        } else { "" },
        hash = HASH,
        digest_tab = r##"<a href="#digest" class="tab" onclick="showTab('digest', this)">Digest</a>"##,
        digest_content = {
            format!(r#"<div id="tab-digest" class="tab-content">
            <div class="card">
                <h2>Booking Digest</h2>
                <p class="text-muted" style="margin-bottom:1rem;">Receive periodic summaries of new bookings.</p>
                <form hx-put="{base_url}/admin/calendars/{id}/digest" hx-swap="none" hx-on::before-request="updateDigestField();this.querySelector('button[type=submit]').disabled=true;this.querySelector('button[type=submit]').textContent='Saving...'" hx-on::after-request="this.querySelector('button[type=submit]').disabled=false;this.querySelector('button[type=submit]').textContent='Save Digest Settings'">
                    <div class="form-group">
                        <label>Frequency</label>
                        <select id="digest-frequency" onchange="digest.frequency=this.value">
                            <option value="none">Disabled</option>
                            <option value="daily">Daily</option>
                            <option value="weekly">Weekly</option>
                        </select>
                    </div>
                    <h4 style="margin-top:1rem;margin-bottom:0.5rem;">Digest Recipients</h4>
                    <div id="digest-responders-list"></div>
                    <button type="button" class="btn btn-secondary" onclick="addDigestResponder()" style="margin-bottom:1rem;">+ Add Recipient</button>
                    <input type="hidden" name="digest_json" id="digest-json">
                    <div class="form-actions">
                        <button type="submit" class="btn">Save Digest Settings</button>
                    </div>
                </form>
            </div>
        </div>"#, base_url = base_url, id = html_escape(&calendar.id))
        },
        digest_script = {
            format!(r#"
            let digest = {digest_json};
            if (!digest.responders) digest.responders = [];

            function renderDigest() {{
                const freqEl = document.getElementById('digest-frequency');
                if (freqEl) freqEl.value = digest.frequency || 'none';
                renderDigestResponders();
            }}

            function renderDigestResponders() {{
                const list = document.getElementById('digest-responders-list');
                if (!list) return;
                list.innerHTML = digest.responders.map((r, i) => {{
                    const isEmail = r.channel === 'twilio_email' || r.channel === 'resend_email';
                    const targetPlaceholder = isEmail ? 'admin@example.com' : '+1234567890';
                    return `<div class="card" style="margin-bottom:0.5rem;padding:0.75rem;background:var(--bg-muted);">
                        <div style="display:flex;gap:0.5rem;align-items:center;flex-wrap:wrap;">
                            <input type="text" value="${{r.name||''}}" onchange="digest.responders[${{i}}].name=this.value" placeholder="Name" style="flex:1;min-width:100px;">
                            <select onchange="digest.responders[${{i}}].channel=this.value;renderDigestResponders();">
                                {js_channel_options}
                            </select>
                            <input type="${{isEmail ? 'email' : 'tel'}}" value="${{r.target_field||''}}" onchange="digest.responders[${{i}}].target_field=this.value" placeholder="${{targetPlaceholder}}" style="flex:1;min-width:150px;">
                            <label style="white-space:nowrap;"><input type="checkbox" ${{r.enabled?'checked':''}} onchange="digest.responders[${{i}}].enabled=this.checked" style="width:auto;"> On</label>
                            <button type="button" class="btn btn-sm btn-danger" onclick="removeDigestResponder(${{i}})">×</button>
                        </div>
                    </div>`;
                }}).join('');
            }}

            function addDigestResponder() {{
                digest.responders.push({{
                    id: 'digest_' + Date.now(),
                    name: 'Digest',
                    channel: '{default_channel}',
                    target_field: '',
                    subject: '',
                    body: '',
                    enabled: true,
                    use_ai: false
                }});
                renderDigestResponders();
            }}

            function removeDigestResponder(i) {{
                digest.responders.splice(i, 1);
                renderDigestResponders();
            }}

            function updateDigestField() {{
                document.getElementById('digest-json').value = JSON.stringify(digest);
            }}

            renderDigest();
            "#, digest_json = digest_json, js_channel_options = js_channel_options, default_channel = default_channel)
        },
    );

    let title = if calendar.archived {
        format!("{} (Archived)", calendar.name)
    } else {
        format!("Edit: {}", calendar.name)
    };
    base_html(&title, &content, &calendar.style)
}

pub fn admin_slots_html(calendar: &CalendarConfig, slots: &[TimeSlot], base_url: &str) -> String {
    let slot_rows: String = slots
        .iter()
        .map(|slot| {
            let day_display = slot
                .day_of_week
                .map(|d| day_name(d as u32).to_string())
                .or_else(|| slot.specific_date.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            format!(
                "<tr>
                    <td>{day}</td>
                    <td>{start} - {end}</td>
                    <td>{duration} min</td>
                    <td>{max}</td>
                    <td>
                        <button class=\"btn btn-sm btn-danger\"
                                hx-delete=\"{base_url}/admin/calendars/{cal_id}/slots/{slot_id}\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Delete</button>
                    </td>
                </tr>",
                base_url = base_url,
                cal_id = html_escape(&calendar.id),
                slot_id = html_escape(&slot.id),
                day = html_escape(&day_display),
                start = html_escape(&slot.start_time),
                end = html_escape(&slot.end_time),
                duration = slot.slot_duration,
                max = slot.max_bookings,
            )
        })
        .collect();

    let content = format!(
        "<p><a href=\"{base_url}/admin/calendars/{id}\">&larr; Back to {name}</a></p>
        <h1>Time Slots: {name}</h1>

        <div class=\"card\">
            <h2>Add Recurring Slot</h2>
            <form hx-post=\"{base_url}/admin/calendars/{id}/slots\" hx-target=\"{hash}slot-list tbody\" hx-swap=\"beforeend\" hx-on::after-request=\"this.reset()\">
                <div class=\"form-group\">
                    <label>Day of Week</label>
                    <select name=\"day_of_week\">
                        <option value=\"1\">Monday</option>
                        <option value=\"2\">Tuesday</option>
                        <option value=\"3\">Wednesday</option>
                        <option value=\"4\">Thursday</option>
                        <option value=\"5\">Friday</option>
                        <option value=\"6\">Saturday</option>
                        <option value=\"0\">Sunday</option>
                    </select>
                </div>
                <div class=\"form-group\">
                    <label>Start Time</label>
                    <input type=\"time\" name=\"start_time\" value=\"09:00\" required>
                </div>
                <div class=\"form-group\">
                    <label>End Time</label>
                    <input type=\"time\" name=\"end_time\" value=\"17:00\" required>
                </div>
                <div class=\"form-group\">
                    <label>Slot Duration (minutes)</label>
                    <input type=\"number\" name=\"slot_duration\" value=\"30\" min=\"5\" max=\"480\" required>
                </div>
                <div class=\"form-group\">
                    <label>Max Bookings per Slot</label>
                    <input type=\"number\" name=\"max_bookings\" value=\"1\" min=\"1\" required>
                </div>
                <button type=\"submit\" class=\"btn\">Add Slot</button>
            </form>
        </div>

        <div class=\"card\">
            <h2>Configured Slots</h2>
            <table id=\"slot-list\">
                <thead><tr><th>Day</th><th>Time Range</th><th>Slot Duration</th><th>Max Bookings</th><th>Actions</th></tr></thead>
                <tbody>{slot_rows}</tbody>
            </table>
        </div>",
        base_url = base_url,
        id = html_escape(&calendar.id),
        name = html_escape(&calendar.name),
        slot_rows = slot_rows,
        hash = HASH,
    );

    base_html(
        &format!("Slots: {}", calendar.name),
        &content,
        &calendar.style,
    )
}

pub fn admin_bookings_html(
    calendar: &CalendarConfig,
    bookings: &[Booking],
    base_url: &str,
) -> String {
    let booking_rows: String = bookings
        .iter()
        .map(|booking| {
            format!(
                "<tr>
                    <td>{date}</td>
                    <td>{time}</td>
                    <td>{name}</td>
                    <td>{email}</td>
                    <td>{status:?}</td>
                    <td>
                        <button class=\"btn btn-sm btn-danger\"
                                hx-post=\"{base_url}/admin/calendars/{cal_id}/bookings/{booking_id}/cancel\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Cancel</button>
                    </td>
                </tr>",
                base_url = base_url,
                cal_id = html_escape(&calendar.id),
                booking_id = html_escape(&booking.id),
                date = html_escape(&booking.slot_date),
                time = html_escape(&booking.slot_time),
                name = html_escape(&booking.name),
                email = html_escape(&booking.email),
                status = booking.status,
            )
        })
        .collect();

    let content = format!(
        "<p><a href=\"{base_url}/admin/calendars/{id}\">&larr; Back to {name}</a></p>
        <h1>Bookings: {name}</h1>

        <div class=\"card\">
            <table>
                <thead><tr><th>Date</th><th>Time</th><th>Name</th><th>Email</th><th>Status</th><th>Actions</th></tr></thead>
                <tbody>{booking_rows}</tbody>
            </table>
        </div>",
        base_url = base_url,
        id = html_escape(&calendar.id),
        name = html_escape(&calendar.name),
        booking_rows = booking_rows,
    );

    base_html(
        &format!("Bookings: {}", calendar.name),
        &content,
        &calendar.style,
    )
}

pub fn admin_booking_link_html(
    calendar: &CalendarConfig,
    link: &BookingLink,
    base_url: &str,
) -> String {
    let responders_json = serde_json::to_string(&link.responders).unwrap_or_else(|_| "[]".into());
    let admin_responders_json =
        serde_json::to_string(&link.admin_responders).unwrap_or_else(|_| "[]".into());
    let js_escape = |s: &str| {
        s.replace('\\', "\\\\")
            .replace('\'', "\\'")
            .replace('\n', "\\n")
    };

    // WhatsApp is the only supported channel
    let channel_options = [("whatsapp", "WhatsApp")];
    let has_channels = true;
    let default_channel = "whatsapp";
    let is_default_email = false;

    // Build JS channel options string
    let js_channel_options: String = channel_options
        .iter()
        .map(|(value, label)| {
            format!(
                "<option value=\"{}\" ${{r.channel==='{}'?'selected':''}}>{}</option>",
                value, value, label
            )
        })
        .collect::<Vec<_>>()
        .join("\\n                                ");

    let responders_section = if has_channels {
        r#"<h3 style="margin-top: 1.5rem; margin-bottom: 1rem;">Customer Notifications</h3>
                <p class="text-muted" style="margin-bottom: 1rem;">Send automatic confirmation messages to customers when bookings are confirmed. Use {{{{name}}}}, {{{{email}}}}, {{{{date}}}}, {{{{time}}}} placeholders.</p>
                <div id="responders-list"></div>
                <button type="button" class="btn btn-secondary" onclick="addResponder()" style="margin-bottom: 1rem;">+ Add Responder</button>
                <input type="hidden" name="responders_json" id="responders-json">"#.to_string()
    } else {
        String::from(
            r#"<input type="hidden" name="responders_json" id="responders-json" value="[]">"#,
        )
    };

    let admin_responders_section = if has_channels {
        r#"<h3 style="margin-top: 1.5rem; margin-bottom: 1rem;">Admin Notifications</h3>
                <p class="text-muted" style="margin-bottom: 1rem;">Receive notifications when bookings need approval (when auto-accept is disabled). Use {{{{name}}}}, {{{{email}}}}, {{{{date}}}}, {{{{time}}}}, {{{{approve_url}}}} placeholders.</p>
                <div id="admin-responders-list"></div>
                <button type="button" class="btn btn-secondary" onclick="addAdminResponder()" style="margin-bottom: 1rem;">+ Add Admin Responder</button>
                <input type="hidden" name="admin_responders_json" id="admin-responders-json">"#.to_string()
    } else {
        String::from(
            r#"<input type="hidden" name="admin_responders_json" id="admin-responders-json" value="[]">"#,
        )
    };

    let responders_script = if has_channels {
        format!(
            r#"<script>
            let responders = {responders_json};
            let adminResponders = {admin_responders_json};

            function renderResponderList(list, containerId, isAdmin) {{
                const container = document.getElementById(containerId);
                container.innerHTML = list.map((r, i) => {{
                    const isEmail = r.channel === 'twilio_email' || r.channel === 'resend_email';
                    const isSms = r.channel === 'twilio_sms' || r.channel === 'twilio_whatsapp';
                    const listName = isAdmin ? 'adminResponders' : 'responders';
                    const renderFn = isAdmin ? 'renderAdminResponders' : 'renderResponders';
                    const removeFn = isAdmin ? 'removeAdminResponder' : 'removeResponder';
                    const targetPlaceholder = isEmail ? 'admin@example.com' : '+1234567890';
                    const targetLabel = isEmail ? 'Email' : 'Phone';
                    return `<div class="card" style="margin-bottom:1rem;padding:1rem;background:var(--bg-muted);">
                        <div style="display:flex;gap:0.5rem;margin-bottom:0.5rem;align-items:center;flex-wrap:wrap;">
                            <input type="text" value="${{r.name||''}}" onchange="${{listName}}[${{i}}].name=this.value" placeholder="Responder Name" style="flex:1;min-width:150px;">
                            <select onchange="${{listName}}[${{i}}].channel=this.value;${{renderFn}}();">
                                {js_channel_options}
                            </select>
                            <label style="white-space:nowrap;"><input type="checkbox" ${{r.enabled?'checked':''}} onchange="${{listName}}[${{i}}].enabled=this.checked" style="width:auto;"> Enabled</label>
                            <button type="button" class="btn btn-sm btn-danger" onclick="${{removeFn}}(${{i}})">Delete</button>
                        </div>
                        ${{isAdmin ? `<div class="form-group" style="margin-bottom:0.5rem;">
                            <input type="${{isEmail ? 'email' : 'tel'}}" value="${{r.target_field||''}}" onchange="${{listName}}[${{i}}].target_field=this.value" placeholder="${{targetPlaceholder}}" style="width:100%;">
                            <small class="text-muted">Your ${{targetLabel.toLowerCase()}} address for notifications</small>
                        </div>` : ''}}
                        ${{isEmail ? `<div class="form-group" style="margin-bottom:0.5rem;">
                            <input type="text" value="${{r.subject||''}}" onchange="${{listName}}[${{i}}].subject=this.value" placeholder="Email Subject">
                        </div>` : ''}}
                        <div class="form-group" style="margin-bottom:0;">
                            <textarea rows="3" onchange="${{listName}}[${{i}}].body=this.value" placeholder="${{isAdmin ? 'Message body. Use {{{{{{{{name}}}}}}}}, {{{{{{{{email}}}}}}}}, {{{{{{{{date}}}}}}}}, {{{{{{{{time}}}}}}}}, {{{{{{{{approve_url}}}}}}}} placeholders.' : 'Message body. Use {{{{{{{{name}}}}}}}}, {{{{{{{{email}}}}}}}}, {{{{{{{{date}}}}}}}}, {{{{{{{{time}}}}}}}} placeholders.'}}">${{r.body||''}}</textarea>
                        </div>
                    </div>`;
                }}).join('');
            }}

            function renderResponders() {{
                renderResponderList(responders, 'responders-list', false);
            }}

            function renderAdminResponders() {{
                renderResponderList(adminResponders, 'admin-responders-list', true);
            }}

            function addResponder() {{
                responders.push({{
                    id: 'resp_' + Date.now(),
                    name: 'Booking Confirmation',
                    channel: '{default_channel}',
                    target_field: 'email',
                    subject: {default_subject},
                    body: 'Hi {{{{{{{{name}}}}}}}},\\n\\nYour booking for {{{{{{{{date}}}}}}}} at {{{{{{{{time}}}}}}}} has been confirmed.\\n\\nThank you!',
                    enabled: true,
                    use_ai: false
                }});
                renderResponders();
            }}

            function addAdminResponder() {{
                adminResponders.push({{
                    id: 'admin_' + Date.now(),
                    name: 'Approval Request',
                    channel: '{default_channel}',
                    target_field: '',
                    subject: {admin_default_subject},
                    body: 'New booking request from {{{{{{{{name}}}}}}}} ({{{{{{{{email}}}}}}}}) for {{{{{{{{date}}}}}}}} at {{{{{{{{time}}}}}}}}.\\n\\nApprove: {{{{{{{{approve_url}}}}}}}}',
                    enabled: true,
                    use_ai: false
                }});
                renderAdminResponders();
            }}

            function removeResponder(i) {{
                responders.splice(i, 1);
                renderResponders();
            }}

            function removeAdminResponder(i) {{
                adminResponders.splice(i, 1);
                renderAdminResponders();
            }}

            function updateRespondersField() {{
                document.getElementById('responders-json').value = JSON.stringify(responders);
                document.getElementById('admin-responders-json').value = JSON.stringify(adminResponders);
            }}

            renderResponders();
            renderAdminResponders();
        </script>"#,
            responders_json = js_escape(&responders_json),
            admin_responders_json = js_escape(&admin_responders_json),
            js_channel_options = js_channel_options,
            default_channel = default_channel,
            default_subject = if is_default_email {
                "'Your booking is confirmed'"
            } else {
                "''"
            },
            admin_default_subject = if is_default_email {
                "'New booking requires approval'"
            } else {
                "''"
            },
        )
    } else {
        String::from("<script>function updateRespondersField() { document.getElementById('responders-json').value = '[]'; document.getElementById('admin-responders-json').value = '[]'; }</script>")
    };

    let content = format!(
        "<p><a href=\"{base_url}/admin/calendars/{cal_id}\">&larr; Back to {cal_name}</a></p>
        <h1>Edit Booking Link: {name}</h1>

        <div class=\"card\">
            <p><strong>URL:</strong> <code>{base_url}/book/{cal_id}/{slug}</code></p>
        </div>

        <div class=\"card\">
            <form id=\"booking-link-form\" hx-put=\"{base_url}/admin/calendars/{cal_id}/booking/{link_id}\" hx-swap=\"none\" hx-on::before-request=\"updateRespondersField();this.querySelector('button[type=submit]').disabled=true;this.querySelector('button[type=submit]').textContent='Saving...'\" hx-on::after-request=\"this.querySelector('button[type=submit]').disabled=false;this.querySelector('button[type=submit]').textContent='Save Changes'\">
                <div class=\"form-group\">
                    <label>Name</label>
                    <input type=\"text\" name=\"name\" value=\"{name}\" required>
                </div>
                <div class=\"form-group\">
                    <label>Description</label>
                    <textarea name=\"description\" rows=\"2\">{description}</textarea>
                </div>
                <div class=\"form-group\">
                    <label>Duration (minutes)</label>
                    <input type=\"number\" name=\"duration\" value=\"{duration}\" min=\"5\" max=\"480\" required>
                </div>
                <div class=\"form-group\">
                    <label>Minimum Notice (hours)</label>
                    <input type=\"number\" name=\"min_notice\" value=\"{min_notice}\" min=\"0\" required>
                </div>
                <div class=\"form-group\">
                    <label>Maximum Advance Booking (days)</label>
                    <input type=\"number\" name=\"max_advance\" value=\"{max_advance}\" min=\"1\" required>
                </div>
                <div class=\"form-group\">
                    <label>Confirmation Message</label>
                    <textarea name=\"confirmation_message\" rows=\"2\">{confirmation}</textarea>
                </div>

                <h3 style=\"margin-top: 1.5rem; margin-bottom: 1rem;\">Display Settings</h3>
                <div class=\"form-group\">
                    <label style=\"display: flex; align-items: center; gap: 0.5rem; cursor: pointer;\">
                        <input type=\"checkbox\" name=\"hide_title\" {hide_title_checked} style=\"width: auto;\">
                        Hide title when embedded
                    </label>
                    <small class=\"text-muted\">Hide the booking link name when embedding in another page</small>
                </div>

                <h3 style=\"margin-top: 1.5rem; margin-bottom: 1rem;\">Approval Settings</h3>
                <div class=\"form-group\">
                    <label style=\"display: flex; align-items: center; gap: 0.5rem; cursor: pointer;\">
                        <input type=\"checkbox\" name=\"auto_accept\" {auto_accept_checked} style=\"width: auto;\" onchange=\"document.getElementById('admin-responders-section').style.display = this.checked ? 'none' : 'block'\">
                        Auto-accept bookings
                    </label>
                    <small class=\"text-muted\">When unchecked, bookings require manual approval</small>
                </div>
                <div id=\"admin-responders-section\" style=\"{admin_responders_display}\">
                    {admin_responders_section}
                </div>

                {responders_section}

                <div class=\"form-group\">
                    <label style=\"display: flex; align-items: center; gap: 0.5rem; cursor: pointer;\">
                        <input type=\"checkbox\" name=\"enabled\" {enabled_checked} style=\"width: auto;\">
                        Enabled
                    </label>
                </div>
                <div class=\"form-actions\">
                    <button type=\"submit\" class=\"btn\">Save Changes</button>
                </div>
            </form>
        </div>

        {responders_script}",
        base_url = base_url,
        cal_id = html_escape(&calendar.id),
        cal_name = html_escape(&calendar.name),
        link_id = html_escape(&link.id),
        slug = html_escape(&link.slug),
        name = html_escape(&link.name),
        description = html_escape(link.description.as_deref().unwrap_or("")),
        duration = link.duration,
        min_notice = link.min_notice,
        max_advance = link.max_advance,
        confirmation = html_escape(&link.confirmation_message),
        hide_title_checked = if link.hide_title { "checked" } else { "" },
        auto_accept_checked = if link.auto_accept { "checked" } else { "" },
        admin_responders_section = admin_responders_section,
        admin_responders_display = if link.auto_accept { "display: none;" } else { "" },
        enabled_checked = if link.enabled { "checked" } else { "" },
        responders_section = responders_section,
        responders_script = responders_script,
    );

    base_html(&format!("Edit: {}", link.name), &content, &calendar.style)
}

pub fn admin_view_link_html(calendar: &CalendarConfig, link: &ViewLink, base_url: &str) -> String {
    let content = format!(
        "<p><a href=\"{base_url}/admin/calendars/{cal_id}\">&larr; Back to {cal_name}</a></p>
        <h1>Edit View Link: {name}</h1>

        <div class=\"card\">
            <p><strong>URL:</strong> <code>{base_url}/view/{cal_id}/{slug}</code></p>
        </div>

        <div class=\"card\">
            <form hx-put=\"{base_url}/admin/calendars/{cal_id}/view/{link_id}\" hx-swap=\"none\" hx-on::before-request=\"this.querySelector('button[type=submit]').disabled=true;this.querySelector('button[type=submit]').textContent='Saving...'\" hx-on::after-request=\"this.querySelector('button[type=submit]').disabled=false;this.querySelector('button[type=submit]').textContent='Save Changes'\">
                <div class=\"form-group\">
                    <label>Name</label>
                    <input type=\"text\" name=\"name\" value=\"{name}\" required>
                </div>
                <div class=\"form-group\">
                    <label>View Type</label>
                    <select name=\"view_type\">
                        <option value=\"week\" {week_sel}>Week</option>
                        <option value=\"month\" {month_sel}>Month</option>
                        <option value=\"year\" {year_sel}>Year</option>
                        <option value=\"endless\" {endless_sel}>Endless</option>
                    </select>
                </div>
                <h3 style=\"margin-top: 1.5rem; margin-bottom: 1rem;\">Events</h3>
                <div class=\"form-group\">
                    <label>
                        <input type=\"checkbox\" name=\"show_events\" {events_checked}>
                        Show Events
                    </label>
                </div>
                <div class=\"form-group\">
                    <label>
                        <input type=\"checkbox\" name=\"show_event_details\" {event_details_checked}>
                        Show Event Details (titles, times)
                    </label>
                    <small style=\"display: block; margin-top: 0.25rem;\">When unchecked, shows \"busy\" instead of details</small>
                </div>

                <h3 style=\"margin-top: 1.5rem; margin-bottom: 1rem;\">Bookings</h3>
                <div class=\"form-group\">
                    <label>
                        <input type=\"checkbox\" name=\"show_bookings\" {bookings_checked}>
                        Show Bookings
                    </label>
                </div>
                <div class=\"form-group\">
                    <label>
                        <input type=\"checkbox\" name=\"show_booking_details\" {booking_details_checked}>
                        Show Booking Details (names, times)
                    </label>
                    <small style=\"display: block; margin-top: 0.25rem;\">When unchecked, shows \"busy\" instead of details</small>
                </div>
                <div class=\"form-group\">
                    <label>
                        <input type=\"checkbox\" name=\"enabled\" {enabled_checked}>
                        Enabled
                    </label>
                </div>
                <div class=\"form-actions\">
                    <button type=\"submit\" class=\"btn\">Save Changes</button>
                </div>
            </form>
        </div>",
        base_url = base_url,
        cal_id = html_escape(&calendar.id),
        cal_name = html_escape(&calendar.name),
        link_id = html_escape(&link.id),
        slug = html_escape(&link.slug),
        name = html_escape(&link.name),
        week_sel = if matches!(link.view_type, ViewType::Week) { "selected" } else { "" },
        month_sel = if matches!(link.view_type, ViewType::Month) { "selected" } else { "" },
        year_sel = if matches!(link.view_type, ViewType::Year) { "selected" } else { "" },
        endless_sel = if matches!(link.view_type, ViewType::Endless) { "selected" } else { "" },
        events_checked = if link.show_events { "checked" } else { "" },
        event_details_checked = if link.show_event_details { "checked" } else { "" },
        bookings_checked = if link.show_bookings { "checked" } else { "" },
        booking_details_checked = if link.show_booking_details { "checked" } else { "" },
        enabled_checked = if link.enabled { "checked" } else { "" },
    );

    base_html(&format!("Edit: {}", link.name), &content, &calendar.style)
}

pub fn admin_form_link_html(calendar: &CalendarConfig, link: &FormLink, base_url: &str) -> String {
    let embed_url = format!("{}/form/{}/{}", base_url, calendar.id, link.slug);
    let responses_url = format!(
        "{}/admin/calendars/{}/form/{}/responses",
        base_url, calendar.id, link.id
    );

    let content = format!(
        "<p><a href=\"{base_url}/admin/calendars/{id}\">&larr; Back to {cal_name}</a></p>
        <h1>Edit Form Link: {name}</h1>

        <div class=\"card\">
            <h2>Form Settings</h2>
            <form hx-put=\"{base_url}/admin/calendars/{id}/form/{link_id}\" hx-swap=\"none\"
                  hx-on::before-request=\"this.querySelector('button[type=submit]').disabled=true;this.querySelector('button[type=submit]').textContent='Saving...'\"
                  hx-on::after-request=\"this.querySelector('button[type=submit]').disabled=false;this.querySelector('button[type=submit]').textContent='Save'\">
                <div class=\"form-group\">
                    <label>Name</label>
                    <input type=\"text\" name=\"name\" value=\"{name}\" required>
                </div>
                <div class=\"form-group\">
                    <label>Google Form URL</label>
                    <input type=\"url\" name=\"google_form_url\" value=\"{google_form_url}\" placeholder=\"https://docs.google.com/forms/d/.../edit\" style=\"font-family: monospace;\" required>
                    <small class=\"text-muted\">Paste the Google Form URL. Share the form with your service account email to enable response viewing.</small>
                </div>
                <div class=\"form-group\">
                    <label><input type=\"checkbox\" name=\"enabled\" {enabled_checked} style=\"width: auto;\"> Enabled</label>
                </div>
                <div style=\"display: flex; justify-content: flex-end; gap: 0.5rem;\">
                    <button type=\"submit\" class=\"btn\">Save</button>
                </div>
            </form>
        </div>

        <div class=\"card\">
            <h2>Embed URL</h2>
            <div class=\"url-cell\">
                <code>{embed_url}</code>
                <button class=\"btn btn-copy\" onclick=\"copyUrl(this, '{embed_url}')\">Copy</button>
            </div>
            <p class=\"text-muted\" style=\"margin-top: 0.5rem;\">Use this URL to embed the form via iframe or HTMX.</p>
        </div>

        <div class=\"card\">
            <h2>Responses</h2>
            <p><a href=\"{responses_url}\" class=\"btn\">View Responses</a></p>
        </div>",
        base_url = base_url,
        id = html_escape(&calendar.id),
        cal_name = html_escape(&calendar.name),
        link_id = html_escape(&link.id),
        name = html_escape(&link.name),
        google_form_url = html_escape(&link.google_form_url),
        enabled_checked = if link.enabled { "checked" } else { "" },
        embed_url = html_escape(&embed_url),
        responses_url = html_escape(&responses_url),
    );

    base_html(&format!("Form: {}", link.name), &content, &calendar.style)
}

pub fn admin_form_responses_html(
    calendar: &CalendarConfig,
    link: &FormLink,
    form: &crate::google_forms::GoogleForm,
    responses: &[crate::google_forms::FormResponse],
    base_url: &str,
) -> String {
    // Build question ID -> title map from form structure
    let question_ids: Vec<String> = form
        .items
        .iter()
        .filter_map(|item| {
            let qi = item.question_item.as_ref()?;
            Some(qi.question.question_id.clone())
        })
        .collect();

    let headers: Vec<String> = form
        .items
        .iter()
        .filter_map(|item| {
            item.question_item.as_ref()?;
            item.title.clone()
        })
        .collect();

    let header_html: String = headers
        .iter()
        .map(|h| format!("<th>{}</th>", html_escape(h)))
        .collect();

    let rows: String = responses
        .iter()
        .rev()
        .map(|resp| {
            let timestamp = resp
                .create_time
                .as_deref()
                .unwrap_or("")
                .split('T')
                .next()
                .unwrap_or("");

            let cells: String = question_ids
                .iter()
                .map(|qid| {
                    let value = resp
                        .answers
                        .get(qid)
                        .and_then(|a| a.text_answers.as_ref())
                        .and_then(|ta| ta.answers.first())
                        .map(|a| a.value.as_str())
                        .unwrap_or("");
                    format!("<td>{}</td>", html_escape(value))
                })
                .collect();

            format!(
                "<tr><td>{}</td><td style=\"font-family: monospace; font-size: 0.8rem;\">{}</td>{}</tr>",
                html_escape(timestamp),
                html_escape(&resp.response_id),
                cells,
            )
        })
        .collect();

    let form_description = form
        .info
        .description
        .as_deref()
        .filter(|d| !d.is_empty())
        .map(|d| format!("<p class=\"text-muted\">{}</p>", html_escape(d)))
        .unwrap_or_default();

    let content = format!(
        "<p><a href=\"{base_url}/admin/calendars/{id}/form/{link_id}\">&larr; Back to {link_name}</a></p>
        <h1>Responses: {form_title}</h1>
        {form_description}
        <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">{count} response(s) &middot; Form ID: <code>{form_id}</code></p>

        <div class=\"card\" style=\"overflow-x: auto;\">
            <table>
                <thead><tr><th>Date</th><th>Response ID</th>{header_html}</tr></thead>
                <tbody>{rows}</tbody>
            </table>
        </div>",
        base_url = base_url,
        id = html_escape(&calendar.id),
        link_id = html_escape(&link.id),
        link_name = html_escape(&link.name),
        form_title = html_escape(&form.info.title),
        form_description = form_description,
        form_id = html_escape(&form.form_id),
        count = responses.len(),
        header_html = header_html,
        rows = rows,
    );

    base_html(
        &format!("Responses: {}", form.info.title),
        &content,
        &calendar.style,
    )
}

// ============================================================================
// WhatsApp Account Templates
// ============================================================================

pub fn admin_whatsapp_list_html(accounts: &[WhatsAppAccount], base_url: &str) -> String {
    let style = CalendarStyle::default();
    let rows: String = accounts
        .iter()
        .map(|a| {
            format!(
                "<tr>
                    <td><a href=\"{base_url}/admin/whatsapp/{id}\">{name}</a></td>
                    <td>{phone}</td>
                    <td>{auto_reply}</td>
                    <td>
                        <a href=\"{base_url}/admin/whatsapp/{id}\" class=\"btn btn-sm\">Edit</a>
                        <button class=\"btn btn-sm btn-danger\"
                                hx-delete=\"{base_url}/admin/whatsapp/{id}\"
                                hx-confirm=\"Delete this WhatsApp account?\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Delete</button>
                    </td>
                </tr>",
                base_url = base_url,
                id = html_escape(&a.id),
                name = html_escape(&a.name),
                phone = html_escape(&a.phone_number),
                auto_reply = if a.auto_reply.enabled { "On" } else { "Off" },
            )
        })
        .collect();

    let content = format!(
        "<p><a href=\"{base_url}/admin\">&larr; Back to Dashboard</a></p>
        <h1>WhatsApp Accounts</h1>
        <p style=\"margin: 1rem 0;\">
            <a href=\"{base_url}/admin/whatsapp\" class=\"btn\"
               hx-post=\"{base_url}/admin/whatsapp\" hx-boost=\"false\">+ New WhatsApp Account</a>
        </p>
        <div class=\"card\">
            <table>
                <thead><tr><th>Name</th><th>Phone</th><th>Auto-Reply</th><th>Actions</th></tr></thead>
                <tbody>{rows}</tbody>
            </table>
        </div>",
        base_url = base_url,
        rows = if rows.is_empty() {
            "<tr><td colspan=\"4\" class=\"text-muted\" style=\"text-align:center;\">No WhatsApp accounts yet.</td></tr>".to_string()
        } else {
            rows
        },
    );

    base_html("WhatsApp Accounts - Concierge", &content, &style)
}

pub fn admin_whatsapp_edit_html(
    account: &WhatsAppAccount,
    has_credentials: bool,
    base_url: &str,
) -> String {
    let style = CalendarStyle::default();
    let content = format!(
        "<p><a href=\"{base_url}/admin/whatsapp\">&larr; Back to WhatsApp Accounts</a></p>
        <h1>Edit: {name}</h1>

        <div class=\"card\">
            <form hx-put=\"{base_url}/admin/whatsapp/{id}\" hx-swap=\"none\"
                  hx-on::before-request=\"this.querySelector('button[type=submit]').disabled=true\"
                  hx-on::after-request=\"this.querySelector('button[type=submit]').disabled=false\">
                <div class=\"form-group\">
                    <label>Account Name</label>
                    <input type=\"text\" name=\"name\" value=\"{name}\" required>
                </div>
                <div class=\"form-group\">
                    <label>Display Phone Number</label>
                    <input type=\"tel\" name=\"phone_number\" value=\"{phone}\" placeholder=\"+1 234 567 8900\">
                    <small class=\"text-muted\">The phone number shown to users (display only)</small>
                </div>

                <h2 style=\"margin-top: 1.5rem;\">API Credentials</h2>
                <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">
                    {cred_status}
                </p>
                <div class=\"form-group\">
                    <label>Access Token</label>
                    <input type=\"text\" name=\"access_token\" value=\"\" placeholder=\"{token_placeholder}\" style=\"font-family: monospace;\">
                    <small class=\"text-muted\">Leave blank to keep existing credentials</small>
                </div>
                <div class=\"form-group\">
                    <label>Phone Number ID</label>
                    <input type=\"text\" name=\"phone_number_id\" value=\"\" placeholder=\"{phone_id_placeholder}\" style=\"font-family: monospace;\">
                </div>

                <h2 style=\"margin-top: 1.5rem;\">Auto-Reply</h2>
                <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">Automatically reply to incoming WhatsApp messages.</p>
                <div class=\"form-group\">
                    <label><input type=\"checkbox\" name=\"auto_reply_enabled\" {ar_enabled} style=\"width: auto;\"> Enable auto-reply</label>
                </div>
                <div class=\"form-group\">
                    <label>Mode</label>
                    <select name=\"auto_reply_mode\">
                        <option value=\"static\" {static_sel}>Static message</option>
                        <option value=\"ai\" {ai_sel}>AI-generated reply</option>
                    </select>
                </div>
                <div class=\"form-group\">
                    <label>Message / AI Prompt</label>
                    <textarea name=\"auto_reply_prompt\" rows=\"4\" placeholder=\"Enter a static reply message or an AI system prompt...\">{prompt}</textarea>
                    <small class=\"text-muted\">For static mode: the exact message to send. For AI mode: the system prompt for generating replies.</small>
                </div>

                <div style=\"display: flex; justify-content: flex-end; margin-top: 1rem;\">
                    <button type=\"submit\" class=\"btn\">Save</button>
                </div>
            </form>
        </div>",
        base_url = base_url,
        id = html_escape(&account.id),
        name = html_escape(&account.name),
        phone = html_escape(&account.phone_number),
        cred_status = if has_credentials { "Credentials configured." } else { "No credentials set." },
        token_placeholder = if has_credentials { "••••••••" } else { "EAAx..." },
        phone_id_placeholder = if has_credentials { "••••••••" } else { "1234567890" },
        ar_enabled = if account.auto_reply.enabled { "checked" } else { "" },
        static_sel = if account.auto_reply.mode == AutoReplyMode::Static { "selected" } else { "" },
        ai_sel = if account.auto_reply.mode == AutoReplyMode::Ai { "selected" } else { "" },
        prompt = html_escape(&account.auto_reply.prompt),
    );

    base_html(&format!("Edit: {}", account.name), &content, &style)
}

// ============================================================================
// Google Form Resource Templates
// ============================================================================

pub fn admin_forms_list_html(forms: &[GoogleFormResource], base_url: &str) -> String {
    let style = CalendarStyle::default();
    let rows: String = forms
        .iter()
        .map(|f| {
            let embed_url = format!("{}/form/{}/{}", base_url, f.id, f.slug);
            format!(
                "<tr>
                    <td><a href=\"{base_url}/admin/forms/{id}\">{name}</a></td>
                    <td><div class=\"url-cell\"><code>{embed_url}</code></div></td>
                    <td>{status}</td>
                    <td>
                        <a href=\"{base_url}/admin/forms/{id}\" class=\"btn btn-sm\">Edit</a>
                        <button class=\"btn btn-sm btn-danger\"
                                hx-delete=\"{base_url}/admin/forms/{id}\"
                                hx-confirm=\"Delete this form?\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Delete</button>
                    </td>
                </tr>",
                base_url = base_url,
                id = html_escape(&f.id),
                name = html_escape(&f.name),
                embed_url = html_escape(&embed_url),
                status = if f.enabled { "Enabled" } else { "Disabled" },
            )
        })
        .collect();

    let content = format!(
        "<p><a href=\"{base_url}/admin\">&larr; Back to Dashboard</a></p>
        <h1>Google Forms</h1>
        <p style=\"margin: 1rem 0;\">
            <a href=\"{base_url}/admin/forms\" class=\"btn\"
               hx-post=\"{base_url}/admin/forms\" hx-boost=\"false\">+ New Form</a>
        </p>
        <div class=\"card\">
            <table>
                <thead><tr><th>Name</th><th>URL</th><th>Status</th><th>Actions</th></tr></thead>
                <tbody>{rows}</tbody>
            </table>
        </div>",
        base_url = base_url,
        rows = if rows.is_empty() {
            "<tr><td colspan=\"4\" class=\"text-muted\" style=\"text-align:center;\">No forms yet.</td></tr>".to_string()
        } else {
            rows
        },
    );

    base_html("Google Forms - Concierge", &content, &style)
}

pub fn admin_form_edit_html(
    form: &GoogleFormResource,
    whatsapp_accounts: &[WhatsAppAccount],
    base_url: &str,
) -> String {
    let style = CalendarStyle::default();
    let embed_url = format!("{}/form/{}/{}", base_url, form.id, form.slug);

    let wa_options: String = std::iter::once("<option value=\"\">None</option>".to_string())
        .chain(whatsapp_accounts.iter().map(|a| {
            let selected = form
                .whatsapp_account_id
                .as_ref()
                .map(|id| id == &a.id)
                .unwrap_or(false);
            format!(
                "<option value=\"{}\" {}>{}</option>",
                html_escape(&a.id),
                if selected { "selected" } else { "" },
                html_escape(&a.name)
            )
        }))
        .collect();

    let content = format!(
        "<p><a href=\"{base_url}/admin/forms\">&larr; Back to Forms</a></p>
        <h1>Edit: {name}</h1>

        <div class=\"card\">
            <h2>Embed URL</h2>
            <div class=\"url-cell\">
                <code>{embed_url}</code>
            </div>
        </div>

        <div class=\"card\">
            <h2>Form Settings</h2>
            <form hx-put=\"{base_url}/admin/forms/{id}\" hx-swap=\"none\"
                  hx-on::before-request=\"this.querySelector('button[type=submit]').disabled=true\"
                  hx-on::after-request=\"this.querySelector('button[type=submit]').disabled=false\">
                <div class=\"form-group\">
                    <label>Name</label>
                    <input type=\"text\" name=\"name\" value=\"{name}\" required>
                </div>
                <div class=\"form-group\">
                    <label>Google Form URL</label>
                    <input type=\"url\" name=\"google_form_url\" value=\"{google_form_url}\" placeholder=\"https://docs.google.com/forms/d/.../edit\" style=\"font-family: monospace;\">
                </div>
                <div class=\"form-group\">
                    <label><input type=\"checkbox\" name=\"enabled\" {enabled_checked} style=\"width: auto;\"> Enabled</label>
                </div>

                <h3 style=\"margin-top: 1.5rem;\">Form &rarr; WhatsApp Automation</h3>
                <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">Send a WhatsApp message when new form responses are submitted.</p>
                <div class=\"form-group\">
                    <label>WhatsApp Account</label>
                    <select name=\"whatsapp_account_id\">{wa_options}</select>
                </div>
                <div class=\"form-group\">
                    <label>Phone Number Field</label>
                    <input type=\"text\" name=\"phone_field\" value=\"{phone_field}\" placeholder=\"Phone Number\">
                    <small class=\"text-muted\">The form question title that contains the respondent's phone number</small>
                </div>
                <div class=\"form-group\">
                    <label>Reply Message / AI Prompt</label>
                    <textarea name=\"reply_prompt\" rows=\"3\" placeholder=\"Thank you for your submission, we'll be in touch!\">{reply_prompt}</textarea>
                </div>
                <div class=\"form-group\">
                    <label><input type=\"checkbox\" name=\"use_ai\" {use_ai_checked} style=\"width: auto;\"> Use AI to generate reply</label>
                </div>

                <div style=\"display: flex; justify-content: flex-end; margin-top: 1rem;\">
                    <button type=\"submit\" class=\"btn\">Save</button>
                </div>
            </form>
        </div>

        <div class=\"card\">
            <h2>Responses</h2>
            <p><a href=\"{base_url}/admin/forms/{id}/responses\" class=\"btn\">View Responses</a></p>
        </div>",
        base_url = base_url,
        id = html_escape(&form.id),
        name = html_escape(&form.name),
        embed_url = html_escape(&embed_url),
        google_form_url = html_escape(&form.google_form_url),
        enabled_checked = if form.enabled { "checked" } else { "" },
        wa_options = wa_options,
        phone_field = html_escape(&form.phone_field),
        reply_prompt = html_escape(&form.reply_prompt),
        use_ai_checked = if form.use_ai { "checked" } else { "" },
    );

    base_html(&format!("Edit: {}", form.name), &content, &style)
}

pub fn admin_form_resource_responses_html(
    form: &GoogleFormResource,
    gform: &crate::google_forms::GoogleForm,
    responses: &[crate::google_forms::FormResponse],
    base_url: &str,
) -> String {
    let style = CalendarStyle::default();

    let question_ids: Vec<String> = gform
        .items
        .iter()
        .filter_map(|item| {
            let qi = item.question_item.as_ref()?;
            Some(qi.question.question_id.clone())
        })
        .collect();

    let headers: Vec<String> = gform
        .items
        .iter()
        .filter_map(|item| {
            item.question_item.as_ref()?;
            item.title.clone()
        })
        .collect();

    let header_html: String = headers
        .iter()
        .map(|h| format!("<th>{}</th>", html_escape(h)))
        .collect();

    let rows: String = responses
        .iter()
        .rev()
        .map(|resp| {
            let timestamp = resp
                .create_time
                .as_deref()
                .unwrap_or("")
                .split('T')
                .next()
                .unwrap_or("");

            let cells: String = question_ids
                .iter()
                .map(|qid| {
                    let value = resp
                        .answers
                        .get(qid)
                        .and_then(|a| a.text_answers.as_ref())
                        .and_then(|ta| ta.answers.first())
                        .map(|a| a.value.as_str())
                        .unwrap_or("");
                    format!("<td>{}</td>", html_escape(value))
                })
                .collect();

            format!("<tr><td>{}</td>{}</tr>", html_escape(timestamp), cells,)
        })
        .collect();

    let content = format!(
        "<p><a href=\"{base_url}/admin/forms/{id}\">&larr; Back to {name}</a></p>
        <h1>Responses: {form_title}</h1>
        <p class=\"text-muted\" style=\"margin-bottom: 1rem;\">{count} response(s)</p>

        <div class=\"card\" style=\"overflow-x: auto;\">
            <table>
                <thead><tr><th>Date</th>{header_html}</tr></thead>
                <tbody>{rows}</tbody>
            </table>
        </div>",
        base_url = base_url,
        id = html_escape(&form.id),
        name = html_escape(&form.name),
        form_title = html_escape(&gform.info.title),
        count = responses.len(),
        header_html = header_html,
        rows = rows,
    );

    base_html(
        &format!("Responses: {}", gform.info.title),
        &content,
        &style,
    )
}

// ============================================================================
// Instagram Account Templates
// ============================================================================

pub fn admin_instagram_list_html(
    accounts: &[InstagramAccount],
    calendars: &[CalendarConfig],
    base_url: &str,
) -> String {
    let style = CalendarStyle::default();
    let rows: String = accounts
        .iter()
        .map(|a| {
            let target = a
                .target_calendar_id
                .as_ref()
                .and_then(|id| calendars.iter().find(|c| &c.id == id))
                .map(|c| c.name.as_str())
                .unwrap_or("Not set");
            format!(
                "<tr>
                    <td><a href=\"{base_url}/admin/instagram/{id}\">@{username}</a></td>
                    <td>{target}</td>
                    <td>{status}</td>
                    <td>
                        <a href=\"{base_url}/admin/instagram/{id}\" class=\"btn btn-sm\">Edit</a>
                        <button class=\"btn btn-sm btn-danger\"
                                hx-delete=\"{base_url}/admin/instagram/{id}\"
                                hx-confirm=\"Disconnect this Instagram account?\"
                                hx-target=\"closest tr\"
                                hx-swap=\"outerHTML\">Disconnect</button>
                    </td>
                </tr>",
                base_url = base_url,
                id = html_escape(&a.id),
                username = html_escape(&a.instagram_username),
                target = html_escape(target),
                status = if a.enabled { "Enabled" } else { "Disabled" },
            )
        })
        .collect();

    // Extract tenant_id from first account or first calendar for the connect link
    let tenant_id = accounts
        .first()
        .map(|a| a.tenant_id.as_str())
        .or_else(|| calendars.first().map(|c| c.tenant_id.as_str()))
        .unwrap_or("");

    let content = format!(
        "<p><a href=\"{base_url}/admin\">&larr; Back to Dashboard</a></p>
        <h1>Instagram Accounts</h1>
        <p style=\"margin: 1rem 0;\">
            <a href=\"{base_url}/instagram/auth/{tenant_id}\" class=\"btn\">Connect Instagram Account</a>
        </p>
        <div class=\"card\">
            <table>
                <thead><tr><th>Account</th><th>Target Calendar</th><th>Status</th><th>Actions</th></tr></thead>
                <tbody>{rows}</tbody>
            </table>
        </div>",
        base_url = base_url,
        tenant_id = html_escape(tenant_id),
        rows = if rows.is_empty() {
            "<tr><td colspan=\"4\" class=\"text-muted\" style=\"text-align:center;\">No Instagram accounts connected.</td></tr>".to_string()
        } else {
            rows
        },
    );

    base_html("Instagram Accounts - Concierge", &content, &style)
}

pub fn admin_instagram_edit_html(
    account: &InstagramAccount,
    calendars: &[CalendarConfig],
    base_url: &str,
) -> String {
    let style = CalendarStyle::default();

    let cal_options: String = std::iter::once("<option value=\"\">None</option>".to_string())
        .chain(calendars.iter().filter(|c| !c.archived).map(|c| {
            let selected = account
                .target_calendar_id
                .as_ref()
                .map(|id| id == &c.id)
                .unwrap_or(false);
            format!(
                "<option value=\"{}\" {}>{}</option>",
                html_escape(&c.id),
                if selected { "selected" } else { "" },
                html_escape(&c.name)
            )
        }))
        .collect();

    let content = format!(
        "<p><a href=\"{base_url}/admin/instagram\">&larr; Back to Instagram Accounts</a></p>
        <h1>@{username}</h1>

        <div class=\"card\">
            <form hx-put=\"{base_url}/admin/instagram/{id}\" hx-swap=\"none\"
                  hx-on::before-request=\"this.querySelector('button[type=submit]').disabled=true\"
                  hx-on::after-request=\"this.querySelector('button[type=submit]').disabled=false\">
                <div class=\"form-group\">
                    <label>Target Calendar</label>
                    <select name=\"target_calendar_id\">{cal_options}</select>
                    <small class=\"text-muted\">Events extracted from posts will be added to this calendar</small>
                </div>
                <div class=\"form-group\">
                    <label>Classification Prompt (optional)</label>
                    <textarea name=\"classification_prompt\" rows=\"3\" placeholder=\"Custom AI prompt for classifying posts...\">{prompt}</textarea>
                    <small class=\"text-muted\">Override the default event extraction prompt</small>
                </div>
                <div class=\"form-group\">
                    <label><input type=\"checkbox\" name=\"enabled\" {enabled_checked} style=\"width: auto;\"> Enabled</label>
                </div>
                <div style=\"display: flex; justify-content: flex-end; margin-top: 1rem;\">
                    <button type=\"submit\" class=\"btn\">Save</button>
                </div>
            </form>
        </div>",
        base_url = base_url,
        id = html_escape(&account.id),
        username = html_escape(&account.instagram_username),
        cal_options = cal_options,
        prompt = html_escape(account.classification_prompt.as_deref().unwrap_or("")),
        enabled_checked = if account.enabled { "checked" } else { "" },
    );

    base_html(
        &format!("@{}", account.instagram_username),
        &content,
        &style,
    )
}
