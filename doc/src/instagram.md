

<div class="page-eyebrow">Channel · 2 of 4</div>
<h1 class="page-title">Instagram <em>auto&#8209;reply</em>.</h1>


<p>Instagram is one of four channels in the unified messaging pipeline. Incoming DMs are normalized into <code>InboundMessage</code> structs and processed through the common pipeline, which handles credit deduction, reply generation, and metadata logging.</p>

<h2>Connecting Instagram</h2>

<ol>
  <li>Go to <strong>Admin &gt; Instagram Accounts &gt; Connect Account</strong></li>
  <li>Sign in with Facebook (the account that manages your Instagram business page)</li>
  <li>Concierge discovers your Facebook Pages and finds the linked Instagram business account</li>
  <li>The page token is encrypted and stored for sending DM replies</li>
</ol>

<h2>Configuring Auto-Reply</h2>

<p>Each Instagram account has its own <code>ReplyConfig</code> with the same shape as WhatsApp:</p>

<ul>
  <li><strong>Enabled</strong>: master switch for the account.</li>
  <li><strong>Auto-Reply Enabled</strong>: master switch for replies on inbound DMs.</li>
  <li><strong>Default reply</strong>: edited inline; canned text or AI prompt for the no-rule-matched fallback.</li>
  <li><strong>Reply rules</strong>: ordered list managed at <code>/admin/rules/instagram/{account_id}</code>. Same matchers (Keywords, Prompt) and responses (Canned, AI prompt) as WhatsApp.</li>
</ul>

<p>The persona prompt at <code>/admin/persona</code> is the system prompt for every AI reply on this account, concatenated with the matched rule's prompt. AI replies require the persona to be safety-<code>Approved</code>.</p>

<h2>How It Works</h2>

<ol>
  <li>Meta delivers incoming DMs to <code>POST /webhook/instagram</code></li>
  <li>Concierge verifies the webhook signature</li>
  <li>Looks up the Instagram account by the recipient page ID</li>
  <li>One reply credit is deducted from the tenant's balance</li>
  <li>Generates a reply (static or AI)</li>
  <li>Sends via the Instagram Pages Messaging API</li>
  <li>If the send fails, the credit is restored</li>
  <li>Message metadata (no content) is logged to the unified <code>messages</code> table</li>
</ol>

<h2>Credit Deduction</h2>

<p>Each auto-reply deducts one reply credit. Credits are deducted before the reply is sent and restored on failure. When credits reach zero, auto-replies stop silently.</p>

<p>See <a href="billing.html">Billing</a> for details on managing credits.</p>

<h2>Requirements</h2>

<p>Your Meta app needs these permissions (requires App Review):</p>

<ul>
  <li><code>instagram_basic</code></li>
  <li><code>instagram_manage_messages</code></li>
  <li><code>pages_manage_metadata</code></li>
  <li><code>pages_messaging</code></li>
</ul>

<h2>Token Refresh</h2>

<p>Page tokens are long-lived but can expire. A daily cron job (6 AM UTC) checks all tokens and refreshes those within 7 days of expiry.</p>
