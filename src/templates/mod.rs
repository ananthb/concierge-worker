//! Template rendering for HTML pages
//!
//! This module is split into submodules:
//! - `base`: Base HTML wrappers, CSS handling
//! - `admin`: Admin dashboard and calendar management pages
//! - `booking`: Public booking form and confirmation pages
//! - `calendar`: Calendar view and iCal feed generation

mod admin;
mod base;
mod booking;
mod calendar;

pub use admin::*;
pub use base::CssOptions;
pub use booking::*;
pub use calendar::*;

/// Hash character constant for use in format strings (avoids escaping issues)
pub(crate) const HASH: &str = "#";
