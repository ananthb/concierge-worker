//! Billing — reply credit tracking + Razorpay payments.
//!
//! Credits = reply count. Management grants free credits.
//! Clients can also buy packs via Razorpay.
//! When credits hit 0, auto-replies stop.

pub mod razorpay;
pub mod webhook;

use worker::*;

use crate::storage;

/// Check if a tenant can send a reply.
pub async fn can_reply(kv: &kv::KvStore, tenant_id: &str) -> Result<bool> {
    let billing = storage::get_tenant_billing(kv, tenant_id).await?;
    Ok(billing.has_credits())
}

/// Deduct one reply after a successful send.
pub async fn deduct_reply(kv: &kv::KvStore, tenant_id: &str) -> Result<()> {
    let mut billing = storage::get_tenant_billing(kv, tenant_id).await?;
    billing.replies_remaining -= 1;
    billing.replies_used += 1;
    storage::save_tenant_billing(kv, tenant_id, &billing).await
}

/// Grant reply credits to a tenant.
pub async fn grant_replies(kv: &kv::KvStore, tenant_id: &str, count: i64) -> Result<()> {
    let mut billing = storage::get_tenant_billing(kv, tenant_id).await?;
    billing.replies_remaining += count;
    billing.replies_granted += count;
    storage::save_tenant_billing(kv, tenant_id, &billing).await
}
