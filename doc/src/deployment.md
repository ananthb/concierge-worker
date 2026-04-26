

<div class="page-eyebrow">Self&#8209;host · ~20 min</div>
<h1 class="page-title">Deployment <em>guide</em>.</h1>
<p class="page-lede">
  From a fresh <code>git clone</code> to a deployed Worker absorbing inbound messages on every channel. You&rsquo;ll provision Cloudflare resources, register Meta and Discord apps, set secrets, and wire webhooks. Plan on twenty minutes if your accounts are ready.
</p>

<div class="page-meta">
  <span><b>Audience</b> Operators / agencies</span>
  <span><b>Prereqs</b> Cloudflare · Meta · Discord</span>
  <span><b>Optional</b> Razorpay</span>
  <span><b>Time</b> ~20 min</span>
</div>

<h2 id="prereqs">Prerequisites</h2>
<div class="cards">
  <div class="card">
    <div class="card__head">
      <span class="card__chip">CF</span>
      <span class="card__title">Cloudflare account</span>
    </div>
    <p class="card__desc">Workers, D1, KV, Queues, and Email Routing enabled. Free tier is fine to start; D1 + AI usage will be the first thing to outgrow it.</p>
  </div>
  <div class="card">
    <div class="card__head">
      <span class="card__chip">META</span>
      <span class="card__title">Meta Developer app</span>
    </div>
    <p class="card__desc">A Facebook App with WhatsApp Business and Instagram Graph products. See <a href="facebook-app-setup.html">Facebook app setup</a>.</p>
  </div>
  <div class="card">
    <div class="card__head">
      <span class="card__chip">GCP</span>
      <span class="card__title">Google Cloud project</span>
    </div>
    <p class="card__desc">For Google OAuth sign&#8209;in. Just an OAuth 2.0 client ID — no service account needed.</p>
  </div>
  <div class="card">
    <div class="card__head">
      <span class="card__chip">DSC</span>
      <span class="card__title">Discord application</span>
    </div>
    <p class="card__desc">For the relay bot and slash commands. <a href="discord.html">Discord integration</a> covers the bot setup.</p>
  </div>
  <div class="card">
    <div class="card__head">
      <span class="card__chip">RZP</span>
      <span class="card__title">Razorpay account <span class="pill" style="margin-left:6px">optional</span></span>
    </div>
    <p class="card__desc">Only if you want to sell paid credit packs. The hosted service uses this; self&#8209;hosters often skip it.</p>
  </div>
  <div class="card">
    <div class="card__head">
      <span class="card__chip">NIX</span>
      <span class="card__title">Nix &amp; direnv</span>
    </div>
    <p class="card__desc">The dev shell pins <code>cargo</code>, <code>wrangler</code>, and the WASM toolchain. <code>nix develop</code> drops you in.</p>
  </div>
</div>

<h2 id="walkthrough">Step-by-step</h2>

<ol class="steps">
  <li>
    <h3>Clone &amp; build</h3>
    <p>Get the source, drop into the dev shell, and run the test suite to confirm your toolchain works.</p>
    <div class="codeblock">
      <div class="codeblock__head">
        <span class="codeblock__lang">bash</span>
        <span class="codeblock__file">terminal</span>
        <button class="codeblock__copy">Copy</button>
      </div>
<pre><code><span class="tok-k">git</span> clone <span class="tok-s">https://github.com/ananthb/concierge</span>
<span class="tok-k">cd</span> concierge
<span class="tok-k">nix</span> develop
<span class="tok-k">cargo</span> test</code></pre>
    </div>
  </li>

  <li>
    <h3>Create Cloudflare resources</h3>
    <p>Provision a D1 database, a KV namespace, and the two queues that back the persona safety classifier. Note the IDs that print &mdash; you&rsquo;ll paste them into <code>wrangler.toml</code>.</p>
    <div class="codeblock">
      <div class="codeblock__head">
        <span class="codeblock__lang">bash</span>
        <span class="codeblock__file">terminal</span>
        <button class="codeblock__copy">Copy</button>
      </div>
