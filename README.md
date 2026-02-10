# Concierge Worker

A Cloudflare Worker in Rust that provides forms, calendars, and event/venue booking.
Embed forms and calendars in your sites using HTMX or iframes.

## Features

### Forms
- **Dynamic Forms** - Create forms with custom fields via admin dashboard
- **Field Types** - Text, email, mobile, long text, file upload
- **Customizable Styling** - Full CSS variable control per form
- **File Uploads** - R2 storage with 2MB limit
- **Responders** - Send acknowledgement messages via Twilio (SMS, RCS, WhatsApp, Email) or Resend (Email)
- **Responses View** - View all form submissions in the admin dashboard
- **Digest Emails** - Receive daily or weekly email summaries of new submissions

### Calendars
- **Multiple Calendars** - Create and manage multiple independent calendars
- **Booking Links** - Public forms for users to book time slots
- **Booking Approval** - Auto-accept or require manual admin approval
- **View Links** - Embeddable calendar views (week, month, year, endless)
- **iCal Feeds** - Subscribe to calendars from other services
- **Instagram Integration** - Auto-import events from Instagram posts using AI

### General
- **HTMX + iframe Embedding** - Embed forms and calendars anywhere
- **Cloudflare Access** - Admin authentication via Cloudflare Zero Trust

## Prerequisites

