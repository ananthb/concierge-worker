# iframe Embedding

Embed booking pages and calendar views using iframes.

## Basic Usage

### Google Forms

```html
<iframe
    src="https://your-worker.workers.dev/form/{calendar_id}/{slug}"
    style="border: none; width: 100%; min-height: 800px;">
</iframe>
```

### Booking Forms

```html
<iframe
    src="https://your-worker.workers.dev/book/{calendar_id}/{slug}"
    style="border: none; width: 100%; min-height: 600px;">
</iframe>
```

### Calendar Views

```html
<iframe
    src="https://your-worker.workers.dev/view/{calendar_id}/{slug}"
    style="border: none; width: 100%; min-height: 500px;">
</iframe>
```

## Query Parameters

Customize appearance via URL parameters:

### Hide Title

```html
<iframe src="https://your-worker.workers.dev/book/{id}/{slug}?notitle=true"></iframe>
```

### Calendar View Options

```html
<!-- Start on specific date -->
<iframe src=".../view/{id}/{slug}?date=2024-03-15"></iframe>

<!-- Force specific view -->
<iframe src=".../view/{id}/{slug}?view=month"></iframe>

<!-- Booking: show more days -->
<iframe src=".../book/{id}/{slug}?days=14"></iframe>
```

See [Query Parameters](./generated/query-params.md) for full reference.

## Cross-Origin Considerations

### Allowed Origins

For security, configure allowed origins in the calendar settings:

1. Open calendar editor
2. Add your domain to **Allowed Origins**
3. Leave empty to allow all (not recommended for production)

### Transparent Background

```html
<iframe src="...?css=body{background:transparent}"></iframe>
```
