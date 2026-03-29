# Setup Guide

This guide walks you through setting up Concierge from scratch.

## Prerequisites

- A [Cloudflare account](https://dash.cloudflare.com/sign-up)
- A [Google Cloud](https://console.cloud.google.com/) project with a service account
- A [Meta Developer](https://developers.facebook.com/) account (for WhatsApp)

## Step 1: Deploy to Cloudflare

See the [Deployment guide](./deployment.md) for detailed instructions on deploying the worker, creating the database, and configuring Cloudflare Access.

## Step 2: Connect Google Calendar

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a service account (or reuse one)
3. Create a JSON key for the service account
4. Set these secrets:

```bash
wrangler secret put GOOGLE_SERVICE_ACCOUNT_EMAIL
# Paste the service account email (e.g., concierge@project.iam.gserviceaccount.com)

wrangler secret put GOOGLE_PRIVATE_KEY
# Paste the private_key field from the JSON key file
```

5. Enable the **Google Calendar API** and **Google Forms API** in your Google Cloud project
6. In Google Calendar, share your calendar with the service account email (give it "Make changes to events" permission)
7. In the Concierge admin, paste the Calendar ID into your calendar's settings

## Step 3: Connect WhatsApp

1. Go to [Meta for Developers](https://developers.facebook.com/)
2. Create an app with the **WhatsApp** product
3. Set up a WhatsApp Business account and get a permanent access token
4. Set these secrets:

```bash
wrangler secret put WHATSAPP_ACCESS_TOKEN
wrangler secret put WHATSAPP_PHONE_NUMBER_ID
```

## Step 4: Connect Instagram (Optional)

For automatic event import from Instagram posts:

1. Create a Meta app with **Instagram Basic Display**
2. Configure the OAuth redirect URI: `https://your-worker.workers.dev/instagram/callback`
3. Set these secrets:

```bash
wrangler secret put ENCRYPTION_KEY
# Generate with: openssl rand -hex 32

wrangler secret put INSTAGRAM_APP_ID
wrangler secret put INSTAGRAM_APP_SECRET
```

4. In the Concierge admin, go to your calendar's Google Calendar tab and click "Connect Instagram Account"

## Step 5: Configure Your Calendar

1. Go to your admin dashboard (`https://your-worker.workers.dev/admin`)
2. Click **+ New Calendar**
3. Set the name, timezone, and Google Calendar ID
4. Add **time slots** for booking availability
5. Create a **booking link** for customers
6. Create a **view link** to embed your calendar
7. Add **form links** for Google Forms
8. Configure **digest** notifications for WhatsApp summaries

## Step 6: Embed on Your Website

Add a single line of HTML to your website:

```html
<!-- Calendar -->
<iframe src="https://your-worker.workers.dev/view/{calendar_id}/{slug}"
        style="border: none; width: 100%; min-height: 500px;"></iframe>

<!-- Booking form -->
<iframe src="https://your-worker.workers.dev/book/{calendar_id}/{slug}"
        style="border: none; width: 100%; min-height: 600px;"></iframe>

<!-- Contact form -->
<iframe src="https://your-worker.workers.dev/form/{calendar_id}/{slug}"
        style="border: none; width: 100%; min-height: 800px;"></iframe>
```

See [Embedding](./embedding.md) for more options.

## You're Done

From here, Concierge runs itself:

- Post on Instagram and events appear on your calendar
- Customers book appointments and get WhatsApp confirmations
- You get daily booking digests on WhatsApp
- Form responses are viewable in the admin dashboard
