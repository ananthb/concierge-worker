//! Template rendering for HTML pages
//!
//! This module is split into submodules:
//! - `base`: Base HTML wrappers, CSS handling
//! - `admin`: Admin dashboard and calendar management pages
//! - `booking`: Public booking form and confirmation pages
//! - `calendar`: Calendar view and iCal feed generation
//! - `forms`: Form editor, responses, and public form rendering

mod admin;
mod base;
mod booking;
mod calendar;
mod forms;

pub use admin::*;
pub use base::{wrap_html, AvailableChannels, CssOptions};
pub use booking::*;
pub use calendar::*;
pub use forms::*;

/// Hash character constant for use in format strings (avoids escaping issues)
pub(crate) const HASH: &str = "#";
