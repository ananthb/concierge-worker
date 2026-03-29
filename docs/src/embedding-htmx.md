# HTMX Embedding

Embed booking forms and calendar views directly into your page using HTMX for a seamless experience.

## Why HTMX?

Compared to iframes:

- **No iframe sizing issues** - Content is part of your page
- **Better styling control** - Your CSS applies directly
- **Smoother interactions** - No iframe refresh on submit
- **SEO friendly** - Content is in the DOM

## Setup

### 1. Include HTMX

Add HTMX to your page:

```html
<script src="https://unpkg.com/htmx.org@1.9.10"></script>
```

### 2. Configure Allowed Origins

Add your domain to the calendar's **Allowed Origins** setting:

```
https://example.com
https://www.example.com
```

## Google Forms

```html
<div hx-get="https://your-worker.workers.dev/form/{calendar_id}/{slug}"
     hx-trigger="load"
     hx-swap="innerHTML">
    Loading form...
</div>
```

## Calendar Views

```html
<div hx-get="https://your-worker.workers.dev/view/{calendar_id}/{slug}"
     hx-trigger="load"
     hx-swap="innerHTML">
    Loading calendar...
</div>
```

### With Date Parameter

```html
<div hx-get="https://your-worker.workers.dev/view/{calendar_id}/{slug}?date=2024-03-15"
     hx-trigger="load"
     hx-swap="innerHTML">
</div>
```

## Booking Forms

```html
<div hx-get="https://your-worker.workers.dev/book/{calendar_id}/{slug}"
     hx-trigger="load"
     hx-swap="innerHTML">
    Loading booking form...
</div>
```

## Styling

Since HTMX content is part of your page, you have full CSS control.

### Override CSS Variables

```html
<style>
    :root {
        --cal-primary: #28a745;
        --cal-bg: transparent;
    }
</style>
```

## Query Parameters

Pass parameters to customize the embedded content:

```html
<!-- Custom view -->
<div hx-get=".../view/{id}/{slug}?view=month" ...></div>

<!-- Hide title -->
<div hx-get=".../book/{id}/{slug}?notitle=true" ...></div>
```

## Error Handling

Handle network errors gracefully:

```html
<div hx-get="https://your-worker.workers.dev/book/{id}/{slug}"
     hx-trigger="load"
     hx-swap="innerHTML"
     hx-on::response-error="this.innerHTML = 'Failed to load'">
    Loading...
</div>
```