<pre><code><span class="tok-c"># SQLite database</span>
<span class="tok-k">wrangler</span> d1 create concierge
<!-- -->
<span class="tok-c"># Config / session / token / persona store</span>
<span class="tok-k">wrangler</span> kv namespace create KV
<!-- -->
<span class="tok-c"># Persona safety classifier queue + DLQ</span>
<span class="tok-k">wrangler</span> queues create concierge-safety
<span class="tok-k">wrangler</span> queues create concierge-safety-dlq</code></pre>
    </div>
    <div class="callout callout--tip">
      <div class="callout__icon">→</div>
      <div class="callout__body">
        <strong>Update wrangler.toml</strong>
        <p>Paste the printed IDs into <code>[[d1_databases]]</code> and <code>[[kv_namespaces]]</code>. The <code>EMAIL</code>, <code>AI</code>, and queue producer / consumer bindings are already declared.</p>
      </div>
    </div>
    <p class="muted">If you skip the queues, persona safety checks stay in <em>Pending</em> forever and AI replies are blocked tenant&#8209;wide. Static / canned replies still work.</p>
  </li>

  <li>
    <h3>Apply database migrations</h3>
    <p>The Worker carries its schema in <code>migrations/*.sql</code>. <code>wrangler d1 migrations apply</code> tracks what&rsquo;s already been run and applies the rest.</p>
    <div class="codeblock">
      <div class="codeblock__head">
        <span class="codeblock__lang">bash</span>
        <span class="codeblock__file">terminal</span>
        <button class="codeblock__copy">Copy</button>
      </div>
<pre><code><span class="tok-k">wrangler</span> d1 migrations apply concierge <span class="tok-v">--remote</span></code></pre>
    </div>
    <p>This is idempotent &mdash; safe to re&#8209;run on every deploy.</p>
  </li>

  <li>
    <h3>Set secrets</h3>
    <p>Most of your real configuration lives in Worker secrets. The full reference is on the <a href="configuration.html">configuration page</a>; the minimum to deploy is below.</p>
    <div class="codeblock">
      <div class="codeblock__head">
        <span class="codeblock__lang">bash</span>
        <span class="codeblock__file">at minimum</span>
        <button class="codeblock__copy">Copy</button>
      </div>
<pre><code><span class="tok-c"># Core encryption — generate with: openssl rand -hex 32</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">ENCRYPTION_KEY</span>
<!-- -->
<span class="tok-c"># Google OAuth (sign-in)</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">GOOGLE_OAUTH_CLIENT_ID</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">GOOGLE_OAUTH_CLIENT_SECRET</span>
<!-- -->
<span class="tok-c"># Meta — Facebook / Instagram / WhatsApp</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">META_APP_ID</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">META_APP_SECRET</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">WHATSAPP_ACCESS_TOKEN</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">WHATSAPP_VERIFY_TOKEN</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">INSTAGRAM_VERIFY_TOKEN</span>
<!-- -->
<span class="tok-c"># Discord</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">DISCORD_PUBLIC_KEY</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">DISCORD_APPLICATION_ID</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">DISCORD_BOT_TOKEN</span>
<!-- -->
<span class="tok-c"># Razorpay — only if billing is enabled</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">RAZORPAY_KEY_ID</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">RAZORPAY_KEY_SECRET</span>
<span class="tok-k">wrangler</span> secret put <span class="tok-v">RAZORPAY_WEBHOOK_SECRET</span></code></pre>
    </div>
  </li>

  <li>
    <h3>Configure <code>wrangler.toml</code> variables</h3>
    <p>Plain&#8209;text vars belong in the <code>[vars]</code> block. These are the ones to set:</p>
    <div class="table-wrap">
      <table class="docs">
        <thead><tr><th>Variable</th><th>What it&rsquo;s for</th></tr></thead>
        <tbody>
          <tr><td><code>CF_ACCESS_TEAM</code></td><td class="muted">Your Cloudflare Access team name (the subdomain in <code>myteam.cloudflareaccess.com</code>).</td></tr>
          <tr><td><code>CF_ACCESS_AUD</code></td><td class="muted">Application Audience (AUD) tag from your Access app for <code>/manage/*</code>.</td></tr>
          <tr><td><code>WHATSAPP_WABA_ID</code></td><td class="muted">Your WhatsApp Business Account ID.</td></tr>
          <tr><td><code>WHATSAPP_SIGNUP_CONFIG_ID</code></td><td class="muted">Meta Embedded Signup configuration ID.</td></tr>
          <tr><td><code>EMAIL_BASE_DOMAIN</code></td><td class="muted">The single domain you own that hosts catch&#8209;all email subdomains (default <code>cncg.email</code>).</td></tr>
        </tbody>
      </table>
    </div>
  </li>

  <li>
    <h3>Deploy</h3>
    <p>One command. The first deploy compiles to WASM, uploads, and prints your Worker URL.</p>
    <div class="codeblock">
      <div class="codeblock__head">
        <span class="codeblock__lang">bash</span>
        <span class="codeblock__file">terminal</span>
        <button class="codeblock__copy">Copy</button>
      </div>
<pre><code><span class="tok-k">wrangler</span> deploy</code></pre>
    </div>
  </li>

  <li>
    <h3>Configure Meta webhooks</h3>
    <p>In the Meta Developer Console, point both products at your Worker. Subscribe to the <code>messages</code> field on each.</p>
    <div class="aside-block">
      <div class="aside-block__title">Webhook URLs</div>
      <dl>
        <dt>WhatsApp</dt><dd><code>https://your-domain/webhook/whatsapp</code></dd>
        <dt>Instagram</dt><dd><code>https://your-domain/webhook/instagram</code></dd>
      </dl>
    </div>
    <p>The verify tokens you set in step&nbsp;4 are echoed back during webhook setup. See <a href="facebook-app-setup.html">Facebook app setup</a> for screenshots.</p>
  </li>

  <li>
    <h3>Set up the Discord bot</h3>
    <ol>
      <li>Create a Discord application at <a href="https://discord.com/developers/applications">discord.com/developers</a>.</li>
      <li>Set the <strong>Interactions Endpoint URL</strong> to <code>https://your-domain/discord/interactions</code>.</li>
      <li>Create a bot user and copy the token into <code>DISCORD_BOT_TOKEN</code>.</li>
      <li>Register the slash commands <code>/status</code>, <code>/domains</code>, <code>/rules</code> via the Discord API.</li>
      <li>Invite the bot to your server with the appropriate permissions.</li>
    </ol>
    <p>Detailed walkthrough on the <a href="discord.html">Discord integration page</a>.</p>
  </li>

  <li>
    <h3>Set up email routing</h3>
    <p>Cloudflare Email Routing handles inbound mail for the domain in <code>EMAIL_BASE_DOMAIN</code>. To finish:</p>
    <ol>
      <li>In the Cloudflare dashboard, open <strong>Email Routing</strong> for your email domain.</li>
      <li>Add a <strong>catch&#8209;all</strong> routing rule: <em>Action</em> → <strong>Send to a Worker</strong> → select <code>concierge</code>.</li>
      <li>Set the DNS API token if you intend to provision tenant subdomains automatically: <code>wrangler secret put CF_DNS_API_TOKEN</code> (a CF API token with DNS Edit on the email zone).</li>
    </ol>
    <p>Tenant subdomains (e.g.&nbsp;<code>acme.cncg.email</code>) are provisioned via the admin panel; MX records are auto&#8209;discovered from the apex domain. See <a href="email-routing.html">Email routing</a>.</p>
  </li>

  <li>
    <h3>Set up Cloudflare Access for management</h3>
    <ol>
      <li>In the Cloudflare dashboard, go to <strong>Zero Trust → Access → Applications</strong>.</li>
      <li>Create an application for <code>your-domain/manage/*</code>.</li>
      <li>Copy the <strong>Application Audience (AUD) Tag</strong> and set it as <code>CF_ACCESS_AUD</code> in <code>wrangler.toml</code>.</li>
      <li>Add a policy allowing the emails that should have management access.</li>
    </ol>
    <p>See the <a href="management.html">Management panel</a> page.</p>
  </li>

  <li>
    <h3>Set up Razorpay <span class="pill">optional</span></h3>
    <ol>
      <li>Create a Razorpay account and grab your API keys.</li>
      <li>Set up a webhook at <code>https://your-domain/webhook/razorpay</code> subscribing to <code>payment.captured</code>, <code>payment.failed</code>, <code>subscription.activated</code>, <code>subscription.charged</code>, <code>subscription.halted</code>, <code>subscription.cancelled</code>.</li>
      <li>Set the webhook secret and API keys as Worker secrets.</li>
    </ol>
    <p>See <a href="billing.html">Billing</a> for the credit&#8209;pack model.</p>
  </li>
</ol>

<div class="callout callout--tip">
  <div class="callout__icon">✓</div>
  <div class="callout__body">
    <strong>You&rsquo;re done.</strong>
    <p>Sign in at <code>https://your-domain/</code> to walk through the in&#8209;app onboarding wizard, then send a test message to your WhatsApp number to confirm the round&#8209;trip.</p>
  </div>
</div>
