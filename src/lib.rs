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

pub const LOGO_SVG: &str = r##"<svg width="200" height="200" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
  <rect x="10" y="10" width="180" height="180" rx="36" fill="#1A1A2E"/>
  <rect x="28" y="28" width="144" height="108" rx="18" fill="#F38020"/>
  <polygon points="54,136 38,164 90,136" fill="#F38020"/>
  <rect x="38" y="114" width="124" height="7" rx="3.5" fill="white"/>
  <rect x="58" y="104" width="84" height="11" rx="5.5" fill="white"/>
  <path d="M62,104 C58,84 58,62 100,54 C142,62 142,84 138,104 Z" fill="white"/>
  <rect x="88" y="49" width="24" height="8" rx="4" fill="white"/>
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
