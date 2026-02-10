# HTMX Embedding

Embed forms and calendars directly into your page using HTMX for a seamless experience.

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

Add your domain to the form/calendar's **Allowed Origins** setting:

```
https://example.com
https://www.example.com
```

## Forms

### Basic Embed

```html
<div hx-get="https://your-worker.workers.dev/f/contact"
     hx-trigger="load"
     hx-swap="innerHTML">
    Loading form...
</div>
```

### With Loading Indicator

```html
<div hx-get="https://your-worker.workers.dev/f/contact"
     hx-trigger="load"
     hx-swap="innerHTML"
     hx-indicator="#form-loading">
    <div id="form-loading" class="htmx-indicator">
        Loading...
    </div>
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
    /* Form variables */
    :root {
        --cf-primary-color: #007bff;
        --cf-border-radius: 8px;
    }

    /* Calendar variables */
    :root {
        --cal-primary: #28a745;
        --cal-bg: transparent;
    }
</style>
```

### Target Form Elements

```html
<style>
    .contact-form {
        max-width: 500px;
        margin: 0 auto;
    }

    .contact-form button {
        text-transform: uppercase;
    }

    .form-group label {
        font-weight: bold;
    }
</style>
```

### Hide Title via CSS

```html
<style>
    .contact-form h1 {
        display: none;
    }
</style>
```

## Query Parameters

Pass parameters to customize the embedded content:

```html
<!-- Hide title -->
<div hx-get=".../f/contact?hide_title=true" ...></div>

<!-- Custom view -->
<div hx-get=".../view/{id}/{slug}?view=month" ...></div>

<!-- Inline CSS override -->
<div hx-get=".../f/contact?css=button{background:green}" ...></div>
```

## Form Submission

Forms use HTMX's `hx-post` for submission:

1. User fills form
2. Submit button triggers HTMX POST
3. Success message replaces form
4. After 3 seconds, form reloads automatically

### Custom Success Handling

Override the default behavior:

```html
<div hx-get="https://your-worker.workers.dev/f/contact"
     hx-trigger="load"
     hx-swap="innerHTML"
     hx-on::after-settle="handleFormLoad(event)">
</div>

<script>
function handleFormLoad(event) {
    // Add custom event listeners to the loaded form
    const form = event.target.querySelector('form');
    if (form) {
        form.addEventListener('htmx:afterRequest', function(e) {
            if (e.detail.successful) {
                // Custom success handling
                console.log('Form submitted successfully');
            }
        });
    }
}
</script>
```

## Error Handling

Handle network errors gracefully:

```html
<div hx-get="https://your-worker.workers.dev/f/contact"
     hx-trigger="load"
     hx-swap="innerHTML"
     hx-on::response-error="this.innerHTML = 'Failed to load form'">
    Loading...
</div>
```

## Complete Example

```html
<!DOCTYPE html>
<html>
<head>
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <style>
        .form-container {
            max-width: 600px;
            margin: 2rem auto;
            padding: 1rem;
        }

        /* Override form styles */
        :root {
            --cf-primary-color: #6366f1;
            --cf-border-radius: 12px;
            --cf-bg-color: transparent;
        }

        .contact-form {
            background: white;
            padding: 2rem;
            border-radius: 12px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        }

        .htmx-indicator {
            display: none;
        }
        .htmx-request .htmx-indicator {
            display: block;
        }
    </style>
</head>
<body>
    <div class="form-container">
        <div hx-get="https://your-worker.workers.dev/f/contact"
             hx-trigger="load"
             hx-swap="innerHTML"
             hx-indicator=".loading">
            <div class="loading htmx-indicator">Loading form...</div>
        </div>
    </div>
</body>
</html>
```
