//! Template rendering for HTML pages

mod admin;
pub mod admin_email;
pub mod base;
mod lead_form;
pub mod onboarding;

pub use admin::*;
pub use lead_form::*;

/// Hash character constant for use in format strings (avoids escaping issues)
pub(crate) const HASH: &str = "#";
