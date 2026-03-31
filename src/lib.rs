use worker::*;

mod ai;
mod crypto;
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

pub const LOGO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="2" y="2" width="56" height="42" rx="12" fill="#F38020"/>
  <polygon points="10,44 4,58 24,44" fill="#F38020"/>
  <path d="M39,15 C33,11 23,13 20,20 C17,27 20,35 27,38 C30,39 33,39 36,38" stroke="#fff" stroke-width="6.5" stroke-linecap="round" fill="none"/>
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

    // Lead capture form routes (public)
    if path.starts_with("/lead/") {
        return handlers::handle_lead_form(req, env, path, method).await;
    }

    // Instagram OAuth routes
    if path.starts_with("/instagram/") {
        return handlers::handle_instagram(req, env, path, method).await;
    }

    // Webhook routes (WhatsApp + Instagram incoming messages)
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
