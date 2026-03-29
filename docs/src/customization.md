# Customizing Appearance

Customize the look of calendars, booking pages, and form wrappers.

## Customization Methods

### 1. Admin Settings

Edit custom CSS in the calendar editor > **Settings** tab > **Custom CSS** field.

### 2. Query Parameters

Override styles via URL:

```html
<!-- Inline CSS -->
<iframe src=".../view/{id}/{slug}?css=.event{background:green}"></iframe>

<!-- External stylesheet -->
<iframe src=".../view/{id}/{slug}?css_url=https://example.com/style.css"></iframe>
```

### 3. Page CSS (HTMX only)

When using HTMX embedding, your page CSS applies directly to the embedded content.

## CSS Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `--cal-primary` | Accent/button color | `#0070f3` |
| `--cal-text` | Text color | `#333333` |
| `--cal-bg` | Background color | `#ffffff` |
| `--cal-border-radius` | Border radius | `4px` |
| `--cal-font` | Font family | `system-ui` |

## CSS Classes

### Calendar Classes

| Class | Element |
|-------|---------|
| `#calendar-view` | Main calendar container |
| `.nav` | Navigation header |
| `.event` | Event item |
| `.booking` | Booking item |
| `.week-day` | Day in week view |
| `.calendar` | Month grid table |
| `.mini-month` | Month in year view |
| `.list-date` | Date group in list view |
| `.list-item` | Item in list view |

### Booking Classes

| Class | Element |
|-------|---------|
| `.form-group` | Field wrapper |
| `.time-slots` | Time slot container |
| `.btn` | Buttons |
| `.success` | Success message |
| `.error` | Error message |

## Examples

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
    color: inherit;
}
```

### Brand Colors

```css
:root {
    --cal-primary: #6366f1;
    --cal-text: #1e1e2e;
    --cal-bg: #fafafa;
    --cal-border-radius: 8px;
}
```

### Transparent Background (for embedding)

```css
:root {
    --cal-bg: transparent;
}
```

## Using External Stylesheets

Host a CSS file and reference it:

```html
<iframe src=".../view/{id}/{slug}?css_url=https://example.com/brand.css"></iframe>
```

## Responsive Design

Calendars and booking forms are responsive by default. Override breakpoints:

```css
@media (max-width: 600px) {
    .time-slots {
        grid-template-columns: repeat(2, 1fr);
    }
}
```

## Hiding the Title

Via query param:
```
?notitle=true
```

Via CSS:
```css
h1 { display: none; }
```
