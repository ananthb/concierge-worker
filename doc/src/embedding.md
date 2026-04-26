

<div class="page-eyebrow">Operate</div>
<h1 class="page-title">Embedding <em>lead forms</em>.</h1>


<p>Lead capture forms can be embedded on any website using an iframe.</p>

<h2>Basic Embed</h2>

<pre><code>&lt;iframe src="https://your-domain/lead/{form_id}/{slug}"
  width="400" height="200" frameborder="0"&gt;&lt;/iframe&gt;</code></pre>

<p>Get the exact snippet from <strong>Admin &gt; Lead Forms &gt; [your form] &gt; Embed Code</strong>.</p>

<h2>Responsive Sizing</h2>

<pre><code>&lt;iframe src="https://your-domain/lead/{form_id}/{slug}"
  style="width: 100%; max-width: 400px; height: 200px; border: none;"&gt;&lt;/iframe&gt;</code></pre>

<h2>Allowed Origins</h2>

<p>To restrict which sites can embed your form, configure <strong>Allowed Origins</strong> in the form settings. Add one origin per line (e.g., <code>https://example.com</code>). If left empty, any site can embed the form.</p>

<h2>Custom Styling</h2>

<p>Use the style fields in the form editor to match your site's design. For advanced customization, use the <strong>Custom CSS</strong> field.</p>
