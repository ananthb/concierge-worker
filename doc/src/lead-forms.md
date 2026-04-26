

<div class="page-eyebrow">Channel</div>
<h1 class="page-title">Lead capture <em>forms</em>.</h1>


<p>Lead forms are embeddable phone number capture widgets. When someone submits their phone number, they receive a WhatsApp message immediately.</p>

<h2>Creating a Form</h2>

<ol>
  <li>Go to <strong>Admin &gt; Lead Forms &gt; New Form</strong></li>
  <li>Configure:
    <ul>
<li><strong>Name</strong>: displayed as the form heading</li>
<li><strong>WhatsApp Account</strong>: which number sends the reply</li>
<li><strong>Reply Mode</strong>: Static or AI</li>
<li><strong>Reply Prompt</strong>: the message text or AI prompt</li>
<li><strong>Style</strong>: colors, button text, placeholder, success message, custom CSS</li>
<li><strong>Allowed Origins</strong>: restrict which domains can embed the form (empty = all)</li>
    </ul>
  </li>
  <li>Copy the embed code from the form's edit page</li>
</ol>

<h2>Embedding</h2>

<p>Use the iframe snippet from the edit page:</p>

<pre><code>&lt;iframe src="https://your-domain/lead/{form_id}/{slug}"
  width="400" height="200" frameborder="0"&gt;&lt;/iframe&gt;</code></pre>

<p>The form submits via HTMX within the iframe: no page navigation.</p>

<h2>CORS</h2>

<p>If you set allowed origins, only those domains can embed and submit the form. Leave empty to allow embedding from any site.</p>

<h2>Styling</h2>

<p>Customize the form appearance with:</p>

<ul>
  <li>Primary color, text color, background color</li>
  <li>Border radius</li>
  <li>Button text and placeholder text</li>
  <li>Success message shown after submission</li>
  <li>Custom CSS (injected into the form's <code>&lt;style&gt;</code> tag)</li>
</ul>
