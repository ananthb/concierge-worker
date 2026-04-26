

<div class="page-eyebrow">Operate</div>
<h1 class="page-title">Facebook <em>app setup</em>.</h1>


<p>Step-by-step guide to configuring your Meta Developer app for Concierge.</p>

<h2>1. Create a Facebook App</h2>

<ol>
  <li>Go to <a href="https://developers.facebook.com/apps/">developers.facebook.com/apps</a></li>
  <li>Click <strong>Create App</strong> &gt; choose <strong>Business</strong> type</li>
  <li>Enter app name and contact email</li>
</ol>

<h2>2. Basic Settings</h2>

<p>Go to <strong>Settings &gt; Basic</strong>:</p>

<ul>
  <li><strong>App Domains</strong>: <code>your-domain</code></li>
  <li><strong>Privacy Policy URL</strong>: <code>https://your-domain/privacy</code></li>
  <li><strong>Terms of Service URL</strong>: <code>https://your-domain/terms</code></li>
  <li><strong>User Data Deletion</strong>: Callback URL <code>https://your-domain/data-deletion</code>, select "Data Deletion Callback URL"</li>
</ul>

<h2>3. Facebook Login</h2>

<ol>
  <li>Add the <strong>Facebook Login</strong> product</li>
  <li>Go to <strong>Facebook Login &gt; Settings</strong></li>
  <li>Add Valid OAuth Redirect URIs:
    <ul>
<li><code>https://your-domain/auth/facebook/callback</code></li>
<li><code>https://your-domain/instagram/callback</code></li>
    </ul>
  </li>
</ol>

<h2>4. WhatsApp</h2>

<ol>
  <li>Add the <strong>WhatsApp</strong> product</li>
  <li>Go to <strong>WhatsApp &gt; API Setup</strong>: note your WABA ID</li>
  <li>Go to <strong>WhatsApp &gt; Configuration</strong>:
    <ul>
<li>Callback URL: <code>https://your-domain/webhook/whatsapp</code></li>
<li>Verify token: your <code>WHATSAPP_VERIFY_TOKEN</code> value</li>
<li>Subscribe to: <code>messages</code></li>
    </ul>
  </li>
</ol>

<h3>Embedded Signup</h3>

<ol>
  <li>Go to <strong>WhatsApp &gt; Embedded Signup</strong></li>
  <li>Create a configuration</li>
  <li>Copy the Config ID &gt; set as <code>WHATSAPP_SIGNUP_CONFIG_ID</code> in <code>wrangler.toml</code></li>
</ol>

<h2>5. Instagram Webhooks</h2>

<ol>
  <li>Go to <strong>Webhooks</strong> in the left sidebar</li>
  <li>Select <strong>Instagram</strong> from the dropdown</li>
  <li>Click <strong>Subscribe to this object</strong>
    <ul>
<li>Callback URL: <code>https://your-domain/webhook/instagram</code></li>
<li>Verify token: your <code>INSTAGRAM_VERIFY_TOKEN</code> value</li>
    </ul>
  </li>
  <li>Subscribe to the <code>messages</code> field</li>
</ol>

<h2>6. App Review</h2>

<p>Request these permissions:</p>

<table>
  <thead>
    <tr><th>Permission</th><th>Purpose</th></tr>
  </thead>
  <tbody>
    <tr><td><code>email</code></td><td>Sign-in (auto-approved)</td></tr>
    <tr><td><code>instagram_basic</code></td><td>Read Instagram account info</td></tr>
    <tr><td><code>instagram_manage_messages</code></td><td>Send/receive Instagram DMs</td></tr>
    <tr><td><code>pages_manage_metadata</code></td><td>Discover Instagram business accounts</td></tr>
    <tr><td><code>pages_messaging</code></td><td>Send DMs via Facebook Pages</td></tr>
    <tr><td><code>whatsapp_business_management</code></td><td>WhatsApp Embedded Signup</td></tr>
    <tr><td><code>whatsapp_business_messaging</code></td><td>Send WhatsApp messages</td></tr>
  </tbody>
</table>

<p>For each permission, Meta requires:</p>

<ul>
  <li>A screencast (30-60 seconds) showing the feature in action</li>
  <li>A description of the use case</li>
  <li>Your privacy policy URL</li>
</ul>

<p><strong>Tip</strong>: Before App Review is complete, add yourself as a tester at <strong>App Roles &gt; People</strong> to test all features in development mode.</p>
