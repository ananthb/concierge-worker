use worker::*;

mod ai;
mod crypto;
mod google_calendar;
mod google_forms;
mod handlers;
mod helpers;
mod instagram;
mod landing;
mod scheduled;
mod storage;
mod templates;
mod types;
mod whatsapp;

pub use types::*;

// Combined logo for concierge - form + calendar elements
pub const LOGO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" fill="none">
  <rect x="8" y="8" width="48" height="48" rx="4" fill="#fff" stroke="#333" stroke-width="2"/>
  <rect x="8" y="8" width="48" height="12" rx="4" fill="#0070f3"/>
  <line x1="14" y1="28" x2="50" y2="28" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
  <line x1="14" y1="36" x2="50" y2="36" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
  <line x1="14" y1="44" x2="40" y2="44" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
  <rect x="42" y="40" width="10" height="10" rx="2" fill="#0070f3"/>
  <path d="M44 45 L46 47 L50 43" stroke="#fff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
</svg>"##;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let url = req.url()?;
    let path = url.path();
    let method = req.method();

    // Serve logo
    if path == "/logo.svg" {
        let headers = Headers::new();
        headers.set("Content-Type", "image/svg+xml")?;
        headers.set("Cache-Control", "public, max-age=31536000")?;
        return Ok(Response::ok(LOGO_SVG)?.with_headers(headers));
    }

    // Health check
    if path == "/health" {
        return Response::ok("OK");
    }

    // Auth routes (login, callback, logout)
    if path.starts_with("/auth/") {
        return handlers::handle_auth(req, env, path, method).await;
    }

    // Admin routes (session-protected)
    if path.starts_with("/admin") {
        return handlers::handle_admin(req, env, path, method).await;
    }

    // Booking routes (public)
    if path.starts_with("/book/") {
        return handlers::handle_booking(req, env, path, method).await;
    }

    // Google Form embed routes (public)
    if path.starts_with("/form/") {
        return handlers::handle_form(req, env, path, method).await;
    }

    // Calendar view routes (public)
    if path.starts_with("/view/") {
        return handlers::handle_view(req, env, path, method).await;
    }

    // Instagram OAuth routes
    if path.starts_with("/instagram/") {
        return handlers::handle_instagram(req, env, path, method).await;
    }

    // Webhook routes (WhatsApp incoming messages)
    if path.starts_with("/webhook/") {
        return handlers::handle_webhook(req, env, path, method).await;
    }

    // Landing page
    if path == "/" || path == "/index.html" {
        let headers = Headers::new();
        headers.set("Content-Type", "text/html; charset=utf-8")?;
        headers.set("Cache-Control", "public, max-age=3600")?;
        return Ok(Response::ok(landing::landing_page_html())?.with_headers(headers));
    }

    Response::error("Not Found", 404)
}

#[event(scheduled)]
async fn scheduled_handler(event: ScheduledEvent, env: Env, ctx: ScheduleContext) {
    scheduled::handle_scheduled(event, env, ctx).await;
}
