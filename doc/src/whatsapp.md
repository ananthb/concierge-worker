

<div class="page-eyebrow">Channel · 1 of 4</div>
<h1 class="page-title">WhatsApp <em>auto&#8209;reply</em>.</h1>
<p class="page-lede">
  WhatsApp is one of four channels in the unified messaging pipeline. Incoming WhatsApp messages are normalized into <code>InboundMessage</code> structs and processed through the common pipeline, which evaluates reply rules, generates the response, and logs metadata.
</p>

<!-- channel banner -->
<div class="channel-banner">
  <div class="channel-banner__icon">
    <svg width="30" height="30" viewBox="0 0 24 24" fill="currentColor"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884a9.825 9.825 0 0 1 6.992 2.898 9.825 9.825 0 0 1 2.892 6.994c-.003 5.45-4.437 9.884-9.888 9.884"/></svg>
  </div>
  <div>
    <div class="channel-banner__title">WhatsApp Business API</div>
    <div class="channel-banner__sub">Shared WABA · Embedded Signup · Workers AI llama&#8209;4&#8209;scout</div>
  </div>
  <div class="channel-banner__stats">
    <div class="channel-banner__stat">
      <div class="channel-banner__stat-label">Status</div>
      <div class="channel-banner__stat-value"><span class="pill pill--live"><span class="pill__dot"></span>Live</span></div>
    </div>
    <div class="channel-banner__stat">
      <div class="channel-banner__stat-label">Cost</div>
      <div class="channel-banner__stat-value">1 credit / AI</div>
    </div>
  </div>
</div>

<h2 id="add-number">Adding a WhatsApp number</h2>
<ol class="steps">
  <li>
    <h3>Open the admin</h3>
    <p>Navigate to <strong>Admin → WhatsApp Accounts → Connect WhatsApp Number</strong>.</p>
  </li>
  <li>
    <h3>Complete Embedded Signup</h3>
    <p>Meta&rsquo;s in&#8209;flow widget walks you through registering the phone number and attaching it to your shared WABA. The phone number and phone number ID are configured automatically when the flow returns.</p>
  </li>
  <li>
    <h3>Verify webhook</h3>
    <p>Concierge sends a test webhook to itself to confirm the round trip. If you see a green check on the account row, you&rsquo;re live.</p>
  </li>
</ol>

<div class="callout callout--tip">
  <div class="callout__icon">→</div>
  <div class="callout__body">
    <strong>Manual fallback</strong>
    <p>If you already know your phone number ID, use <code>/admin/whatsapp/manual</code> instead. Useful for migrating from another platform or when Embedded Signup isn&rsquo;t available in your region.</p>
  </div>
</div>

<h2 id="auto-reply">Configuring auto-reply</h2>
<p>Each WhatsApp account has its own <code>ReplyConfig</code> with two layers:</p>
<ul>
  <li><strong>Default reply</strong> — edited inline on the account&rsquo;s edit page. Sets the fallback when no rule matches. Pick canned text or AI prompt.</li>
  <li><strong>Reply rules</strong> — managed at <code>/admin/rules/whatsapp/{account_id}</code>. An ordered list checked top&#8209;down on every inbound message; first match wins.</li>
</ul>

