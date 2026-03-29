# Google Forms

Embed Google Forms on your website through Concierge. Use them for contact forms, intake questionnaires, feedback, or anything else — with the ability to view responses directly in the admin dashboard.

## Setting Up

### 1. Create a Google Form

Create your form at [Google Forms](https://docs.google.com/forms/). Design it however you like — all of Google Forms' field types, validation, and theming work normally.

### 2. Share With Your Service Account

To view responses in the Concierge admin, share the form with your service account email address (the same one used for Google Calendar).

### 3. Add a Form Link

In the Concierge admin:

1. Go to your calendar editor > **Settings** tab
2. Click **+ Add Form Link**
3. Click **Edit** on the new form link
4. Paste the Google Form URL
5. Give it a name (e.g., "Contact Form")
6. Save

### 4. Embed on Your Website

```html
<iframe src="https://your-worker.workers.dev/form/{calendar_id}/{slug}"
        style="border: none; width: 100%; min-height: 800px;"></iframe>
```

Or with HTMX:

```html
<div hx-get="https://your-worker.workers.dev/form/{calendar_id}/{slug}"
     hx-trigger="load" hx-swap="innerHTML">
    Loading form...
</div>
```

## Viewing Responses

Go to your form link editor and click **View Responses**. Concierge fetches responses directly from the Google Forms API and displays them in a table with all fields and timestamps.

This requires the form to be shared with your service account email.

## Customizing Appearance

Google Forms has its own theming (header image, colors, font). These carry into the embedded version. Concierge wraps the form in its own page with CSS variable support, so you can match the surrounding page.

Query parameters:

| Parameter | Effect |
|-----------|--------|
| `notitle=true` | Hide the Concierge title above the form |
| `css=...` | Inject inline CSS |
| `css_url=...` | Load an external stylesheet |

Note: The Google Form itself is in an iframe within the page, so your CSS cannot reach inside the form. Style the Google Form using Google Forms' built-in theme editor.

## Multiple Forms

You can add multiple form links to a single calendar — each gets its own slug and embed URL. Use this for different purposes (contact, intake, feedback, etc.).
