# Event Calendars

Display your events on an embeddable calendar powered by Google Calendar. Events can come from Google Calendar directly, be created automatically from Instagram posts, or be generated when customers book appointments.

## Setting Up

### 1. Create a Calendar

Go to the admin dashboard > **+ New Calendar**. Set a name and timezone.

### 2. Connect Google Calendar

In the calendar editor > **Settings** tab:

1. Paste your Google Calendar ID (found in Google Calendar Settings > Integrate calendar)
2. Make sure your Google Calendar is shared with the service account email

Events from your Google Calendar will now appear in your embedded views.

### 3. Create a View Link

In **Settings** tab > **+ Add View Link**. This creates a public URL for your calendar.

### View Types

| Type | Best For |
|------|----------|
| **Month** | Overview of a full month |
| **Week** | Detailed 7-day view |
| **Year** | Annual overview with event indicators |
| **List** | Scrolling list of upcoming events |

### Visibility Controls

Each view link can show or hide:

- Event titles and details
- Booking names and details

Useful for public calendars where you want to show that slots are taken without revealing customer names.

## Embedding

```html
<!-- iframe -->
<iframe src="https://your-worker.workers.dev/view/{calendar_id}/{slug}"
        style="border: none; width: 100%; min-height: 500px;"></iframe>

<!-- HTMX -->
<div hx-get="https://your-worker.workers.dev/view/{calendar_id}/{slug}"
     hx-trigger="load" hx-swap="innerHTML">Loading...</div>
```

### Navigation

Embedded calendars include prev/next navigation and a view type selector. When using HTMX, navigation is seamless — only the calendar content updates, not the whole page.

## Where Events Come From

Events appear on your calendar from three sources:

1. **Google Calendar** — Any event you add to Google Calendar shows up automatically
2. **Instagram** — AI extracts event details from your posts (see [Instagram Integration](./instagram.md))
3. **Bookings** — Confirmed appointments create Google Calendar events (see [Bookings](./bookings.md))

## Calendar Settings

| Setting | Description |
|---------|-------------|
| Name | Calendar name shown in views |
| Description | Optional description |
| Timezone | Used for event display and Google Calendar API |
| Google Calendar ID | Links to your Google Calendar |
| Allowed Origins | Domains that can embed your calendar |
| Custom CSS | Additional styling for embedded views |
