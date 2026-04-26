

<div class="page-eyebrow">Architecture</div>
<h1 class="page-title">How it <em>works</em>.</h1>
<p class="page-lede">
  Concierge runs as a single Cloudflare Worker that handles four messaging channels through one unified pipeline. Every inbound event &mdash; a WhatsApp message, an Instagram DM, an email arriving at your catch&#8209;all, or a lead form submission &mdash; is normalized into the same shape and processed by the same steps.
</p>

<!-- HERO PIPELINE -->
<div class="hero">
  <div class="hero__caption">The pipeline · runs identically for every channel</div>
  <div class="pipeline">
    <div class="pipeline__col">
      <div class="pipeline__col-label">Inbound</div>
      <div class="pipeline__node">
        <span class="pipeline__node-dot wa"></span>
        <span class="pipeline__node-text"><b>WhatsApp</b><span>webhook</span></span>
      </div>
      <div class="pipeline__node">
        <span class="pipeline__node-dot ig"></span>
        <span class="pipeline__node-text"><b>Instagram</b><span>webhook</span></span>
      </div>
      <div class="pipeline__node">
        <span class="pipeline__node-dot em"></span>
        <span class="pipeline__node-text"><b>Email</b><span>email() handler</span></span>
      </div>
      <div class="pipeline__node">
        <span class="pipeline__node-dot lf"></span>
        <span class="pipeline__node-text"><b>Lead form</b><span>POST /lead/&hellip;</span></span>
      </div>
    </div>

    <div class="pipeline__connector">
      <div class="pipeline__connector-line"></div>
      <div class="pipeline__connector-label">Normalize</div>
    </div>

    <div class="pipeline__core">
      <div class="pipeline__core-title">Common pipeline</div>
      <div class="pipeline__core-step"><span class="num">01</span><span><b>Normalize</b> → <em class="serif">InboundMessage</em></span></div>
      <div class="pipeline__core-step"><span class="num">02</span><span><b>Tenant</b> + <b>credit</b> check</span></div>
      <div class="pipeline__core-step"><span class="num">03</span><span><b>Reply rules</b> · keyword + embedding</span></div>
      <div class="pipeline__core-step"><span class="num">04</span><span><b>Persona</b> prompt + action dispatch</span></div>
      <div class="pipeline__core-step"><span class="num">05</span><span><b>Log metadata</b> &middot; D1</span></div>
    </div>

    <div class="pipeline__connector">
      <div class="pipeline__connector-line"></div>
      <div class="pipeline__connector-label">Dispatch</div>
    </div>

    <div class="pipeline__col">
      <div class="pipeline__col-label">Actions</div>
      <div class="pipeline__action"><b>Canned reply</b><span>Static text · no AI · free</span></div>
      <div class="pipeline__action"><b>AI reply</b><span>Workers AI · llama&#8209;4&#8209;scout · persona + rule prompt</span></div>
      <div class="pipeline__action"><b>Forward → Discord</b><span>Embed + Reply / Approve / Drop</span></div>
      <div class="pipeline__action"><b>Forward email</b><span>Reverse&#8209;alias for replies</span></div>
      <div class="pipeline__action"><b>Drop / spam reject</b><span>Silent or NDR</span></div>
    </div>
  </div>
</div>

<h2 id="pipeline">The unified pipeline</h2>
<p>
  Regardless of which channel a message arrives on, it&rsquo;s normalized into an <code>InboundMessage</code> struct &mdash; channel, sender, recipient, tenant, metadata &mdash; and processed through the same steps. This is the spine of the codebase. Channel handlers exist only to translate webhooks into <code>InboundMessage</code> and to dispatch outbound replies.
</p>

