# Configuration

All sensitive configuration is stored as Cloudflare Workers secrets.

## Setting Secrets

Use the Wrangler CLI to set secrets:

```bash
wrangler secret put SECRET_NAME
```

You'll be prompted to enter the value securely.

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `ENVIRONMENT` | Set to `development` to bypass auth | No |

## All Secrets Reference

| Secret | Purpose |
|--------|---------|
| `GOOGLE_SERVICE_ACCOUNT_EMAIL` | Google service account email (for Calendar) |
| `GOOGLE_PRIVATE_KEY` | Google service account RSA private key (for Calendar) |
| `WHATSAPP_ACCESS_TOKEN` | WhatsApp Business API access token |
| `WHATSAPP_PHONE_NUMBER_ID` | WhatsApp Business API phone number ID |
| `ENCRYPTION_KEY` | Token encryption key (for Instagram) |
| `INSTAGRAM_APP_ID` | Meta/Instagram app ID |
| `INSTAGRAM_APP_SECRET` | Meta/Instagram app secret |

## Google Calendar Integration

For displaying and creating events in Google Calendar:

```bash
wrangler secret put GOOGLE_SERVICE_ACCOUNT_EMAIL
wrangler secret put GOOGLE_PRIVATE_KEY
```

### Setup

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a service account (or reuse an existing one)
3. Create a key for the service account (JSON format)
4. Set `GOOGLE_SERVICE_ACCOUNT_EMAIL` to the service account's email address
5. Set `GOOGLE_PRIVATE_KEY` to the `private_key` field from the JSON key file
6. Enable the **Google Calendar API** in your project
7. Share each Google Calendar with the service account email (give it "Make changes to events" permission)
8. Copy the Calendar ID from Google Calendar Settings > Integrate calendar
9. Paste it into the calendar's **Google Calendar ID** field in the admin UI

## WhatsApp Business API (Meta)

For booking confirmations and admin notifications:

```bash
wrangler secret put WHATSAPP_ACCESS_TOKEN
wrangler secret put WHATSAPP_PHONE_NUMBER_ID
```

### Setup

1. Go to [Meta for Developers](https://developers.facebook.com/)
2. Create an app with **WhatsApp** product
3. Set up a WhatsApp Business account
4. Get a permanent access token and phone number ID

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
