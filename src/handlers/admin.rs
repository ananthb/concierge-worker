//! Admin handlers for forms and calendars

use worker::*;

use super::get_base_url;
use crate::helpers::*;
use crate::storage::*;
use crate::templates::*;
use crate::types::*;

/// Unified admin handler - session-protected
pub async fn handle_admin(
    mut req: Request,
    env: Env,
    path: &str,
    method: Method,
) -> Result<Response> {
    let kv = env.kv("CALENDARS_KV")?;

    // Resolve tenant from session cookie
    let tenant_id = match super::auth::resolve_tenant_id(&req, &kv).await {
        Some(id) => id,
        None => {
            // Fallback: Cloudflare Access header (for superadmin/legacy)
            let access_user = req
                .headers()
                .get("Cf-Access-Authenticated-User-Email")
                .ok()
                .flatten();

            let is_dev = env
                .var("ENVIRONMENT")
                .map(|v| v.to_string() == "development")
                .unwrap_or(false);

            match access_user {
                Some(email) => email,
                None if is_dev => "default".to_string(),
                None => {
                    // Not authenticated — redirect to login
                    let headers = Headers::new();
                    headers.set("Location", "/auth/login")?;
                    return Ok(Response::empty()?.with_status(302).with_headers(headers));
                }
            }
        }
    };

    let base_url = get_base_url(&req);

    if path == "/admin/settings" && method == Method::Get {
        let encryption_key = env
            .secret("ENCRYPTION_KEY")
            .map(|s| s.to_string())
            .unwrap_or_default();
        let creds = get_tenant_credentials(&kv, &tenant_id, &encryption_key)
            .await
            .unwrap_or_default();
        return Response::from_html(admin_settings_html(&creds, &base_url));
    }

    if path == "/admin/settings" && method == Method::Put {
        let form = req.form_data().await?;
        let encryption_key = env.secret("ENCRYPTION_KEY")?.to_string();
        // Preserve existing creds (including legacy WhatsApp fields)
        let mut creds = get_tenant_credentials(&kv, &tenant_id, &encryption_key)
            .await
            .unwrap_or_default();
        if let Some(FormEntry::Field(v)) = form.get("google_service_account_email") {
            creds.google_service_account_email = if v.is_empty() { None } else { Some(v) };
        }
        if let Some(FormEntry::Field(v)) = form.get("google_private_key") {
            creds.google_private_key = if v.is_empty() { None } else { Some(v) };
        }
        save_tenant_credentials(&kv, &tenant_id, &creds, &encryption_key).await?;
        return Response::from_html(calendar_success_html("Credentials saved"));
    }

    if path.starts_with("/admin/whatsapp") {
        return super::admin_whatsapp::handle_whatsapp_admin(
            req, env, path, method, &base_url, &tenant_id,
        )
        .await;
    }

    if path.starts_with("/admin/forms") {
        return super::admin_forms::handle_forms_admin(
            req, env, path, method, &base_url, &tenant_id,
        )
        .await;
    }

    if path.starts_with("/admin/instagram") {
        return super::admin_instagram::handle_instagram_admin(
            req, env, path, method, &base_url, &tenant_id,
        )
        .await;
    }

    if path.starts_with("/admin/calendars") {
        return handle_calendars_admin(req, env, path, method, &base_url, &tenant_id).await;
    }

    if path.starts_with("/admin/migrate") {
        return handle_migrate(req, env, method, &tenant_id).await;
    }

    if path == "/admin" || path == "/admin/" {
        let calendars_kv = env.kv("CALENDARS_KV")?;
        let calendars = list_calendars(&calendars_kv, &tenant_id).await?;
        let whatsapp_accounts = list_whatsapp_accounts(&calendars_kv, &tenant_id).await?;
        let form_resources = list_form_resources(&calendars_kv, &tenant_id).await?;
        let instagram_accounts = list_instagram_accounts(&calendars_kv, &tenant_id).await?;

        let mut resp = Response::from_html(admin_dashboard_html(
            &calendars,
            &whatsapp_accounts,
            &form_resources,
            &instagram_accounts,
            &base_url,
        ))?;
        resp.headers_mut().set("Cache-Control", "no-store")?;
        return Ok(resp);
    }

    Response::error("Not Found", 404)
}

