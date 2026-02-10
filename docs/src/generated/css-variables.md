# CSS Variables Reference

This page is auto-generated from the source code.

## Form CSS Variables

Use these variables to customize form appearance:

| Variable | Description |
|----------|-------------|
| `--cf-bg-color` | Page background color |
| `--cf-border-color` | Input border color |
| `--cf-border-radius` | Border radius |
| `--cf-font-family` | Font family |
| `--cf-font-size` | Base font size |
| `--cf-form-bg` | Form container background |
| `--cf-input-padding` | Input padding |
| `--cf-primary-color` | Primary/button color |
| `--cf-primary-hover` | Primary hover color |
| `--cf-text-color` | Main text color |

## Calendar CSS Variables

Use these variables to customize calendar and booking views:

| Variable | Description |
|----------|-------------|
| `--cal-bg` | Background color |
| `--cal-border-radius` | Border radius |
| `--cal-font` | Font family |
| `--cal-primary` | Primary/accent color |
| `--cal-text` | Text color |

## Usage Examples

### Override via Query Parameter

```html
<iframe src=".../f/contact?css=--cf-primary-color:green"></iframe>
```

### Override in Page CSS (HTMX)

```css
:root {
    --cf-primary-color: #6366f1;
    --cal-primary: #6366f1;
}
```

### Override in Admin Settings

1. Open form/calendar editor
2. Go to Styling tab
3. Add Custom CSS:

```css
:root {
    --cf-primary-color: #your-color;
}
```