<ol class="steps">
  <li>
    <h3>Channel handler receives the raw event</h3>
    <p>WhatsApp and Instagram POST signed webhook payloads to <code>/webhook/*</code>. Email arrives via Cloudflare&rsquo;s <code>send_to_worker</code> action, invoking the Worker&rsquo;s <code>email()</code> entrypoint. Lead forms POST to <code>/lead/{slug}</code>.</p>
  </li>
  <li>
    <h3>Normalize into <code>InboundMessage</code></h3>
    <p>Channel, sender, recipient, tenant ID, and any channel&#8209;specific metadata. From here, every code path is identical.</p>
  </li>
  <li>
    <h3>Log metadata to <code>messages</code></h3>
    <p>An append&#8209;only row in D1: <em>channel, direction, sender ID, recipient ID, tenant, timestamp</em>. Message <strong>content</strong> is never persisted &mdash; only the fact that something happened, with whom.</p>
  </li>
  <li>
    <h3>Reply rules evaluate in order</h3>
    <p>Each channel carries an ordered <code>ReplyConfig</code>. Rules pair a matcher (case&#8209;insensitive keyword substring or BGE&#8209;embedding cosine similarity over a user&#8209;written intent description) with a response (canned text or an AI prompt). First match wins; the mandatory default rule fires if nothing else does.</p>
  </li>
  <li>
    <h3>Action dispatches &middot; canned text or LLM call</h3>
    <p>Canned responses send verbatim, no credit charge. Prompt responses concatenate the tenant&rsquo;s <em>persona prompt</em> with the rule&rsquo;s prompt and run the main reply model; one credit is deducted before the call (optimistic) and restored if generation or send fails. AI replies are blocked unless the persona&rsquo;s asynchronous safety check has approved the current prompt.</p>
  </li>
</ol>

<div class="callout callout--note">
  <div class="callout__icon">i</div>
  <div class="callout__body">
    <strong>Privacy note</strong>
    <p>The unified <code>messages</code> table stores <em>only</em> metadata: channel, direction, sender ID, recipient ID, tenant, timestamp. No subjects, no bodies, no attachments. AI replies are generated synchronously from in&#8209;memory data and discarded.</p>
  </div>
</div>

<h2 id="whatsapp">WhatsApp / Instagram auto&#8209;reply</h2>
<p>Both Meta channels run through the same reply pipeline:</p>
<ol>
  <li>Meta delivers the inbound message to <code>POST /webhook/whatsapp</code> or <code>POST /webhook/instagram</code>.</li>
  <li>Concierge looks up the channel account (phone number ID for WhatsApp, page ID for Instagram) and its <code>ReplyConfig</code>.</li>
  <li>The body is truncated to 1000 chars and run past a fast prompt&#8209;injection scanner; injection attempts are dropped.</li>
  <li>If any rule has a <em>Prompt</em> matcher, the inbound message is embedded once and compared via cosine similarity to each rule&rsquo;s precomputed embedding.</li>
  <li>Rules are walked in order; the first match wins. Otherwise the mandatory default rule fires.</li>
  <li>Canned responses send verbatim with no credit charge. Prompt responses combine persona + rule prompt + a context block, deduct one credit, and run the main LLM. AI replies require the tenant&rsquo;s persona to be safety&#8209;<em>Approved</em>.</li>
</ol>

<h2 id="persona">Persona &amp; safety check</h2>
<ol>
  <li>The tenant picks a curated preset, fills in the builder (tone / catch&#8209;phrases / off&#8209;topic boundaries), or writes a raw custom prompt at <code>/admin/persona</code>.</li>
  <li>On save, the active prompt is hashed; if the hash differs from the last&#8209;vetted hash, status flips to <em>Pending</em> and a <code>SafetyJob</code> is enqueued onto Cloudflare Queue <code>concierge-safety</code>.</li>
  <li>The queue consumer reads the job, re&#8209;checks the hash (drops stale jobs), and runs the safety classifier with Calculon Tech&rsquo;s content policy.</li>
  <li>The result lands back in KV as <em>Approved</em> or <em>Rejected</em> with a vague user&#8209;facing reason.</li>
  <li>While <em>Pending</em> or <em>Rejected</em>, AI replies are blocked tenant&#8209;wide; canned default replies still send.</li>
</ol>

<h2 id="email">Email routing</h2>
<ol>
  <li>An email arrives at your catch&#8209;all domain (configured via Cloudflare Email Routing).</li>
  <li>Cloudflare triggers the Worker&rsquo;s <code>email()</code> handler.</li>
  <li>Concierge extracts the domain, looks up the tenant, and parses the MIME message.</li>
  <li>Routing rules are evaluated in priority order using glob&#8209;pattern matching on <code>from</code>, <code>to</code>, <code>subject</code>, <code>body</code>, and <code>has_attachment</code>.</li>
  <li>The matched rule&rsquo;s action executes: <em>drop</em>, <em>spam reject</em>, <em>forward email</em>, <em>forward to Discord</em>, or <em>AI reply with approval</em>.</li>
  <li>For email forwarding, a reverse&#8209;alias address is generated so replies route back through Concierge to the original sender.</li>
</ol>

<div class="aside-block">
  <div class="aside-block__title">Glob semantics &nbsp;<span class="pill"><span class="pill__dot"></span>last match wins</span></div>
  <dl>
    <dt><code>*</code></dt><dd>any sequence of characters, zero or more</dd>
    <dt><code>?</code></dt><dd>exactly one character</dd>
    <dt>case</dt><dd>matching is case&#8209;insensitive</dd>
    <dt>combine</dt><dd>all non&#8209;<em>None</em> criteria are AND&#8209;ed (from + to + subject + body + has_attachment)</dd>
    <dt>order</dt><dd>rules are sorted by ascending priority; the highest priority match wins</dd>
  </dl>
</div>

<h2 id="discord">Discord relay</h2>
<ol>
  <li>When a message from any channel is forwarded to Discord (via email routing rules or future direct integrations), it arrives as an embed with <span class="kbd">Reply</span>, <span class="kbd">Approve</span>, and <span class="kbd">Drop</span> buttons.</li>
  <li>A <code>ConversationContext</code> is saved in KV, linking the Discord message to the original channel, sender, and reply metadata.</li>
  <li>When someone clicks <strong>Reply</strong>, a modal opens for composing a response.</li>
  <li>The reply is sent back through the originating channel using the stored context.</li>
  <li>For AI&#8209;generated drafts, <strong>Approve</strong> sends the draft and <strong>Drop</strong> discards it.</li>
</ol>

<h2 id="lead-forms">Lead capture forms</h2>
<ol>
  <li>You create a lead form in the admin and embed it on your website via iframe.</li>
  <li>A visitor enters their phone number and submits.</li>
  <li>Concierge generates a message (canned or AI&#8209;prompt) and sends it via WhatsApp.</li>
  <li>The submission metadata is logged to the database.</li>
</ol>

<h2 id="billing">Billing</h2>
<p>
  Each AI&#8209;mode reply (rule with a <em>Prompt</em> response) deducts one credit from the tenant&rsquo;s balance. Canned replies, embedding lookups, intent classification, and persona safety checks are free. Credits are deducted <em>before</em> the AI call (optimistic deduction) and restored if generation or send fails. When credits reach zero, AI replies stop; canned defaults still send. Credits can be granted by management or purchased via Razorpay.
</p>

<h2 id="platform-model">Platform model</h2>
<div class="table-wrap">
  <div class="table-wrap__head">
    <b>Per-channel architecture</b><span>How each channel attaches to a tenant</span>
  </div>
  <table class="docs">
    <thead><tr><th>Channel</th><th>Model</th><th>Token storage</th></tr></thead>
    <tbody>
      <tr>
        <td><strong>WhatsApp</strong></td>
        <td><span class="pill pill--accent"><span class="pill__dot"></span>Shared WABA</span> — you own one WABA, customers add numbers via Meta Embedded Signup</td>
        <td>Single platform token <code>WHATSAPP_ACCESS_TOKEN</code></td>
      </tr>
      <tr>
        <td><strong>Instagram</strong></td>
        <td><span class="pill"><span class="pill__dot"></span>Per-account OAuth</span> — Facebook Login, page tokens per customer</td>
        <td>Encrypted in KV, rotated daily by cron</td>
      </tr>
      <tr>
        <td><strong>Email</strong></td>
        <td><span class="pill"><span class="pill__dot"></span>Per-domain</span> — each tenant registers domains and creates rules</td>
        <td>No tokens; Cloudflare Email Routing dispatches</td>
      </tr>
      <tr>
        <td><strong>Discord</strong></td>
        <td><span class="pill"><span class="pill__dot"></span>Guild → tenant</span> — each Discord server is linked to one tenant</td>
        <td>Shared bot token (<code>DISCORD_BOT_TOKEN</code> env secret)</td>
      </tr>
    </tbody>
  </table>
</div>

<h2 id="architecture">Architecture</h2>
<ul>
  <li><strong>Cloudflare Worker</strong> — Rust compiled to WebAssembly. Handles all HTTP routes, webhooks, and email events.</li>
  <li><strong>Cloudflare KV</strong> — tenant configs, account configs, tokens, sessions, routing rules, billing state, conversation contexts, persona.</li>
  <li><strong>Cloudflare D1</strong> — SQLite for message metadata, email metrics, lead form submissions, credit packs, payments, audit logs.</li>
  <li><strong>Cloudflare Workers AI</strong> — reply generation, prompt&#8209;injection scanning, persona safety classification, BGE embeddings.</li>
  <li><strong>Cloudflare Queues</strong> — persona safety classifier (<code>concierge-safety</code> + <code>concierge-safety-dlq</code>).</li>
  <li><strong>Cloudflare Email Routing</strong> — triggers the Worker&rsquo;s email handler for inbound emails.</li>
  <li><strong>Discord Interactions API</strong> — slash commands, button interactions, modal submissions via <code>POST /discord/interactions</code>.</li>
  <li><strong>Razorpay</strong> — payment processing for credit pack purchases.</li>
</ul>

<div class="codeblock">
  <div class="codeblock__head">
    <span class="codeblock__lang">rust</span>
    <span class="codeblock__file">src/types.rs</span>
    <button class="codeblock__copy">Copy</button>
  </div>
<pre><code><span class="tok-c">/// The normalized form every channel produces. Channel handlers</span>
<span class="tok-c">/// exist only to translate webhooks into this struct.</span>
<span class="tok-k">pub struct</span> <span class="tok-f">InboundMessage</span> {
    <span class="tok-k">pub</span> id:                 <span class="tok-v">String</span>,
    <span class="tok-k">pub</span> channel:            Channel,        <span class="tok-c">// WhatsApp | Instagram | Email | Discord</span>
    <span class="tok-k">pub</span> sender:             <span class="tok-v">String</span>,
    <span class="tok-k">pub</span> sender_name:        <span class="tok-v">Option</span>&lt;<span class="tok-v">String</span>&gt;,
    <span class="tok-k">pub</span> recipient:          <span class="tok-v">String</span>,
    <span class="tok-k">pub</span> body:               <span class="tok-v">String</span>,         <span class="tok-c">// in&#8209;memory only, never persisted</span>
    <span class="tok-k">pub</span> tenant_id:          <span class="tok-v">String</span>,
    <span class="tok-k">pub</span> channel_account_id: <span class="tok-v">String</span>,
    <span class="tok-k">pub</span> raw_metadata:       <span class="tok-v">Value</span>,
}</code></pre>
</div>
