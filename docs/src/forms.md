# Forms

Create dynamic forms with custom fields, styling, and automated responses.

## Creating a Form

1. Go to the admin dashboard (`/admin`)
2. Click **+ Create Form**
3. Configure the form settings

## Form Settings

### Basic Settings

| Field | Description |
|-------|-------------|
| Form Name | Internal name (shown in admin) |
| Slug | URL path (e.g., `contact` → `/f/contact`) |
| Form Title | Displayed to users |
| Submit Button Text | Button label |
| Success Message | Shown after submission |
| Allowed Origins | Domains that can embed this form |
| Google Sheet URL | Optional: sync submissions to a spreadsheet |

### Field Types

| Type | HTML Input | Validation |
|------|------------|------------|
| Text | `<input type="text">` | None |
| Email | `<input type="email">` | Email format |
| Mobile | `<input type="tel">` | Phone format |
| Long Text | `<textarea>` | None |
| File | `<input type="file">` | Size limit |

### Field Configuration

Each field has:

- **Label** - Displayed above the input
- **Field ID** - Used in templates and data (e.g., `name`, `email`)
- **Type** - Input type (see above)
- **Required** - Whether the field must be filled

## Styling

Forms can be styled using CSS variables. See [CSS Customization](./customization.md) for details.

### Show/Hide Title

Toggle "Show form title" in the Styling tab to hide the title when embedding.

### Custom CSS

Add custom CSS rules in the Styling tab:

```css
.contact-form {
    max-width: 500px;
}
button {
    text-transform: uppercase;
}
```

## Viewing Responses

1. Go to admin dashboard
2. Click **Responses** next to the form
3. View all submissions in a table

## Archiving Forms

Archive forms to make them read-only without deleting data:

1. Click **Archive** in the dashboard
2. Form becomes read-only
3. Click **Unarchive** to restore

## Deleting Forms

> **Warning:** This permanently deletes the form and all submissions.

1. Open the form editor
2. Scroll to Danger Zone
3. Click **Delete Form**
