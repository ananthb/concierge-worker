//! Billing templates — tenant-facing credit balance + purchase

use crate::helpers::html_escape;
use crate::types::{CreditPackRow, TenantBilling};

use super::base::{app_shell, base_html};
use super::HASH;

pub fn billing_overview_html(
    billing: &TenantBilling,
    packs: &[CreditPackRow],
    currency: &str,
    base_url: &str,
) -> String {
    let credits_style = if billing.replies_remaining <= 0 {
        r#"style="color:var(--warn)""#
    } else {
        ""
    };

    let pack_buttons: String = packs
        .iter()
        .map(|p| {
            let price = if currency == "INR" {
                format!("₹{}", p.price_inr / 100)
            } else {
                format!("${}", p.price_usd / 100)
            };
            format!(
                r##"<form hx-post="{base_url}/admin/billing/checkout" hx-target="body" hx-swap="innerHTML" style="display:inline">
  <input type="hidden" name="credits" value="{credits}">
  <button class="btn sm" type="submit">{name}: {credits} replies &mdash; {price}</button>
</form>"##,
                base_url = base_url,
                credits = p.replies,
                name = html_escape(&p.name),
                price = price,
            )
        })
        .collect();

    let content = format!(
        r##"<div style="padding:24px 28px">
  <p><a href="{base_url}/admin">&larr; Back to Dashboard</a></p>
  <div class="eyebrow">Billing</div>
  <h2 class="display-sm" style="margin:4px 0 16px">Reply credits</h2>

  <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:16px;margin-bottom:24px">
    <div class="card" style="padding:18px;text-align:center">
      <div class="stat-n serif" {credits_style}>{remaining}</div>
      <div class="mono muted" style="font-size:11px">Replies remaining</div>
    </div>
    <div class="card" style="padding:18px;text-align:center">
      <div class="stat-n serif">{used}</div>
      <div class="mono muted" style="font-size:11px">Replies sent</div>
    </div>
    <div class="card" style="padding:18px;text-align:center">
      <div class="stat-n serif">{granted}</div>
      <div class="mono muted" style="font-size:11px">Total granted</div>
    </div>
  </div>

  <div class="card" style="padding:22px">
    <h3 style="margin-bottom:8px">Buy reply credits</h3>
    <p class="muted" style="margin-bottom:16px">Purchase a pack to top up your balance. First 100 replies each month are free.</p>
    <div class="row gap-12" style="flex-wrap:wrap">
      {pack_buttons}
    </div>
  </div>
</div>"##,
        base_url = base_url,
        remaining = billing.replies_remaining,
        used = billing.replies_used,
        granted = billing.replies_granted,
        credits_style = credits_style,
        pack_buttons = pack_buttons,
    );

    let page = app_shell(&content, "Billing", base_url);
    base_html("Billing - Concierge", &page)
}

pub fn checkout_html(
    order_id: &str,
    amount: i64,
    currency: &str,
    credits: i64,
    razorpay_key: &str,
    tenant_id: &str,
    base_url: &str,
) -> String {
    let display_amount = if currency == "INR" {
        format!("₹{}", amount / 100)
    } else {
        format!("${}", amount / 100)
    };

    // Using r##" to avoid issues with JS single quotes
    let content = format!(
        r##"<div style="max-width:480px;margin:4rem auto;text-align:center;padding:0 1rem">
  <div class="card" style="padding:28px">
    <h2 class="display-sm">Complete purchase</h2>
    <p class="muted" style="margin:8px 0 24px">Buying <strong>{credits}</strong> reply credits</p>
    <div class="stat-n serif" style="margin-bottom:24px">{display_amount}</div>
    <button id="pay-btn" class="btn primary lg" style="width:100%">Pay with Razorpay</button>
    <p class="mono muted" style="font-size:11px;margin-top:12px">Secure payment via Razorpay</p>
  </div>
  <a href="{base_url}/admin/billing" class="btn ghost sm" style="margin-top:16px">&larr; Cancel</a>
</div>
<script src="https://checkout.razorpay.com/v1/checkout.js"></script>
<script>
document.getElementById("pay-btn").addEventListener("click", function() {{
  var options = {{
    key: "{key}",
    amount: {amount},
    currency: "{currency}",
    order_id: "{order_id}",
    name: "Concierge",
    description: "{credits} reply credits",
    notes: {{ tenant_id: "{tenant_id}", credits: "{credits}" }},
    handler: function(response) {{
      fetch("{base_url}/admin/billing/verify", {{
        method: "POST",
        headers: {{"Content-Type": "application/json"}},
        body: JSON.stringify({{
          razorpay_order_id: response.razorpay_order_id,
          razorpay_payment_id: response.razorpay_payment_id,
          razorpay_signature: response.razorpay_signature,
          credits: "{credits}"
        }})
      }}).then(function() {{ window.location.href = "{base_url}/admin/billing"; }});
    }},
    theme: {{ color: "#E86A2C" }}
  }};
  new Razorpay(options).open();
}});
</script>"##,
        credits = credits,
        display_amount = display_amount,
        amount = amount,
        currency = currency,
        order_id = html_escape(order_id),
        key = html_escape(razorpay_key),
        tenant_id = html_escape(tenant_id),
        base_url = base_url,
    );

    base_html("Checkout - Concierge", &content)
}
