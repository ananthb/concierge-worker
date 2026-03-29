# Instagram Integration

Concierge monitors your Instagram posts and uses AI to extract event details from your captions. When it finds an event, it automatically creates a Google Calendar event — so you post once and your calendar updates everywhere.

## How It Works

1. You connect your Instagram account in the admin
2. Concierge checks for new posts every hour (via a cron trigger)
3. For each new post, AI analyzes the caption
4. If the caption describes an event (date, time, title), a Google Calendar event is created
5. The event appears on your embedded calendar automatically

## What the AI Extracts

The AI looks for:

- **Title** — The name of the event
- **Date** — When the event happens
- **Start time** and **end time**
- **Description** — Details about the event
- **Cancellations** — If a post is cancelling a previously announced event

The AI uses confidence scoring — it only creates events when it's confident the post actually describes an event.

## Setting Up

### 1. Create a Meta App

1. Go to [Meta for Developers](https://developers.facebook.com/)
2. Create an app with **Instagram Basic Display**
3. Set the OAuth redirect URI to: `https://your-worker.workers.dev/instagram/callback`
4. Add your Instagram account as a test user (required in development mode)

### 2. Configure Secrets

```bash
wrangler secret put ENCRYPTION_KEY
# Generate with: openssl rand -hex 32

wrangler secret put INSTAGRAM_APP_ID
wrangler secret put INSTAGRAM_APP_SECRET
```

### 3. Connect Your Account

In the Concierge admin:

1. Go to your calendar editor > **Google Calendar** tab
2. Click **Connect Instagram Account**
3. Authorize access when prompted

### 4. Ensure Google Calendar Is Connected

Instagram events are created in your Google Calendar, so make sure you've configured the Google Calendar ID in your calendar settings.

## Deduplication

Concierge tracks which posts it has already processed:

- Same post won't be processed twice
- Events with the same title and date won't be duplicated
- If a post is edited, it will be re-processed

## Cancellations

If you post about cancelling an event, the AI detects this and logs it. Currently, cancelled events should be removed from Google Calendar directly.

## Syncing Schedule

Instagram posts are synced hourly via a Cloudflare cron trigger. Recent posts are checked each time.