async fn handle_calendars_admin(
    mut req: Request,
    env: Env,
    path: &str,
    method: Method,
    base_url: &str,
    tenant_id: &str,
) -> Result<Response> {
    let kv = env.kv("CALENDARS_KV")?;
    let db = env.d1("DB")?;

    let path_parts: Vec<&str> = path
        .strip_prefix("/admin/calendars")
        .unwrap_or("")
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    match (method, path_parts.as_slice()) {
        (Method::Post, []) => {
            let id = generate_id();
            let now = now_iso();
            let calendar = CalendarConfig {
                id: id.clone(),
                name: String::from("New Calendar"),
                description: None,
                timezone: String::from("UTC"),
                booking_links: Vec::new(),
                view_links: Vec::new(),
                google_calendar_id: None,
                form_links: Vec::new(),
                tenant_id: tenant_id.to_string(),
                instagram_sources: Vec::new(),
                style: CalendarStyle::default(),
                allowed_origins: Vec::new(),
                digest: DigestConfig::default(),
                archived: false,
                created_at: now.clone(),
                updated_at: now,
            };
            save_calendar(&kv, &calendar).await?;

            Response::from_html(format!(
                r#"<tr>
                    <td><a href="{base_url}/admin/calendars/{id}">New Calendar</a></td>
                    <td>0 booking links</td>
                    <td>0 view links</td>
                    <td>{date}</td>
                    <td>
                        <a href="{base_url}/admin/calendars/{id}" class="btn btn-sm">Edit</a>
                        <button class="btn btn-sm btn-secondary"
                                hx-post="{base_url}/admin/calendars/{id}/archive"
                                hx-confirm="Archive this calendar? It will become read-only."
                                hx-target="closest tr"
                                hx-swap="outerHTML">Archive</button>
                    </td>
                </tr>"#,
                base_url = base_url,
                id = id,
                date = calendar.updated_at.split('T').next().unwrap_or(""),
            ))
        }

        (Method::Get, [id]) => {
            let calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            if !calendar.tenant_id.is_empty() && calendar.tenant_id != tenant_id {
                return Response::error("Not found", 404);
            }
            let time_slots = get_time_slots(&db, id).await?;
            Response::from_html(admin_calendar_html(&calendar, &time_slots, base_url))
        }

        (Method::Put, [id]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };

            let form = req.form_data().await?;
            if let Some(FormEntry::Field(name)) = form.get("name") {
                calendar.name = name;
            }
            if let Some(FormEntry::Field(desc)) = form.get("description") {
                calendar.description = if desc.is_empty() { None } else { Some(desc) };
            }
            if let Some(FormEntry::Field(tz)) = form.get("timezone") {
                calendar.timezone = tz;
            }
            if let Some(FormEntry::Field(origins)) = form.get("allowed_origins") {
                calendar.allowed_origins = origins
                    .lines()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            if let Some(FormEntry::Field(css)) = form.get("custom_css") {
                calendar.style.custom_css = css;
            }
            if let Some(FormEntry::Field(gcal_id)) = form.get("google_calendar_id") {
                calendar.google_calendar_id = if gcal_id.is_empty() {
                    None
                } else {
                    Some(gcal_id)
                };
            }
            calendar.updated_at = now_iso();

            save_calendar(&kv, &calendar).await?;
            Response::from_html(calendar_success_html("Calendar updated"))
        }

        (Method::Delete, [id]) => {
            delete_calendar(&kv, tenant_id, id).await?;
            Response::empty()
        }

        (Method::Post, [id, "archive"]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            calendar.archived = true;
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;
            Response::empty()
        }

        (Method::Post, [id, "unarchive"]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            calendar.archived = false;
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;
            Response::empty()
        }

        (Method::Put, [id, "digest"]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };

            let form = req.form_data().await?;
            if let Some(FormEntry::Field(digest_json)) = form.get("digest_json") {
                if let Ok(digest) = serde_json::from_str(&digest_json) {
                    calendar.digest = digest;
                }
            }
            calendar.updated_at = now_iso();

            save_calendar(&kv, &calendar).await?;
            Response::from_html(calendar_success_html("Digest settings updated"))
        }

        (Method::Get, [id, "slots"]) => {
            let calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            let slots = get_time_slots(&db, id).await?;
            Response::from_html(admin_slots_html(&calendar, &slots, base_url))
        }

        (Method::Post, [id, "slots"]) => {
            let form = req.form_data().await?;
            let day_of_week = match form.get("day_of_week") {
                Some(FormEntry::Field(d)) => d.parse().ok(),
                _ => None,
            };
            let start_time = match form.get("start_time") {
                Some(FormEntry::Field(t)) => t,
                _ => return Response::error("Start time required", 400),
            };
            let end_time = match form.get("end_time") {
                Some(FormEntry::Field(t)) => t,
                _ => return Response::error("End time required", 400),
            };
            let slot_duration = match form.get("slot_duration") {
                Some(FormEntry::Field(d)) => d.parse().unwrap_or(30),
                _ => 30,
            };
            let max_bookings = match form.get("max_bookings") {
                Some(FormEntry::Field(m)) => m.parse().unwrap_or(1),
                _ => 1,
            };

            let slot = TimeSlot {
                id: generate_id(),
                calendar_id: id.to_string(),
                day_of_week,
                specific_date: None,
                start_time: start_time.clone(),
                end_time: end_time.clone(),
                slot_duration,
                buffer_time: 0,
                max_bookings,
            };

            save_time_slot(&db, &slot, tenant_id).await?;

            let day_display = day_of_week
                .map(|d| day_name(d as u32).to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            Response::from_html(format!(
                r#"<tr>
                    <td>{day}</td>
                    <td>{start} - {end}</td>
                    <td>{duration} min</td>
                    <td>{max}</td>
                    <td>
                        <button class="btn btn-sm btn-danger"
                                hx-delete="{base_url}/admin/calendars/{cal_id}/slots/{slot_id}"
                                hx-target="closest tr"
                                hx-swap="outerHTML">Delete</button>
                    </td>
                </tr>"#,
                base_url = base_url,
                cal_id = id,
                slot_id = slot.id,
                day = html_escape(&day_display),
                start = html_escape(&slot.start_time),
                end = html_escape(&slot.end_time),
                duration = slot.slot_duration,
                max = slot.max_bookings,
            ))
        }

        (Method::Delete, [_id, "slots", slot_id]) => {
            delete_time_slot(&db, slot_id).await?;
            Response::empty()
        }

        (Method::Post, [id, "booking"]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };

            let link = BookingLink {
                id: generate_id(),
                slug: generate_slug(),
                ..BookingLink::default()
            };

            calendar.booking_links.push(link.clone());
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;

            Response::from_html(format!(
                r#"<tr>
                    <td>{name}</td>
                    <td><code>{base_url}/book/{cal_id}/{slug}</code></td>
                    <td>{duration} min</td>
                    <td>Enabled</td>
                    <td>
                        <a href="{base_url}/admin/calendars/{cal_id}/booking/{link_id}" class="btn btn-sm">Edit</a>
                        <button class="btn btn-sm btn-danger"
                                hx-delete="{base_url}/admin/calendars/{cal_id}/booking/{link_id}"
                                hx-target="closest tr"
                                hx-swap="outerHTML">Delete</button>
                    </td>
                </tr>"#,
                base_url = base_url,
                cal_id = id,
                link_id = link.id,
                slug = html_escape(&link.slug),
                name = html_escape(&link.name),
                duration = link.duration,
            ))
        }

        (Method::Get, [id, "booking", link_id]) => {
            let calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            let link = match calendar.booking_links.iter().find(|l| l.id == *link_id) {
                Some(l) => l,
                None => return Response::error("Booking link not found", 404),
            };

            Response::from_html(admin_booking_link_html(&calendar, link, base_url))
        }

        (Method::Put, [id, "booking", link_id]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };

            let form = req.form_data().await?;

            if let Some(link) = calendar.booking_links.iter_mut().find(|l| l.id == *link_id) {
                if let Some(FormEntry::Field(name)) = form.get("name") {
                    link.name = name;
                }
                if let Some(FormEntry::Field(desc)) = form.get("description") {
                    link.description = if desc.is_empty() { None } else { Some(desc) };
                }
                if let Some(FormEntry::Field(duration)) = form.get("duration") {
                    link.duration = duration.parse().unwrap_or(30);
                }
                if let Some(FormEntry::Field(min_notice)) = form.get("min_notice") {
                    link.min_notice = min_notice.parse().unwrap_or(24);
                }
                if let Some(FormEntry::Field(max_advance)) = form.get("max_advance") {
                    link.max_advance = max_advance.parse().unwrap_or(30);
                }
                if let Some(FormEntry::Field(msg)) = form.get("confirmation_message") {
                    link.confirmation_message = msg;
                }
                link.enabled = form.get("enabled").is_some();
                link.auto_accept = form.get("auto_accept").is_some();
                link.hide_title = form.get("hide_title").is_some();
                if let Some(FormEntry::Field(responders_json)) = form.get("responders_json") {
                    if let Ok(responders) = serde_json::from_str(&responders_json) {
                        link.responders = responders;
                    }
                }
                if let Some(FormEntry::Field(admin_responders_json)) =
                    form.get("admin_responders_json")
                {
                    if let Ok(admin_responders) = serde_json::from_str(&admin_responders_json) {
                        link.admin_responders = admin_responders;
                    }
                }
            }

            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;
            Response::from_html(calendar_success_html("Booking link updated"))
        }

        (Method::Delete, [id, "booking", link_id]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            calendar.booking_links.retain(|l| l.id != *link_id);
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;
            Response::empty()
        }

        (Method::Post, [id, "view"]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };

            let link = ViewLink {
                id: generate_id(),
                slug: generate_slug(),
                ..ViewLink::default()
            };

            calendar.view_links.push(link.clone());
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;

            Response::from_html(format!(
                r#"<tr>
                    <td>{name}</td>
                    <td><code>{base_url}/view/{cal_id}/{slug}</code></td>
                    <td>{view_type:?}</td>
                    <td>Enabled</td>
                    <td>
                        <a href="{base_url}/admin/calendars/{cal_id}/view/{link_id}" class="btn btn-sm">Edit</a>
                        <button class="btn btn-sm btn-danger"
                                hx-delete="{base_url}/admin/calendars/{cal_id}/view/{link_id}"
                                hx-target="closest tr"
                                hx-swap="outerHTML">Delete</button>
                    </td>
                </tr>"#,
                base_url = base_url,
                cal_id = id,
                link_id = link.id,
                slug = html_escape(&link.slug),
                name = html_escape(&link.name),
                view_type = link.view_type,
            ))
        }

        (Method::Get, [id, "view", link_id]) => {
            let calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            let link = match calendar.view_links.iter().find(|l| l.id == *link_id) {
                Some(l) => l,
                None => return Response::error("View link not found", 404),
            };
            Response::from_html(admin_view_link_html(&calendar, link, base_url))
        }

        (Method::Put, [id, "view", link_id]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };

            let form = req.form_data().await?;

            if let Some(link) = calendar.view_links.iter_mut().find(|l| l.id == *link_id) {
                if let Some(FormEntry::Field(name)) = form.get("name") {
                    link.name = name;
                }
                if let Some(FormEntry::Field(view_type)) = form.get("view_type") {
                    link.view_type = match view_type.as_str() {
                        "week" => ViewType::Week,
                        "month" => ViewType::Month,
                        "year" => ViewType::Year,
                        "endless" => ViewType::Endless,
                        _ => ViewType::Month,
                    };
                }
                link.show_events = form.get("show_events").is_some();
                link.show_event_details = form.get("show_event_details").is_some();
                link.show_bookings = form.get("show_bookings").is_some();
                link.show_booking_details = form.get("show_booking_details").is_some();
                link.enabled = form.get("enabled").is_some();
            }

            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;
            Response::from_html(calendar_success_html("View link updated"))
        }

        (Method::Delete, [id, "view", link_id]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            calendar.view_links.retain(|l| l.id != *link_id);
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;
            Response::empty()
        }

        // Form link CRUD
        (Method::Post, [id, "form"]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };

            let link = FormLink {
                id: generate_id(),
                slug: generate_slug(),
                ..FormLink::default()
            };

            calendar.form_links.push(link.clone());
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;

            Response::from_html(format!(
                r#"<tr>
                    <td>{name}</td>
                    <td><code>{base_url}/form/{cal_id}/{slug}</code></td>
                    <td>Not set</td>
                    <td>Enabled</td>
                    <td>
                        <a href="{base_url}/admin/calendars/{cal_id}/form/{link_id}" class="btn btn-sm">Edit</a>
                        <button class="btn btn-sm btn-danger"
                                hx-delete="{base_url}/admin/calendars/{cal_id}/form/{link_id}"
                                hx-target="closest tr"
                                hx-swap="outerHTML">Delete</button>
                    </td>
                </tr>"#,
                base_url = base_url,
                cal_id = id,
                link_id = link.id,
                slug = html_escape(&link.slug),
                name = html_escape(&link.name),
            ))
        }

        (Method::Get, [id, "form", link_id]) => {
            let calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            let link = match calendar.form_links.iter().find(|l| l.id == *link_id) {
                Some(l) => l,
                None => return Response::error("Form link not found", 404),
            };
            Response::from_html(admin_form_link_html(&calendar, link, base_url))
        }

        (Method::Put, [id, "form", link_id]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };

            let form = req.form_data().await?;

            if let Some(link) = calendar.form_links.iter_mut().find(|l| l.id == *link_id) {
                if let Some(FormEntry::Field(name)) = form.get("name") {
                    link.name = name;
                }
                if let Some(FormEntry::Field(url)) = form.get("google_form_url") {
                    link.google_form_url = url;
                }
                link.enabled = form.get("enabled").is_some();
            }

            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;
            Response::from_html(calendar_success_html("Form link updated"))
        }

        (Method::Delete, [id, "form", link_id]) => {
            let mut calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            calendar.form_links.retain(|l| l.id != *link_id);
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;
            Response::empty()
        }

        // Form responses (via Google Forms API)
        (Method::Get, [id, "form", link_id, "responses"]) => {
            let calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            let link = match calendar.form_links.iter().find(|l| l.id == *link_id) {
                Some(l) => l,
                None => return Response::error("Form link not found", 404),
            };

            let form_id = crate::google_forms::parse_form_id(&link.google_form_url);
            if form_id.is_empty() {
                return Response::from_html(calendar_error_html("No Google Form URL configured"));
            }

            let encryption_key = env
                .secret("ENCRYPTION_KEY")
                .map(|s| s.to_string())
                .unwrap_or_default();
            let creds = get_tenant_credentials(&kv, tenant_id, &encryption_key)
                .await
                .unwrap_or_default();
            let (sa_email, sa_key) = match (
                &creds.google_service_account_email,
                &creds.google_private_key,
            ) {
                (Some(e), Some(k)) => (e.as_str(), k.as_str()),
                _ => {
                    return Response::from_html(calendar_error_html(
                        "Google service account not configured. Add credentials in Settings.",
                    ))
                }
            };

            let form_result = crate::google_forms::get_form(sa_email, sa_key, &form_id).await;
            let responses_result =
                crate::google_forms::get_responses(sa_email, sa_key, &form_id).await;

            match (form_result, responses_result) {
                (Ok(form), Ok(responses)) => {
                    Response::from_html(admin_form_responses_html(
                        &calendar, link, &form, &responses, base_url,
                    ))
                }
                (Err(e), _) | (_, Err(e)) => {
                    Response::from_html(calendar_error_html(&format!(
                        "Failed to fetch form data: {}. Make sure the form is shared with your service account email.",
                        e
                    )))
                }
            }
        }

        (Method::Get, [id, "bookings"]) => {
            let calendar = match get_calendar(&kv, id).await? {
                Some(c) => c,
                None => return Response::error("Calendar not found", 404),
            };
            let today = today_date();
            let end_date = add_days(&today, 90);
            let bookings = get_bookings(&db, id, &today, &end_date).await?;
            Response::from_html(admin_bookings_html(&calendar, &bookings, base_url))
        }

        (Method::Post, [_id, "bookings", booking_id, "cancel"]) => {
            if let Some(mut booking) = get_booking(&db, booking_id).await? {
                booking.status = BookingStatus::Cancelled;
                booking.updated_at = now_iso();
                save_booking(&db, &booking, tenant_id).await?;
            }
            Response::from_html(r#"<tr><td colspan="6">Booking cancelled</td></tr>"#)
        }

        _ => Response::error("Not Found", 404),
    }
}

