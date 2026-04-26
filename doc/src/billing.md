

<div class="page-eyebrow">Operate</div>
<h1 class="page-title">Billing <em>&amp; credits</em>.</h1>


<p>Concierge uses a prepaid reply credit system. Every auto-reply (WhatsApp, Instagram) and relay reply (Discord) deducts one credit from the tenant's balance. When credits reach zero, auto-replies stop silently.</p>

<h2>How Credits Work</h2>

<ul>
  <li>Each tenant has a <code>TenantBilling</code> record in KV tracking: <code>replies_remaining</code>, <code>replies_used</code> (lifetime), and <code>replies_granted</code> (lifetime)</li>
  <li>Credits are deducted <em>before</em> the reply is sent (optimistic deduction)</li>
  <li>If the AI generation or message send fails, the credit is automatically restored</li>
  <li>An empty reply (e.g., AI returns nothing) also triggers a credit restore</li>
</ul>

<h2>Getting Credits</h2>

<p>There are two ways to get reply credits:</p>

<h3>1. Management Grants (Free)</h3>

<p>Platform operators can grant free credits to any tenant via the <a href="management.html">management panel</a>. This is useful for onboarding, promotions, or support situations. Grants are logged to the audit trail.</p>

<h3>2. Paid Credit Packs (Razorpay)</h3>

<p>Tenants can purchase credit packs via Razorpay checkout. The available packs are configurable by management. Default packs seeded by the migration:</p>

<table>
  <thead>
    <tr><th>Pack</th><th>Replies</th><th>Price (INR)</th><th>Price (USD)</th></tr>
  </thead>
  <tbody>
    <tr><td>Starter</td><td>500</td><td>249.00</td><td>$3.00</td></tr>
    <tr><td>Growth</td><td>2,000</td><td>799.00</td><td>$10.00</td></tr>
    <tr><td>Scale</td><td>10,000</td><td>2,999.00</td><td>$36.00</td></tr>
    <tr><td>Volume</td><td>50,000</td><td>9,999.00</td><td>$120.00</td></tr>
  </tbody>
</table>

<p>Prices are stored in paise (INR) and cents (USD) internally.</p>

<h2>Razorpay Integration</h2>

<p>The payment flow works as follows:</p>

<ol>
  <li>Tenant selects a credit pack on the pricing page (<code>/pricing</code>)</li>
  <li>Concierge creates a Razorpay order via the API</li>
  <li>The Razorpay checkout widget opens in the browser</li>
  <li>On successful payment, Razorpay sends a webhook to <code>POST /webhook/razorpay</code></li>
  <li>Concierge verifies the payment signature (HMAC-SHA256) and credits the tenant's balance</li>
  <li>The payment is recorded in the <code>payments</code> D1 table</li>
</ol>

<h3>Required Secrets</h3>

<ul>
  <li><code>RAZORPAY_KEY_ID</code>: Razorpay API key ID</li>
  <li><code>RAZORPAY_KEY_SECRET</code>: Razorpay API key secret</li>
  <li><code>RAZORPAY_WEBHOOK_SECRET</code>: Webhook secret for signature verification</li>
</ul>

<h2>Credit Pack Management</h2>

<p>Platform operators can manage credit packs via the <a href="management.html">management panel</a> at <code>/manage/billing</code>:</p>

<ul>
  <li>Create new packs with custom names, reply counts, and prices</li>
  <li>Update existing pack details</li>
  <li>Activate or deactivate packs (inactive packs are hidden from tenants)</li>
  <li>Delete packs</li>
  <li>Control sort order for display on the pricing page</li>
</ul>

<p>All pack management actions are recorded in the audit log.</p>

<h2>Billing State</h2>

<p>Each tenant's billing state is stored in KV as a <code>TenantBilling</code> JSON object:</p>

<pre><code>{
  "replies_remaining": 450,
  "replies_used": 50,
  "replies_granted": 500
}</code></pre>
