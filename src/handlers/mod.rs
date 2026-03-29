//! Handler modules for the concierge worker
//!
//! Each module handles a specific domain of routes.

mod admin;
mod admin_forms;
mod admin_instagram;
mod admin_whatsapp;
pub mod auth;
mod booking;
mod forms;
mod instagram_oauth;
mod views;
mod webhook;

pub use admin::handle_admin;
pub use auth::handle_auth;
pub use booking::handle_booking;
pub use forms::handle_form;
pub use instagram_oauth::handle_instagram;
pub use views::handle_view;
pub use webhook::handle_webhook;

use worker::Request;

/// Extract base URL from request
pub(crate) fn get_base_url(req: &Request) -> String {
    let url = req.url().unwrap();
    format!(
        "{}://{}",
        url.scheme(),
        url.host_str().unwrap_or("localhost")
    )
}

/// Extract Origin header from request
pub(crate) fn get_origin(req: &Request) -> Option<String> {
    req.headers().get("Origin").ok().flatten()
}
