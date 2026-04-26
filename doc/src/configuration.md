

<div class="page-eyebrow">Reference</div>
<h1 class="page-title">Configuration <em>&amp; secrets</em>.</h1>
<p class="page-lede">
  Every environment variable, secret, binding, OAuth redirect, webhook URL, and cron trigger Concierge expects. All sensitive values are stored as Cloudflare Workers secrets via <code>wrangler secret put</code>; non&#8209;sensitive values live in the <code>[vars]</code> block of <code>wrangler.toml</code>.
</p>

<div class="page-meta">
  <span><b>Total secrets</b> 14 required · 4 optional</span>
  <span><b>Env vars</b> 8</span>
  <span><b>Bindings</b> 6</span>
</div>

<div class="callout callout--note">
  <div class="callout__icon">i</div>
  <div class="callout__body">
    <strong>Setting secrets</strong>
    <p>Use the Wrangler CLI: <code>wrangler secret put SECRET_NAME</code>. You&rsquo;ll be prompted to enter the value securely. Secrets are encrypted at rest by Cloudflare and exposed to your Worker as environment variables.</p>
  </div>
</div>

<h2 id="env">Environment variables</h2>
<p>Plain&#8209;text vars in the <code>[vars]</code> block of <code>wrangler.toml</code>. Safe to commit.</p>

