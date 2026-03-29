//! Public form handlers - embeds Google Forms

use worker::*;

use super::get_origin;
use crate::helpers::*;
use crate::storage::*;
use crate::templates::*;

/// Handle public form routes (/form/*)
///
/// Supports two URL patterns:
/// - /form/{form_resource_id}/{slug} — new resource-based lookup
/// - /form/{calendar_id}/{slug} — legacy lookup via CalendarConfig.form_links
pub async fn handle_form(req: Request, env: Env, path: &str, method: Method) -> Result<Response> {
    let origin = get_origin(&req);
    let kv = env.kv("CALENDARS_KV")?;

    let path_parts: Vec<&str> = path
        .strip_prefix("/form/")
        .unwrap_or("")
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let id = path_parts.first().copied().unwrap_or("");
    let slug = path_parts.get(1).copied().unwrap_or("");

    // Handle CORS preflight
    if method == Method::Options {
        // Try form resource first, then calendar
        if !id.is_empty() {
            if let Ok(Some(_form)) = get_form_resource(&kv, id).await {
                return cors_preflight(origin.as_deref(), &[]);
            }
            if let Ok(Some(calendar)) = get_calendar(&kv, id).await {
                return cors_preflight(origin.as_deref(), &calendar.allowed_origins);
            }
        }
        return cors_preflight(origin.as_deref(), &[]);
    }

    if id.is_empty() || slug.is_empty() {
        return Response::error("Form ID and slug required", 400);
    }

    if method != Method::Get {
        return Response::error("Method not allowed", 405);
    }

    let url = req.url()?;
    let query_pairs: std::collections::HashMap<_, _> = url.query_pairs().collect();
    let inline_css = query_pairs.get("css").map(|s| s.to_string());
    let css_url = query_pairs.get("css_url").map(|s| s.to_string());
    let css_options = CssOptions {
        inline_css: inline_css.as_deref(),
        css_url: css_url.as_deref(),
    };
    let hide_title = query_pairs
        .get("notitle")
        .map(|v| v == "1" || v == "true")
        .unwrap_or(false);
    let is_htmx = is_htmx_request(&req);

    // Try new resource-based lookup first
    if let Ok(Some(form)) = get_form_resource(&kv, id).await {
        if form.slug == slug && form.enabled {
            let html = form_resource_embed_html(&form, &css_options, is_htmx, hide_title);
            let response = Response::from_html(html)?;
            return Ok(with_cors(response, origin.as_deref(), &[]));
        }
    }

    // Fallback: legacy CalendarConfig.form_links lookup
    let calendar = match get_calendar(&kv, id).await? {
        Some(c) => c,
        None => return Response::error("Not found", 404),
    };

    let link = match calendar
        .form_links
        .iter()
        .find(|l| l.slug == slug && l.enabled)
    {
        Some(l) => l.clone(),
        None => return Response::error("Form not found", 404),
    };

    let html = form_embed_html(&calendar, &link, &css_options, is_htmx, hide_title);
    let response = Response::from_html(html)?;
    Ok(with_cors(
        response,
        origin.as_deref(),
        &calendar.allowed_origins,
    ))
}
