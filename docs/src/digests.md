# Digests

Receive periodic summaries of new form submissions and bookings.

## Prerequisites

Configure at least one notification channel. See [Configuration](./configuration.md).

> **Note:** The Digest tab only appears when channels are configured.

## Form Digests

Get summaries of new form submissions.

### Setup

1. Open form editor
2. Go to **Digest** tab
3. Select frequency
4. Add recipients

### Configuration

| Field | Description |
|-------|-------------|
| Frequency | None, Daily, or Weekly |
| Recipients | List of notification recipients |

### Adding Recipients

1. Click **+ Add Recipient**
2. Configure:
   - **Name** - Recipient identifier
   - **Channel** - SMS, WhatsApp, or Email
   - **Address** - Phone number or email
   - **Enabled** - Toggle on/off

### Digest Content

Form digests include:

- Form name
- Number of new submissions
- For each submission:
  - Timestamp
  - All field values

Example email digest:

```
New submissions for form: Contact Form

You have 3 new response(s) since the last digest.

--- Response #1 (2024-03-15 10:30:00) ---
Name: John Doe
Email: john@example.com
Message: I'd like to learn more about your services.

--- Response #2 (2024-03-15 14:45:00) ---
Name: Jane Smith
Email: jane@example.com
Message: Do you offer weekend appointments?

...
```

## Booking Digests

Get summaries of new bookings.

### Setup

1. Open calendar editor
2. Go to **Digest** tab
3. Select frequency
4. Add recipients

### Digest Content

Booking digests include:

- Calendar name
- Number of new bookings
- For each booking:
  - Timestamp
  - Customer name and contact
  - Date and time
  - Duration
  - Status (Pending/Confirmed)
  - Notes (if any)

Example:

```
New bookings for calendar: Appointments

You have 2 new booking(s) since the last digest.

--- Booking #1 (2024-03-15 09:00:00) ---
Name: John Doe
Email: john@example.com
Phone: +1234567890
Date: 2024-03-20
Time: 2:00 PM
Duration: 30 minutes
Status: Confirmed

--- Booking #2 (2024-03-15 11:30:00) ---
Name: Jane Smith
Email: jane@example.com
Date: 2024-03-21
Time: 10:00 AM
Duration: 60 minutes
Status: Pending
Notes: First-time consultation
```

## Frequency

| Option | When Sent |
|--------|-----------|
| None | Disabled |
| Daily | Every day (via cron) |
| Weekly | Every week (via cron) |

Digests are sent via Cloudflare's cron trigger. The exact timing depends on your cron configuration in `wrangler.toml`:

```toml
[triggers]
crons = ["0 * * * *"]  # Every hour
```

## Multiple Recipients

Add multiple recipients to send the same digest to different people or channels:

- Send email to admin and SMS to on-call staff
- CC multiple team members
- Use different channels for redundancy
