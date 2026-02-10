# CSS Customization

Customize the appearance of forms, calendars, and booking pages.

## Customization Methods

### 1. Admin Settings

Edit CSS variables and custom CSS in the admin UI:

1. Open form or calendar editor
2. Go to **Styling** tab
3. Modify CSS variables or add custom CSS

### 2. Query Parameters

Override styles via URL:

```html
<!-- Inline CSS -->
<iframe src=".../f/contact?css=button{background:green}"></iframe>

<!-- External stylesheet -->
<iframe src=".../f/contact?css_url=https://example.com/style.css"></iframe>
```

### 3. Page CSS (HTMX only)

When using HTMX embedding, your page CSS applies directly.

## CSS Variables

### Form Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `--cf-font-family` | Font family | `inherit` |
| `--cf-font-size` | Base font size | `1rem` |
| `--cf-text-color` | Text color | `#333333` |
| `--cf-bg-color` | Page background | `transparent` |
| `--cf-form-bg` | Form background | `#ffffff` |
| `--cf-border-color` | Border color | `#dddddd` |
| `--cf-border-radius` | Border radius | `4px` |
| `--cf-primary-color` | Button color | `#0070f3` |
| `--cf-primary-hover` | Button hover | `#0060df` |
| `--cf-input-padding` | Input padding | `0.5rem` |

### Calendar Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `--cal-primary` | Accent color | `#0070f3` |
| `--cal-text` | Text color | `#333333` |
| `--cal-bg` | Background | `#ffffff` |
| `--cal-border-radius` | Border radius | `4px` |
| `--cal-font` | Font family | `system-ui` |

See [CSS Variables Reference](./generated/css-variables.md) for the complete auto-generated list.

## CSS Classes

### Form Classes

| Class | Element |
|-------|---------|
| `.contact-form` | Form container |
| `.form-group` | Field wrapper |
| `label` | Field labels |
| `input`, `textarea` | Input fields |
| `button` | Submit button |
| `.success` | Success message |
| `.error` | Error message |

### Calendar Classes

| Class | Element |
|-------|---------|
| `.calendar-view` | Main container |
| `.calendar-header` | Navigation header |
| `.calendar-grid` | Day grid |
| `.event` | Event item |
| `.booking-form` | Booking form |
| `.time-slots` | Time slot container |
| `.time-slot` | Individual slot button |

## Examples

### Dark Theme Form

```css
:root {
    --cf-bg-color: #1a1a1a;
    --cf-form-bg: #2d2d2d;
    --cf-text-color: #e0e0e0;
    --cf-border-color: #444444;
    --cf-primary-color: #3b9eff;
}
```

### Rounded Modern Style

```css
:root {
    --cf-border-radius: 12px;
    --cf-input-padding: 0.75rem;
}

.contact-form {
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

button {
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}
```

### Minimal Calendar

```css
:root {
    --cal-primary: #000000;
    --cal-bg: transparent;
    --cal-border-radius: 0;
}

.event {
    border-left: 3px solid var(--cal-primary);
    background: transparent;
}
```

### Brand Colors

```css
/* Use your brand colors */
:root {
    --cf-primary-color: #6366f1;  /* Indigo */
    --cf-primary-hover: #4f46e5;
    --cal-primary: #6366f1;
}
```

## Using External Stylesheets

Host a CSS file and reference it:

```html
<iframe src=".../f/contact?css_url=https://example.com/brand.css"></iframe>
```

Your external CSS can override any styles:

```css
/* brand.css */
:root {
    --cf-primary-color: #your-color;
}

.contact-form {
    font-family: 'Your Font', sans-serif;
}
```

## Responsive Design

Forms and calendars are responsive by default. Override breakpoints:

```css
@media (max-width: 600px) {
    .contact-form {
        padding: 1rem;
    }

    .time-slots {
        grid-template-columns: repeat(2, 1fr);
    }
}
```

## Hiding Elements

### Hide Title

Via query param:
```
?hide_title=true
```

Via CSS:
```css
.contact-form h1,
.booking-form h1 {
    display: none;
}
```

### Hide Specific Fields

```css
.form-group:has(input[name="phone"]) {
    display: none;
}
```

## Print Styles

Add print-specific styles:

```css
@media print {
    .calendar-header button {
        display: none;
    }

    .event {
        break-inside: avoid;
    }
}
```
