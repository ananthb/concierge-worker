

<div class="page-eyebrow">Operate</div>
<h1 class="page-title">Management <em>panel</em>.</h1>


<p>The management panel at <code>/manage/*</code> is a platform operator dashboard for managing tenants, billing, and reviewing audit logs. It is protected by Cloudflare Access with full JWT verification.</p>

<h2>Authentication</h2>

<p>Management routes verify the <code>Cf-Access-Jwt-Assertion</code> header:</p>

<ol>
  <li><strong>Cloudflare Access</strong>: Handles the login flow and issues a signed JWT</li>
  <li><strong>JWT verification</strong>: Concierge fetches the team's JWKS, verifies the RS256 signature, checks the <code>aud</code> claim against <code>CF_ACCESS_AUD</code>, and extracts the email</li>
</ol>

<p>Anyone with a valid Access JWT (matching your Access policy) is authorized. Scope access via your Cloudflare Access policy.</p>

<h2>Dashboard</h2>

<p>The main dashboard (<code>/manage</code>) shows:</p>

<ul>
  <li>The authenticated operator's email</li>
  <li>Total tenant count</li>
  <li>Links to tenant management, billing, and audit log</li>
</ul>

<h2>Tenant Management</h2>

<p>At <code>/manage/tenants</code>, operators can:</p>

<ul>
  <li>List all tenants with their email, plan, and creation date</li>
  <li>View individual tenant details including connected accounts</li>
  <li>See a tenant's billing state (credits remaining, used, granted)</li>
  <li>Grant free reply credits to a tenant</li>
</ul>

<h2>Billing Controls</h2>

<p>At <code>/manage/billing</code>, operators can:</p>

<ul>
  <li><strong>View credit packs</strong>: See all defined packs with their reply counts, prices, and status</li>
  <li><strong>Create packs</strong>: Add new credit packs with name, reply count, INR price, USD price, and sort order</li>
  <li><strong>Update packs</strong>: Edit existing pack details</li>
  <li><strong>Activate/Deactivate packs</strong>: Control which packs are visible to tenants</li>
  <li><strong>Delete packs</strong>: Remove packs entirely</li>
  <li><strong>Grant credits</strong>: Give free reply credits to a specific tenant (at <code>/manage/billing/grant/{tenant_id}</code>)</li>
</ul>

<h2>Audit Log</h2>

<p>At <code>/manage/audit</code>, operators can review a chronological log of all management actions. Each entry records:</p>

<table>
  <thead>
    <tr><th>Field</th><th>Description</th></tr>
  </thead>
  <tbody>
    <tr><td><code>actor_email</code></td><td>The operator who performed the action</td></tr>
    <tr><td><code>action</code></td><td>What was done (e.g., <code>grant_replies</code>, <code>create_pack</code>, <code>update_pack</code>, <code>delete_pack</code>)</td></tr>
    <tr><td><code>resource_type</code></td><td>The type of resource affected (e.g., <code>billing</code>, <code>credit_pack</code>)</td></tr>
    <tr><td><code>resource_id</code></td><td>The specific resource identifier (optional)</td></tr>
    <tr><td><code>details</code></td><td>JSON payload with action-specific details</td></tr>
    <tr><td><code>created_at</code></td><td>Timestamp</td></tr>
  </tbody>
</table>

<p>The audit log shows the most recent 100 entries.</p>

<h2>Setup</h2>

<ol>
  <li>In the Cloudflare dashboard, go to <strong>Zero Trust &gt; Access &gt; Applications</strong></li>
  <li>Create a Self-hosted application for <code>your-domain/manage/*</code></li>
  <li>Copy the <strong>Application Audience (AUD) Tag</strong> from the application overview</li>
  <li>Set <code>CF_ACCESS_TEAM</code> (your team name) and <code>CF_ACCESS_AUD</code> in <code>wrangler.toml</code></li>
  <li>Add a policy allowing the emails that should have management access</li>
  <li>Deploy with <code>wrangler deploy</code></li>
</ol>
