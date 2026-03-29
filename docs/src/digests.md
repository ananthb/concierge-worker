# Digest Notifications

Get periodic summaries of new bookings delivered to your WhatsApp. Never miss an appointment, even when you're busy.

## How It Works

Concierge checks for new bookings on a schedule and sends you a WhatsApp message with a summary. Each digest includes:

- Customer name and email
- Date, time, and duration
- Booking status (confirmed, pending)
- Phone number and notes (if provided)

## Setting Up

In the calendar editor > **Digest** tab:

1. Set **Frequency** to Daily or Weekly
2. Click **+ Add Recipient**
3. Enter a name and WhatsApp phone number
4. Save

### Frequency Options

| Frequency | When It Sends |
|-----------|--------------|
| **Daily** | Every day (on the cron schedule) |
| **Weekly** | Every Monday |
| **Disabled** | No digests sent |

### Multiple Recipients

You can add multiple recipients — each gets the same digest. Useful when multiple people need to know about upcoming appointments.

## Example Digest

```
New bookings for calendar: My Business

You have 3 new booking(s) since the last digest.

--- Booking #1 (2026-03-28T14:30:00Z) ---
Name: Jane Smith
Email: jane@example.com
Phone: +1234567890
Date: 2026-03-30
Time: 2:00 PM
Duration: 30 minutes
Status: Confirmed

--- Booking #2 ...
```

## Requirements

- WhatsApp Business API configured (see [Configuration](./configuration.md))
- Cron trigger configured in `wrangler.toml` (included in default setup)
