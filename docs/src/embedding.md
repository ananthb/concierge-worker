# Embedding on Your Website

Concierge is designed to be embedded on your existing website. Drop a snippet of HTML and your calendar, booking form, or contact form appears on your page.

## Two Ways to Embed

### iframe (Simple)

Works everywhere. One line of HTML. The content loads in an isolated frame.

```html
<iframe src="https://your-worker.workers.dev/view/{calendar_id}/{slug}"
        style="border: none; width: 100%; min-height: 500px;">
</iframe>
```

Best for: WordPress, Squarespace, Wix, or any site where you can add HTML.

[iframe details](./embedding-iframe.md)

### HTMX (Seamless)

Content loads directly into your page. Your CSS applies. No iframe sizing issues.

```html
<script src="https://unpkg.com/htmx.org@1.9.10"></script>
<div hx-get="https://your-worker.workers.dev/view/{calendar_id}/{slug}"
     hx-trigger="load" hx-swap="innerHTML">
    Loading calendar...
</div>
```

Best for: Custom-built sites where you want full control over styling.

[HTMX details](./embedding-htmx.md)

## What You Can Embed

| Type | URL Pattern | Use For |
|------|------------|---------|
| Calendar view | `/view/{calendar_id}/{slug}` | Displaying events (week, month, year, list) |
| Booking form | `/book/{calendar_id}/{slug}` | Letting customers book appointments |
| Google Form | `/form/{calendar_id}/{slug}` | Contact forms, questionnaires, feedback |

## Quick Customization

Add query parameters to any embed URL:

```
?notitle=true          Hide the title
?view=month            Force a specific calendar view
?days=14               Show 14 days of booking slots
?css=body{background:transparent}   Inject custom CSS
```

See [Customizing Appearance](./customization.md) for full details.

## Allowed Origins

For security, add your website's domain to the calendar's **Allowed Origins** setting in the admin. This enables cross-origin requests for HTMX embedding. Leave empty to allow all origins.
