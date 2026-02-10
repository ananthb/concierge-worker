# Query Parameters Reference

This page is auto-generated from the source code.

## Form Embedding

| Parameter | Description | Example |
|-----------|-------------|---------|
| `css` | Inline CSS to apply | `?css=button{background:green}` |
| `css_url` | URL to external stylesheet | `?css_url=https://example.com/style.css` |

## Calendar & Booking Views

| Parameter | Description | Example |
|-----------|-------------|---------|
| `css` | Inline CSS to apply | `?css=.event{color:blue}` |
| `css_url` | URL to external stylesheet | `?css_url=https://example.com/cal.css` |
| `date` | Starting date for view | `?date=2024-03-15` |
| `view` | View type (week/month/year/endless) | `?view=month` |
| `days` | Number of days to show (booking) | `?days=14` |
| `hide_title` | Hide the title | `?hide_title=true` |

## iCal Feeds

| Parameter | Description | Example |
|-----------|-------------|---------|
| `token` | Authentication token | `?token=abc123` |

## Combining Parameters

Multiple parameters can be combined:

```
/f/contact?css=button{background:green}&hide_title=true
/view/{id}/{slug}?date=2024-03-15&view=month&css_url=https://example.com/cal.css
```