- [Cloudflare account](https://dash.cloudflare.com/sign-up)
- [Nix](https://nixos.org/) with flakes enabled
- [direnv](https://direnv.net/) (optional, for automatic environment loading)

## Setup

### 1. Clone and enter dev environment

```bash
git clone https://github.com/YOUR_USERNAME/concierge-worker.git
cd concierge-worker

# Enter the development environment
nix develop

# Or if using direnv
direnv allow
```

### 2. Create Cloudflare resources

```bash
# Create D1 database
wrangler d1 create concierge-worker

# Create KV namespace
wrangler kv namespace create concierge-worker

# Create R2 bucket (for file uploads)
wrangler r2 bucket create concierge-worker-uploads
```

### 3. Update wrangler.toml

Replace the placeholder IDs with the values from the commands above:
- `database_id` - from `wrangler d1 create` output
- KV namespace `id` - from `wrangler kv namespace create` output

### 4. Run database migrations

```bash
wrangler d1 migrations apply concierge-worker
```

### 5. Deploy

**Option A: Deploy from local machine**

```bash
wrangler deploy
```

**Option B: Deploy via GitHub Actions**

1. Go to your GitHub repo Settings > Secrets and variables > Actions
2. Add a secret named `CLOUDFLARE_API_TOKEN` with a [Cloudflare API token](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/) that has Workers permissions
3. Push to `main` branch to trigger automatic deployment

### 6. Set up Cloudflare Access (Required)

The admin dashboard requires Cloudflare Access for authentication.

1. Go to [Cloudflare Zero Trust](https://one.dash.cloudflare.com/)
2. Navigate to **Access** > **Applications**
3. Click **Add an application** > **Self-hosted**
4. Configure:
   - **Application name**: Concierge Worker Admin
   - **Session duration**: 24 hours (or your preference)
   - **Application domain**: `your-worker.your-subdomain.workers.dev`
   - **Path**: `/admin`
5. Add a policy to control who can access (e.g., email ends with `@yourdomain.com`)
6. Save

For local development, set `ENVIRONMENT = "development"` in `wrangler.toml` to bypass Access.

## Local Development

```bash
# Run locally with simulated bindings
wrangler dev

# Run locally with real D1/KV/R2 (requires account)
wrangler dev --remote
```

## Auto-Responders

Automatically send acknowledgement messages when forms are submitted or bookings are made.

### Supported Channels

| Channel | Provider | Target Field Type |
|---------|----------|-------------------|
| Twilio SMS | Twilio | Mobile |
| Twilio RCS | Twilio | Mobile |
| Twilio WhatsApp | Twilio | Mobile |
| Twilio Email | SendGrid (via Twilio) | Email |
| Resend Email | Resend | Email |

### Configuration

All credentials are stored as Workers secrets. Set them with `wrangler secret put <NAME>`.

#### Twilio (SMS, RCS, WhatsApp)

```bash
wrangler secret put TWILIO_SID
wrangler secret put TWILIO_TOKEN
wrangler secret put TWILIO_FROM_SMS        # e.g., +15551234567
wrangler secret put TWILIO_FROM_WHATSAPP   # e.g., whatsapp:+15551234567
```

#### Twilio Email (via SendGrid)

```bash
wrangler secret put SENDGRID_API_KEY
wrangler secret put TWILIO_FROM_EMAIL      # e.g., noreply@yourdomain.com
```

#### Resend Email

```bash
wrangler secret put RESEND_API_KEY
wrangler secret put RESEND_FROM            # e.g., noreply@yourdomain.com
```

### Template Placeholders

Use `{{field_id}}` in your message body or subject to insert field values:

```
Hi {{name}},

Thank you for your booking on {{date}} at {{time}}.
```

## Instagram Integration

Connect Instagram accounts to automatically extract events from post captions using Cloudflare AI.

### 1. Create a Meta App

1. Go to [Meta for Developers](https://developers.facebook.com/) and create an account
2. Click **My Apps** > **Create App**
3. Select **Consumer** as the app type
4. Once created, go to **Add Products** and add **Instagram Basic Display**

### 2. Configure Instagram Basic Display

1. In your app dashboard, go to **Instagram Basic Display** > **Basic Display**
2. Click **Create New App**
3. Fill in the required fields:
   - **Valid OAuth Redirect URIs**: `https://your-worker.workers.dev/instagram/callback`
   - **Deauthorize Callback URL**: `https://your-worker.workers.dev/instagram/deauthorize`
   - **Data Deletion Request URL**: `https://your-worker.workers.dev/instagram/delete`
4. Note your **Instagram App ID** and **Instagram App Secret**

### 3. Add Test Users (Development Mode)

While your app is in development mode:

1. Go to **Roles** > **Roles**
2. Click **Add Instagram Testers**
3. The Instagram user must accept at: Instagram > Settings > Apps and Websites > Tester Invites

### 4. Set Worker Secrets

```bash
# Generate encryption key: openssl rand -hex 32
wrangler secret put ENCRYPTION_KEY
wrangler secret put INSTAGRAM_APP_ID
wrangler secret put INSTAGRAM_APP_SECRET
```

### How It Works

- The worker runs hourly (via cron trigger) to sync Instagram posts
- Cloudflare AI (`@cf/meta/llama-3.1-8b-instruct`) analyzes captions to extract event details
- Events with confidence >= 0.6 are created on the calendar
- Cancellation posts automatically remove matching events

## Embedding

### iframe

```html
<!-- Form -->
<iframe src="https://your-worker.workers.dev/f/contact"
        style="border:none;width:100%;min-height:500px;"></iframe>

<!-- Booking form -->
<iframe src="https://your-worker.workers.dev/book/{calendar_id}/{slug}"
        style="border:none;width:100%;min-height:600px;"></iframe>

<!-- Calendar view -->
<iframe src="https://your-worker.workers.dev/view/{calendar_id}/{slug}"
        style="border:none;width:100%;min-height:500px;"></iframe>
```

### HTMX

Include HTMX and add your domain to "Allowed Origins" in admin settings.

```html
<script src="https://unpkg.com/htmx.org@1.9.10"></script>

<!-- Form -->
<div hx-get="https://your-worker.workers.dev/f/contact"
     hx-trigger="load"
     hx-swap="innerHTML">
    Loading form...
</div>

<!-- Calendar view -->
<div hx-get="https://your-worker.workers.dev/view/{calendar_id}/{slug}"
     hx-trigger="load"
     hx-swap="innerHTML">
    Loading calendar...
</div>
```

### CSS Customization

Override styles via query parameters:

```html
<!-- Inline CSS -->
<iframe src="https://your-worker.workers.dev/f/contact?css=button{background:green}"></iframe>

<!-- External stylesheet -->
<iframe src="https://your-worker.workers.dev/view/{calendar_id}/{slug}?css_url=https://example.com/custom.css"></iframe>
```

#### Form CSS Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `--cf-font-family` | Font family | `inherit` |
| `--cf-font-size` | Base font size | `1rem` |
| `--cf-text-color` | Main text color | `#333333` |
| `--cf-bg-color` | Page background | `transparent` |
| `--cf-form-bg` | Form container background | `#ffffff` |
| `--cf-border-color` | Input border color | `#dddddd` |
| `--cf-border-radius` | Border radius | `4px` |
| `--cf-primary-color` | Primary button color | `#0070f3` |
| `--cf-primary-hover` | Primary button hover | `#0060df` |

#### Calendar CSS Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `--cal-primary` | Primary/accent color | `#0070f3` |
| `--cal-text` | Text color | `#333333` |
| `--cal-bg` | Background color | `#ffffff` |
| `--cal-border-radius` | Border radius | `4px` |
| `--cal-font` | Font family | `system-ui` |

## License

AGPL3 - see [LICENSE](LICENSE) for the full license text.