<div class="table-wrap">
  <div class="table-wrap__head">
    <b>Account fields</b><span>WhatsAppAccount.auto_reply (ReplyConfig)</span>
  </div>
  <table class="docs">
    <thead><tr><th>Field</th><th>Type</th><th>Description</th></tr></thead>
    <tbody>
      <tr><td><code>enabled</code></td><td><code>bool</code></td><td class="muted">Master switch; turns auto-reply off without losing rules.</td></tr>
      <tr><td><code>rules</code></td><td><code>Vec&lt;ReplyRule&gt;</code></td><td class="muted">Ordered list. First match wins. Each rule pairs a matcher (keyword substring or BGE&#8209;embedding cosine similarity over a user&#8209;written intent description) with a response (canned text or AI prompt).</td></tr>
      <tr><td><code>default_rule</code></td><td><code>ReplyRule</code></td><td class="muted">Mandatory fallback. Fires when no rule in <code>rules</code> matches.</td></tr>
      <tr><td><code>wait_seconds</code></td><td><code>u32</code></td><td class="muted">0&ndash;30s buffer; lets quick&#8209;fire bursts collapse into one reply via the <code>REPLY_BUFFER</code> Durable Object.</td></tr>
    </tbody>
  </table>
</div>

<h3 id="canned">Canned response</h3>
<p>Sent verbatim. No AI call, no credit charge. Good for hours/closed/FAQ replies.</p>

<div class="codeblock">
  <div class="codeblock__head">
    <span class="codeblock__lang">json</span>
    <span class="codeblock__file">example · canned</span>
    <button class="codeblock__copy">Copy</button>
  </div>
<pre><code>{
  <span class="tok-s">"id"</span><span class="tok-p">:</span> <span class="tok-s">"hours"</span>,
  <span class="tok-s">"label"</span><span class="tok-p">:</span> <span class="tok-s">"Hours / location"</span>,
  <span class="tok-s">"matcher"</span><span class="tok-p">:</span> {
    <span class="tok-s">"kind"</span><span class="tok-p">:</span> <span class="tok-s">"static_text"</span>,
    <span class="tok-s">"keywords"</span><span class="tok-p">:</span> [<span class="tok-s">"hours"</span>, <span class="tok-s">"open"</span>, <span class="tok-s">"closed"</span>, <span class="tok-s">"address"</span>]
  },
  <span class="tok-s">"response"</span><span class="tok-p">:</span> {
    <span class="tok-s">"kind"</span><span class="tok-p">:</span> <span class="tok-s">"canned"</span>,
    <span class="tok-s">"text"</span><span class="tok-p">:</span> <span class="tok-s">"We're open 7am&ndash;7pm every day. Come say hi! &#9749;"</span>
  }
}</code></pre>
</div>

<h3 id="ai">AI prompt response</h3>
<p>The matched rule&rsquo;s prompt is concatenated to the tenant&rsquo;s <em>persona prompt</em> (set at <code>/admin/persona</code>) and sent to Workers AI as the system instruction. The sender&rsquo;s name and message are passed as user&#8209;message context. One credit deducts before the call.</p>

<div class="codeblock">
  <div class="codeblock__head">
    <span class="codeblock__lang">json</span>
    <span class="codeblock__file">example · prompt rule</span>
    <button class="codeblock__copy">Copy</button>
  </div>
<pre><code>{
  <span class="tok-s">"id"</span><span class="tok-p">:</span> <span class="tok-s">"booking"</span>,
  <span class="tok-s">"label"</span><span class="tok-p">:</span> <span class="tok-s">"Booking requests"</span>,
  <span class="tok-s">"matcher"</span><span class="tok-p">:</span> {
    <span class="tok-s">"kind"</span><span class="tok-p">:</span> <span class="tok-s">"prompt"</span>,
    <span class="tok-s">"description"</span><span class="tok-p">:</span> <span class="tok-s">"wants to book, reschedule, or check availability for an appointment"</span>,
    <span class="tok-s">"threshold"</span><span class="tok-p">:</span> <span class="tok-n">0.72</span>
  },
  <span class="tok-s">"response"</span><span class="tok-p">:</span> {
    <span class="tok-s">"kind"</span><span class="tok-p">:</span> <span class="tok-s">"prompt"</span>,
    <span class="tok-s">"text"</span><span class="tok-p">:</span> <span class="tok-s">"Ask which service and which day they prefer; mention a stylist will confirm shortly."</span>
  }
}</code></pre>
</div>

<div class="callout callout--note">
  <div class="callout__icon">i</div>
  <div class="callout__body">
    <strong>Models</strong>
    <p>Reply generation uses <code>@cf/meta/llama-4-scout-17b-16e-instruct</code>. Prompt&#8209;injection scanning + persona safety classification use the smaller <code>@cf/meta/llama-3.1-8b-instruct-fast</code>. Rule embeddings use <code>@cf/baai/bge-base-en-v1.5</code>. Latency is typically 600&ndash;1200ms; the customer sees it as &ldquo;typing&hellip;&rdquo; in WhatsApp.</p>
  </div>
</div>

<h2 id="persona">Persona</h2>
<p>
  The system prompt for every AI reply is the tenant&#8209;wide persona prompt at <code>/admin/persona</code> (preset, builder, or custom) concatenated with the matched rule&rsquo;s prompt. AI replies are blocked unless the persona&rsquo;s safety status is <em>Approved</em>; canned default replies still send.
</p>
<p>See <a href="how-it-works.html#persona">Persona &amp; safety check</a> for the async classifier flow.</p>

<h2 id="credits">Credit deduction</h2>
<p>
  Each AI&#8209;mode reply (rule with a <em>Prompt</em> response) deducts one credit from the tenant&rsquo;s balance. Canned responses are free. Credits are deducted <em>before</em> the AI call (optimistic) and restored if generation or send fails. When credits reach zero, AI replies stop silently &mdash; canned default replies still send.
</p>
<p>See <a href="billing.html">Billing</a> for managing credits.</p>

<div class="callout callout--warn">
  <div class="callout__icon">!</div>
  <div class="callout__body">
    <strong>Silent stop on zero</strong>
    <p>When you run out of credits, Concierge stops AI replies without notifying the customer. The inbound message is still logged. We do not send error messages to customers because that would itself cost money &mdash; and produce a bad impression.</p>
  </div>
</div>

<h2 id="model-platform">Platform model</h2>
<p>
  All WhatsApp numbers share a single platform token (<code>WHATSAPP_ACCESS_TOKEN</code>). This is a system user token for your WhatsApp Business Account. You don&rsquo;t need per&#8209;customer tokens. Customers add their phone numbers to <em>your</em> WABA via Meta&rsquo;s Embedded Signup flow.
</p>
<p>
  This means: one Meta app, one WABA, many phone numbers. The hosted service uses this same model &mdash; when you connect on <code>concierge.calculon.tech</code>, your number joins our WABA.
</p>

<h2 id="logging">Message logging</h2>
<p>
  Inbound and outbound message metadata (direction, sender ID, recipient ID, timestamp) is logged to the unified <code>messages</code> table. <strong>No message content is stored.</strong> The body lives in memory long enough to be passed to the AI and dispatched to the outbound API, then is dropped.
</p>

<div class="codeblock">
  <div class="codeblock__head">
    <span class="codeblock__lang">sql</span>
    <span class="codeblock__file">migrations/0001_create_schema.sql · partial</span>
    <button class="codeblock__copy">Copy</button>
  </div>
<pre><code><span class="tok-k">CREATE TABLE</span> messages (
  id          <span class="tok-k">INTEGER PRIMARY KEY AUTOINCREMENT</span>,
  tenant_id   <span class="tok-k">TEXT NOT NULL</span>,
  channel     <span class="tok-k">TEXT NOT NULL</span>,    <span class="tok-c">-- whatsapp | instagram | email | discord</span>
  direction   <span class="tok-k">TEXT NOT NULL</span>,    <span class="tok-c">-- inbound | outbound</span>
  sender_id   <span class="tok-k">TEXT NOT NULL</span>,
  recipient_id <span class="tok-k">TEXT NOT NULL</span>,
  created_at  <span class="tok-k">TEXT NOT NULL DEFAULT</span> (datetime(<span class="tok-s">'now'</span>))
  <span class="tok-c">-- intentionally no body, no subject, no attachments</span>
);</code></pre>
</div>

<h2 id="webhook">Webhook payload reference</h2>
<p>The Worker exposes the standard Meta WhatsApp webhook contract. <code>GET /webhook/whatsapp</code> handles verification (echoes <code>hub.challenge</code> when <code>hub.verify_token</code> matches <code>WHATSAPP_VERIFY_TOKEN</code>); <code>POST /webhook/whatsapp</code> ingests messages.</p>

<div class="aside-block">
  <div class="aside-block__title">Endpoints &nbsp;<span class="pill"><span class="pill__dot"></span>signed by Meta</span></div>
  <dl>
    <dt><code>GET</code></dt><dd><code>/webhook/whatsapp?hub.mode=subscribe&hellip;</code> &mdash; verification handshake</dd>
    <dt><code>POST</code></dt><dd><code>/webhook/whatsapp</code> &mdash; signed with <code>X-Hub-Signature-256</code></dd>
    <dt>verify</dt><dd>HMAC&#8209;SHA256 with <code>META_APP_SECRET</code> &middot; rejected if the signature is invalid</dd>
    <dt>subscribed</dt><dd><code>messages</code> field on the WhatsApp product</dd>
  </dl>
</div>