/// Migrate legacy data (WhatsApp creds from TenantCredentials, form_links, instagram_sources)
/// into the new resource model.
async fn handle_migrate(
    _req: Request,
    env: Env,
    method: Method,
    tenant_id: &str,
) -> Result<Response> {
    if method != Method::Post {
        return Response::error("Method not allowed", 405);
    }

    let kv = env.kv("CALENDARS_KV")?;
    let encryption_key = env
        .secret("ENCRYPTION_KEY")
        .map(|s| s.to_string())
        .unwrap_or_default();

    let mut migrated = Vec::new();

    // 1. Migrate WhatsApp credentials from TenantCredentials
    let creds = get_tenant_credentials(&kv, tenant_id, &encryption_key)
        .await
        .unwrap_or_default();

    let wa_account_id =
        if creds.whatsapp_access_token.is_some() && creds.whatsapp_phone_number_id.is_some() {
            let now = now_iso();
            let account = WhatsAppAccount {
                id: generate_id(),
                tenant_id: tenant_id.to_string(),
                name: String::from("Default WhatsApp"),
                phone_number: String::new(),
                auto_reply: AutoReplyConfig::default(),
                created_at: now.clone(),
                updated_at: now,
            };
            save_whatsapp_account(&kv, &account).await?;

            let wa_creds = WhatsAppAccountCredentials {
                access_token: creds.whatsapp_access_token.clone().unwrap_or_default(),
                phone_number_id: creds.whatsapp_phone_number_id.clone().unwrap_or_default(),
            };
            save_whatsapp_credentials(&kv, &account.id, &wa_creds, &encryption_key).await?;
            migrated.push(format!("Created WhatsApp account: {}", account.id));
            Some(account.id)
        } else {
            None
        };

    // 2. Migrate form_links and instagram_sources from calendars
    let calendars = list_calendars(&kv, tenant_id).await?;
    for mut calendar in calendars {
        let mut changed = false;

        // Migrate form_links -> GoogleFormResource
        for form_link in &calendar.form_links {
            let now = now_iso();
            let form = GoogleFormResource {
                id: generate_id(),
                tenant_id: tenant_id.to_string(),
                name: form_link.name.clone(),
                slug: form_link.slug.clone(),
                google_form_url: form_link.google_form_url.clone(),
                enabled: form_link.enabled,
                whatsapp_account_id: wa_account_id.clone(),
                phone_field: String::new(),
                reply_prompt: String::new(),
                use_ai: false,
                last_polled_at: None,
                created_at: now.clone(),
                updated_at: now,
            };
            save_form_resource(&kv, &form).await?;
            migrated.push(format!(
                "Migrated form '{}' from calendar '{}'",
                form_link.name, calendar.name
            ));
        }

        // Migrate instagram_sources -> InstagramAccount
        for source in &calendar.instagram_sources {
            let account = InstagramAccount {
                id: generate_id(),
                tenant_id: tenant_id.to_string(),
                instagram_user_id: source.instagram_user_id.clone(),
                instagram_username: source.instagram_username.clone(),
                target_calendar_id: Some(calendar.id.clone()),
                classification_prompt: None,
                enabled: source.enabled,
                last_synced_at: source.last_synced_at.clone(),
                created_at: source.created_at.clone(),
            };

            // Migrate the Instagram token to the new key format
            let old_key = format!("instagram_token:{}:{}", calendar.id, source.id);
            if let Ok(Some(token_data)) = kv.get(&old_key).text().await {
                kv.put(&format!("instagram_token:{}", account.id), &token_data)?
                    .execute()
                    .await?;
            }

            save_instagram_account(&kv, &account).await?;
            migrated.push(format!(
                "Migrated Instagram @{} from calendar '{}'",
                source.instagram_username, calendar.name
            ));
        }

        // Set whatsapp_account_id on booking links and digest
        if let Some(ref wa_id) = wa_account_id {
            for link in &mut calendar.booking_links {
                if link.whatsapp_account_id.is_none() {
                    link.whatsapp_account_id = Some(wa_id.clone());
                    changed = true;
                }
            }
            if calendar.digest.whatsapp_account_id.is_none()
                && calendar.digest.frequency != DigestFrequency::None
            {
                calendar.digest.whatsapp_account_id = Some(wa_id.clone());
                changed = true;
            }
        }

        if changed {
            calendar.updated_at = now_iso();
            save_calendar(&kv, &calendar).await?;
            migrated.push(format!(
                "Updated calendar '{}' with WhatsApp account references",
                calendar.name
            ));
        }
    }

    if migrated.is_empty() {
        Response::from_html(calendar_success_html("Nothing to migrate"))
    } else {
        let summary = migrated
            .iter()
            .map(|m| format!("<li>{}</li>", html_escape(m)))
            .collect::<String>();
        Response::from_html(format!(
            "<div class=\"success\"><strong>Migration complete:</strong><ul>{}</ul></div>",
            summary
        ))
    }
}
