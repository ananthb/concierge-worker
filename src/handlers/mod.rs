//! Handler modules for the concierge worker
//!
//! Each module handles a specific domain of routes.

mod admin;
mod booking;
mod forms;
mod instagram_oauth;
mod views;
mod webhooks;

pub use admin::handle_admin;
pub use booking::handle_booking;
pub use forms::handle_form_routes;
pub use instagram_oauth::handle_instagram;
pub use views::{handle_feed, handle_view};
pub use webhooks::handle_webhook;

use worker::Request;

/// Extract base URL from request
pub(crate) fn get_base_url(req: &Request) -> String {
    let url = req.url().unwrap();
    format!("{}://{}", url.scheme(), url.host_str().unwrap_or("localhost"))
}

/// Extract Origin header from request
pub(crate) fn get_origin(req: &Request) -> Option<String> {
    req.headers().get("Origin").ok().flatten()
}
