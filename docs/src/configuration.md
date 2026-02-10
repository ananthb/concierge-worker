# Configuration

All sensitive configuration is stored as Cloudflare Workers secrets.

## Setting Secrets

Use the Wrangler CLI to set secrets:

```bash
wrangler secret put SECRET_NAME
```

You'll be prompted to enter the value securely.

## Notification Channels

> **Important:** The Responders and Digest tabs in the admin UI are only visible when at least one notification channel is configured.

### Twilio (SMS & WhatsApp)

Required for SMS and WhatsApp notifications:

```bash
wrangler secret put TWILIO_ACCOUNT_SID
wrangler secret put TWILIO_AUTH_TOKEN
wrangler secret put TWILIO_FROM_SMS        # e.g., +15551234567
wrangler secret put TWILIO_FROM_WHATSAPP   # e.g., whatsapp:+15551234567
```

### Twilio Email (via SendGrid)

Required for email via Twilio/SendGrid:

```bash
wrangler secret put TWILIO_ACCOUNT_SID     # same as above
wrangler secret put TWILIO_AUTH_TOKEN      # same as above
wrangler secret put SENDGRID_API_KEY
wrangler secret put TWILIO_FROM_EMAIL      # e.g., noreply@yourdomain.com
```

### Resend Email

Alternative email provider:

```bash
wrangler secret put RESEND_API_KEY
wrangler secret put RESEND_FROM            # e.g., noreply@yourdomain.com
```

## Channel Detection

The admin UI automatically detects which channels are available:

| Channel | Required Secrets |
|---------|------------------|
| Twilio SMS | `TWILIO_ACCOUNT_SID` + `TWILIO_AUTH_TOKEN` |
| Twilio WhatsApp | `TWILIO_ACCOUNT_SID` + `TWILIO_AUTH_TOKEN` |
| Twilio Email | `TWILIO_ACCOUNT_SID` + `TWILIO_AUTH_TOKEN` + `SENDGRID_API_KEY` |
| Resend Email | `RESEND_API_KEY` |

## Instagram Integration

For automatic event extraction from Instagram posts:

```bash
# Generate with: openssl rand -hex 32
wrangler secret put ENCRYPTION_KEY

wrangler secret put INSTAGRAM_APP_ID
wrangler secret put INSTAGRAM_APP_SECRET
```

### Setting up Meta App

1. Go to [Meta for Developers](https://developers.facebook.com/)
2. Create an app with **Instagram Basic Display** product
3. Configure OAuth redirect URI: `https://your-worker.workers.dev/instagram/callback`
4. Add test users while in development mode

## Google Sheets Integration

For syncing form submissions to Google Sheets:

```bash
wrangler secret put GOOGLE_SERVICE_ACCOUNT_EMAIL
wrangler secret put GOOGLE_SERVICE_ACCOUNT_KEY
```

The service account key should be the private key from your Google Cloud service account JSON file.

## All Secrets Reference

| Secret | Purpose | Required |
|--------|---------|----------|
| `TWILIO_ACCOUNT_SID` | Twilio account SID | For Twilio channels |
| `TWILIO_AUTH_TOKEN` | Twilio auth token | For Twilio channels |
| `TWILIO_FROM_SMS` | SMS sender number | For SMS |
| `TWILIO_FROM_WHATSAPP` | WhatsApp sender | For WhatsApp |
| `TWILIO_FROM_EMAIL` | Email sender address | For Twilio email |
| `SENDGRID_API_KEY` | SendGrid API key | For Twilio email |
| `RESEND_API_KEY` | Resend API key | For Resend email |
| `RESEND_FROM` | Resend sender address | For Resend email |
| `ENCRYPTION_KEY` | Token encryption | For Instagram |
| `INSTAGRAM_APP_ID` | Meta app ID | For Instagram |
| `INSTAGRAM_APP_SECRET` | Meta app secret | For Instagram |
| `GOOGLE_SERVICE_ACCOUNT_EMAIL` | GCP service account | For Sheets |
| `GOOGLE_SERVICE_ACCOUNT_KEY` | GCP private key | For Sheets |