<div class="table-wrap">
  <div class="table-wrap__head">
    <b>[vars]</b><span>wrangler.toml · plaintext, committable</span>
    <span class="count">8 vars</span>
  </div>
  <table class="docs">
    <thead><tr><th style="width:30%">Variable</th><th>Description</th></tr></thead>
    <tbody>
      <tr><td><code>ENVIRONMENT</code></td><td class="muted"><code>production</code> or <code>development</code>. Dev mode bypasses some auth checks.</td></tr>
      <tr><td><code>CF_ACCESS_TEAM</code></td><td class="muted">Cloudflare Access team name (the subdomain in <code>myteam.cloudflareaccess.com</code>).</td></tr>
      <tr><td><code>CF_ACCESS_AUD</code></td><td class="muted">Cloudflare Access Application Audience (AUD) tag for <code>/manage/*</code>.</td></tr>
      <tr><td><code>WHATSAPP_WABA_ID</code></td><td class="muted">Your WhatsApp Business Account ID.</td></tr>
      <tr><td><code>WHATSAPP_SIGNUP_CONFIG_ID</code></td><td class="muted">Meta Embedded Signup configuration ID.</td></tr>
      <tr><td><code>EMAIL_BASE_DOMAIN</code></td><td class="muted">Single platform email domain (default <code>cncg.email</code>). MX records configured manually on the apex.</td></tr>
      <tr><td><code>AI_MODEL</code></td><td class="muted">Optional override for the Workers AI model used for replies. Default <code>@cf/meta/llama-4-scout-17b-16e-instruct</code>.</td></tr>
      <tr><td><code>AI_FAST_MODEL</code></td><td class="muted">Optional override for the fast classifier model (prompt&#8209;injection scan + persona safety check). Default <code>@cf/meta/llama-3.1-8b-instruct-fast</code>.</td></tr>
    </tbody>
  </table>
</div>

<h2 id="secrets">Secrets</h2>
<p>Sensitive values, set with <code>wrangler secret put</code>. Grouped by integration.</p>

<h3 id="core">Core</h3>
<div class="table-wrap">
  <div class="table-wrap__head">
    <b>Encryption</b><span>required for token storage</span>
    <span class="count">1 secret</span>
  </div>
  <table class="docs">
    <thead><tr><th style="width:32%">Secret</th><th>Description</th></tr></thead>
    <tbody>
      <tr>
        <td><code>ENCRYPTION_KEY</code></td>
        <td class="muted">32&#8209;byte hex key for AES&#8209;256&#8209;GCM encryption of stored tokens. Generate with <code>openssl rand -hex 32</code>.</td>
      </tr>
    </tbody>
  </table>
</div>

<h3 id="google">Google OAuth</h3>
<div class="table-wrap">
  <div class="table-wrap__head">
    <b>Sign-in</b><span>used by /auth/login</span>
    <span class="count">2 secrets</span>
  </div>
  <table class="docs">
    <thead><tr><th style="width:32%">Secret</th><th>Description</th></tr></thead>
    <tbody>
      <tr><td><code>GOOGLE_OAUTH_CLIENT_ID</code></td><td class="muted">Google OAuth client ID (for sign-in).</td></tr>
      <tr><td><code>GOOGLE_OAUTH_CLIENT_SECRET</code></td><td class="muted">Google OAuth client secret.</td></tr>
    </tbody>
  </table>
</div>

<h3 id="meta">Meta &mdash; Facebook / Instagram / WhatsApp</h3>
<div class="table-wrap">
  <div class="table-wrap__head">
    <b>Meta Graph API</b><span>shared app for FB Login, Instagram, WhatsApp signup</span>
    <span class="count">5 secrets</span>
  </div>
  <table class="docs">
    <thead><tr><th style="width:32%">Secret</th><th>Description</th></tr></thead>
    <tbody>
      <tr><td><code>META_APP_ID</code></td><td class="muted">Meta app ID — shared for Facebook Login, Instagram, WhatsApp signup.</td></tr>
      <tr><td><code>META_APP_SECRET</code></td><td class="muted">Meta app secret — used for webhook signature verification and token exchange.</td></tr>
      <tr><td><code>WHATSAPP_ACCESS_TOKEN</code></td><td class="muted">System user token for your shared WABA.</td></tr>
      <tr><td><code>WHATSAPP_VERIFY_TOKEN</code></td><td class="muted">Webhook verification token. Generate with <code>openssl rand -hex 16</code>.</td></tr>
      <tr><td><code>INSTAGRAM_VERIFY_TOKEN</code></td><td class="muted">Instagram webhook verification token. Generate with <code>openssl rand -hex 16</code>.</td></tr>
    </tbody>
  </table>
</div>

<h3 id="discord">Discord</h3>
<div class="table-wrap">
  <div class="table-wrap__head">
    <b>Interactions API</b><span>slash commands, button &amp; modal callbacks</span>
    <span class="count">3 secrets</span>
  </div>
  <table class="docs">
    <thead><tr><th style="width:32%">Secret</th><th>Description</th></tr></thead>
    <tbody>
      <tr><td><code>DISCORD_PUBLIC_KEY</code></td><td class="muted">Discord application public key (Ed25519 signature verification for interactions).</td></tr>
      <tr><td><code>DISCORD_APPLICATION_ID</code></td><td class="muted">Discord application ID (for registering slash commands).</td></tr>
      <tr><td><code>DISCORD_BOT_TOKEN</code></td><td class="muted">Discord bot token (for sending messages and managing interactions).</td></tr>
    </tbody>
  </table>
</div>

<h3 id="razorpay">Razorpay <span class="pill" style="margin-left:8px;vertical-align:middle">optional</span></h3>
<div class="table-wrap">
  <div class="table-wrap__head">
    <b>Payments</b><span>credit packs &amp; subscription billing</span>
    <span class="count">3 secrets</span>
  </div>
  <table class="docs">
    <thead><tr><th style="width:32%">Secret</th><th>Description</th></tr></thead>
    <tbody>
      <tr><td><code>RAZORPAY_KEY_ID</code></td><td class="muted">Razorpay API key ID.</td></tr>
      <tr><td><code>RAZORPAY_KEY_SECRET</code></td><td class="muted">Razorpay API key secret.</td></tr>
      <tr><td><code>RAZORPAY_WEBHOOK_SECRET</code></td><td class="muted">Razorpay webhook secret (for verifying payment notifications at <code>POST /webhook/razorpay</code>).</td></tr>
    </tbody>
  </table>
</div>

<h3 id="dns">Cloudflare DNS <span class="pill" style="margin-left:8px;vertical-align:middle">optional</span></h3>
<div class="table-wrap">
  <div class="table-wrap__head">
    <b>Email subdomain provisioning</b><span>required if you sell email subdomains</span>
    <span class="count">1 secret</span>
  </div>
  <table class="docs">
    <thead><tr><th style="width:32%">Secret</th><th>Description</th></tr></thead>
    <tbody>
      <tr>
        <td><code>CF_DNS_API_TOKEN</code></td>
        <td class="muted">Cloudflare API token scoped to <em>DNS Edit</em> on the email zone. Used to create / delete MX records for tenant subdomains.</td>
      </tr>
    </tbody>
  </table>
</div>

<h2 id="bindings">Bindings</h2>
<p>Cloudflare resource bindings declared in <code>wrangler.toml</code>. The IDs come from the <a href="deployment.html">deployment guide</a>.</p>

<div class="table-wrap">
  <div class="table-wrap__head">
    <b>wrangler.toml</b><span>resource bindings</span>
    <span class="count">6 bindings</span>
  </div>
  <table class="docs">
    <thead><tr><th>Binding</th><th>Type</th><th>Description</th></tr></thead>
    <tbody>
      <tr><td><code>DB</code></td><td><span class="pill">D1</span></td><td class="muted">SQLite database for message logs, payments, audit, credit packs.</td></tr>
      <tr><td><code>KV</code></td><td><span class="pill">KV</span></td><td class="muted">Config store for tenants, accounts, sessions, routing rules, billing state, persona.</td></tr>
      <tr><td><code>AI</code></td><td><span class="pill">AI</span></td><td class="muted">Cloudflare Workers AI for reply generation, prompt&#8209;injection scanning, persona safety classification, and BGE embeddings.</td></tr>
      <tr><td><code>EMAIL</code></td><td><span class="pill">send_email</span></td><td class="muted">Outbound email for forwarding and reverse&#8209;alias replies.</td></tr>
      <tr><td><code>REPLY_BUFFER</code></td><td><span class="pill">DO</span></td><td class="muted">Per&#8209;conversation reply buffer that batches multi&#8209;message bursts within <code>wait_seconds</code>.</td></tr>
      <tr><td><code>SAFETY_QUEUE</code></td><td><span class="pill">Queue</span></td><td class="muted">Persona safety classifier jobs. Producer + consumer bound on the same Worker.</td></tr>
    </tbody>
  </table>
</div>

<h2 id="queues">Cloudflare Queues</h2>
<p>Two queues must exist before deploy:</p>
<ul>
  <li><code>concierge-safety</code> — producer + consumer for the persona safety classifier. The Worker enqueues a <code>SafetyJob</code> on each persona save and consumes the same queue via <code>#[event(queue)]</code>.</li>
  <li><code>concierge-safety-dlq</code> — dead&#8209;letter queue for safety jobs after 3 retries. Inspect failures here when persona checks stop completing.</li>
</ul>
<div class="codeblock">
  <div class="codeblock__head">
    <span class="codeblock__lang">bash</span>
    <span class="codeblock__file">terminal</span>
    <button class="codeblock__copy">Copy</button>
  </div>
<pre><code><span class="tok-k">wrangler</span> queues create concierge-safety
<span class="tok-k">wrangler</span> queues create concierge-safety-dlq</code></pre>
</div>
<p class="muted">Local <code>wrangler dev</code> works without queues configured: <code>safety_queue::enqueue</code> logs and falls through, leaving personas in <em>Pending</em> until you deploy against a properly bound environment.</p>

<h2 id="locale">Locale &amp; supported languages</h2>
<p>
  Every tenant has a BCP&#8209;47 locale tag (<code>Tenant.locale</code>) plus an independent currency override. Currently shipped: <code>en-IN</code> (default; Indian&#8209;style number grouping, INR currency) and <code>en-US</code> (Western grouping, USD). Locale is set at signup from the request&rsquo;s <code>Accept-Language</code> header, falling back to <code>cf-ipcountry</code>, then to <code>en-IN</code>. Admins can change both locale and currency independently from <code>/admin/settings</code>.
</p>
<p>
  Adding a new locale is a drop&#8209;in change: place a translated <code>messages.ftl</code> at <code>assets/locales/{tag}/</code>, register the tag in <code>src/i18n.rs::Translator::new</code> and <code>src/locale.rs::Locale::from_request</code>, then rebuild. CLDR data for number / currency formatting ships automatically via icu4x&rsquo;s <code>compiled_data</code> feature. AI&#8209;generated reply content stays English regardless of the UI locale.
</p>

<h2 id="oauth">OAuth redirect URIs</h2>
<p>Register these in the respective developer consoles:</p>
<ul>
  <li><strong>Google:</strong> <code>https://your-domain/auth/callback</code></li>
  <li><strong>Facebook:</strong> <code>https://your-domain/auth/facebook/callback</code> and <code>https://your-domain/instagram/callback</code></li>
</ul>

<h2 id="webhooks">Webhook URLs</h2>
<p>Configure these in the respective platforms:</p>
<div class="table-wrap">
  <table class="docs">
    <thead><tr><th>Platform</th><th>URL</th><th>Method</th></tr></thead>
    <tbody>
      <tr><td><strong>WhatsApp</strong></td><td><code>https://your-domain/webhook/whatsapp</code></td><td><span class="pill">GET&nbsp;/&nbsp;POST</span></td></tr>
      <tr><td><strong>Instagram</strong></td><td><code>https://your-domain/webhook/instagram</code></td><td><span class="pill">GET&nbsp;/&nbsp;POST</span></td></tr>
      <tr><td><strong>Discord</strong></td><td><code>https://your-domain/discord/interactions</code></td><td><span class="pill">POST</span></td></tr>
      <tr><td><strong>Razorpay</strong></td><td><code>https://your-domain/webhook/razorpay</code></td><td><span class="pill">POST</span></td></tr>
    </tbody>
  </table>
</div>

<h2 id="cron">Cron triggers</h2>
<p>A daily cron runs at <code>0 6 * * *</code> (06:00 UTC) for Instagram token refresh. Configured in the <code>[triggers]</code> section of <code>wrangler.toml</code>.</p>

<div class="codeblock">
  <div class="codeblock__head">
    <span class="codeblock__lang">toml</span>
    <span class="codeblock__file">wrangler.toml · partial</span>
    <button class="codeblock__copy">Copy</button>
  </div>
<pre><code><span class="tok-c"># Identity</span>
<span class="tok-v">name</span> = <span class="tok-s">"concierge"</span>
<span class="tok-v">main</span> = <span class="tok-s">"worker-shim.mjs"</span>
<span class="tok-v">compatibility_date</span> = <span class="tok-s">"2024-01-01"</span>
<!-- -->
<span class="tok-c"># Plain-text variables</span>
[<span class="tok-k">vars</span>]
<span class="tok-v">ENVIRONMENT</span>           = <span class="tok-s">"production"</span>
<span class="tok-v">CF_ACCESS_TEAM</span>        = <span class="tok-s">"calculon"</span>
<span class="tok-v">CF_ACCESS_AUD</span>         = <span class="tok-s">"abcd1234…"</span>
<span class="tok-v">WHATSAPP_WABA_ID</span>      = <span class="tok-s">"123456789012345"</span>
<span class="tok-v">EMAIL_BASE_DOMAIN</span>     = <span class="tok-s">"cncg.email"</span>
<!-- -->
<span class="tok-c"># Bindings</span>
[[<span class="tok-k">d1_databases</span>]]
<span class="tok-v">binding</span>      = <span class="tok-s">"DB"</span>
<span class="tok-v">database_name</span> = <span class="tok-s">"concierge"</span>
<span class="tok-v">database_id</span>   = <span class="tok-s">"…"</span>
<!-- -->
[[<span class="tok-k">kv_namespaces</span>]]
<span class="tok-v">binding</span> = <span class="tok-s">"KV"</span>
<span class="tok-v">id</span>      = <span class="tok-s">"…"</span>
<!-- -->
[<span class="tok-k">ai</span>]            <span class="tok-v">binding</span> = <span class="tok-s">"AI"</span>
[[<span class="tok-k">send_email</span>]]   <span class="tok-v">name</span>    = <span class="tok-s">"EMAIL"</span>
<!-- -->
[[<span class="tok-k">durable_objects.bindings</span>]]
<span class="tok-v">name</span>       = <span class="tok-s">"REPLY_BUFFER"</span>
<span class="tok-v">class_name</span> = <span class="tok-s">"ReplyBufferDO"</span>
<!-- -->
<span class="tok-c"># Persona safety classifier</span>
[[<span class="tok-k">queues.producers</span>]]
<span class="tok-v">queue</span>   = <span class="tok-s">"concierge-safety"</span>
<span class="tok-v">binding</span> = <span class="tok-s">"SAFETY_QUEUE"</span>
<!-- -->
[[<span class="tok-k">queues.consumers</span>]]
<span class="tok-v">queue</span>             = <span class="tok-s">"concierge-safety"</span>
<span class="tok-v">max_batch_size</span>    = <span class="tok-n">10</span>
<span class="tok-v">max_batch_timeout</span> = <span class="tok-n">5</span>
<span class="tok-v">max_retries</span>       = <span class="tok-n">3</span>
<span class="tok-v">dead_letter_queue</span> = <span class="tok-s">"concierge-safety-dlq"</span>
<!-- -->
<span class="tok-c"># Daily Instagram token refresh, 06:00 UTC</span>
[<span class="tok-k">triggers</span>]
<span class="tok-v">crons</span> = [<span class="tok-s">"0 6 * * *"</span>]</code></pre>
</div>

<div class="callout callout--warn">
  <div class="callout__icon">!</div>
  <div class="callout__body">
    <strong>Rotating secrets</strong>
    <p>Rotating <code>ENCRYPTION_KEY</code> invalidates every encrypted Instagram page token in KV — customers will need to reconnect. Webhook verify tokens (<code>WHATSAPP_VERIFY_TOKEN</code>, <code>INSTAGRAM_VERIFY_TOKEN</code>) can be rotated independently; just update them in the Meta Developer Console at the same time.</p>
  </div>
</div>
