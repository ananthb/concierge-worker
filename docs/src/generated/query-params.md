# Query Parameters Reference

## Calendar & Booking Views

| Parameter | Description | Example |
|-----------|-------------|---------|
| `css` | Inline CSS to apply | `?css=.event{color:blue}` |
| `css_url` | URL to external stylesheet | `?css_url=https://example.com/cal.css` |
| `date` | Starting date for view | `?date=2024-03-15` |
| `view` | View type (week/month/year/endless) | `?view=month` |
| `days` | Number of days to show (booking) | `?days=14` |
| `notitle` | Hide the title | `?notitle=true` |

## Combining Parameters

Multiple parameters can be combined:

```
/view/{id}/{slug}?date=2024-03-15&view=month&css_url=https://example.com/cal.css
/book/{id}/{slug}?days=14&notitle=true
```
