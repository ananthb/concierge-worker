# iframe Embedding

Embed forms, booking pages, and calendar views using iframes.

## Basic Usage

### Forms

```html
<iframe
    src="https://your-worker.workers.dev/f/contact"
    style="border: none; width: 100%; min-height: 500px;">
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

## Auto-Resizing

iframes don't automatically resize to fit content. Options:

### 1. Fixed Height

Set a minimum height that accommodates your content:

```html
<iframe src="..." style="min-height: 600px;"></iframe>
```

### 2. JavaScript Resize

Use a library like [iframe-resizer](https://github.com/davidjbradshaw/iframe-resizer):

```html
<script src="https://cdn.jsdelivr.net/npm/iframe-resizer@4/js/iframeResizer.min.js"></script>
<iframe id="form" src="..."></iframe>
<script>iFrameResize({}, '#form')</script>
```

## Query Parameters

Customize appearance via URL parameters:

### Inline CSS

```html
<iframe src="https://your-worker.workers.dev/f/contact?css=button{background:green}"></iframe>
```

### External Stylesheet

```html
<iframe src="https://your-worker.workers.dev/f/contact?css_url=https://example.com/form.css"></iframe>
```

### Hide Title

```html
<iframe src="https://your-worker.workers.dev/book/{id}/{slug}?hide_title=true"></iframe>
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

For security, configure allowed origins in the form/calendar settings:

1. Open form or calendar editor
2. Add your domain to **Allowed Origins**
3. Leave empty to allow all (not recommended for production)

Example:
```
https://example.com
https://www.example.com
```

### Cookie Issues

Third-party cookies may be blocked by browsers. This can affect:

- Session persistence
- CSRF protection

Solutions:

1. Use HTMX embedding instead (same-origin)
2. Host the worker on a subdomain of your site
3. Use the form as a standalone page with redirect

## Styling Tips

### Remove iframe Border

```html
<iframe src="..." style="border: none;"></iframe>
```

### Responsive Width

```html
<iframe src="..." style="width: 100%; max-width: 600px;"></iframe>
```

### Transparent Background

The form/calendar will use its configured background color. To make it transparent:

```html
<iframe src="...?css=body{background:transparent}"></iframe>
```

Then ensure your page has a background color.

## Example: Complete Form Embed

```html
<!DOCTYPE html>
<html>
<head>
    <style>
        .form-container {
            max-width: 600px;
            margin: 2rem auto;
        }
        .form-container iframe {
            border: none;
            width: 100%;
            min-height: 500px;
        }
    </style>
</head>
<body>
    <div class="form-container">
        <iframe src="https://your-worker.workers.dev/f/contact?css=body{background:transparent}"></iframe>
    </div>
</body>
</html>
```
