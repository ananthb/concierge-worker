//! Durable Object classes used by the worker.
//!
//! Each DO needs to be exported via `wasm_bindgen` from `src/lib.rs` (the
//! `#[durable_object]` macro handles that under the hood).

pub mod approvals_do;
pub mod reply_buffer;

pub use approvals_do::ApprovalsDO;
pub use reply_buffer::ReplyBufferDO;
