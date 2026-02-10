# Auto-Responders

Send automatic messages when forms are submitted or bookings are made.

## Prerequisites

Configure at least one notification channel. See [Configuration](./configuration.md).

> **Note:** The Responders tab only appears when channels are configured.

## Form Responders

Send acknowledgement messages to form submitters.

### Setup

1. Open form editor
2. Go to **Responders** tab
3. Click **+ Add Responder**

### Configuration

| Field | Description |
|-------|-------------|
| Name | Internal name |
| Channel | SMS, WhatsApp, or Email |
| Target Field | Form field containing recipient (email/phone) |
| Subject | Email subject (email only) |
| Body | Message content |
| Enabled | Toggle on/off |
| Use AI | Generate response with AI |

### Template Placeholders

Use `{{field_id}}` to insert field values:

```
Hi {{name}},

Thank you for contacting us. We received your message:

"{{message}}"

We'll get back to you at {{email}} soon.
```

## Booking Responders

### Customer Notifications

Send confirmation to customers when bookings are confirmed:

1. Open booking link editor
2. Find **Customer Notifications** section
3. Click **+ Add Responder**

Available placeholders:

| Placeholder | Description |
|-------------|-------------|
| `{{name}}` | Customer name |
| `{{email}}` | Customer email |
| `{{date}}` | Booking date |
| `{{time}}` | Booking time |

Example:

```
Hi {{name}},

Your booking for {{date}} at {{time}} has been confirmed.

Thank you!
```

### Admin Notifications

Receive notifications when bookings need approval:

1. Open booking link editor
2. Disable **Auto-Accept**
3. Find **Admin Notifications** section
4. Click **+ Add Responder**

Additional placeholder:

| Placeholder | Description |
|-------------|-------------|
| `{{approve_url}}` | Link to approve booking |

Example:

```
New booking request:

Name: {{name}}
Email: {{email}}
Date: {{date}}
Time: {{time}}

Approve: {{approve_url}}
```

## AI-Powered Responses

Enable **Use AI** to generate personalized responses:

1. Toggle **Use AI** on
2. Write a system prompt describing desired behavior
3. The AI uses form data to generate contextual responses

Example prompt:

```
You are a helpful assistant for a photography studio.
Thank the customer for their inquiry and provide a brief,
friendly response based on their message.
Keep it under 3 sentences.
```

## Channel-Specific Notes

### SMS

- Keep messages under 160 characters for single-segment
- Longer messages are split and may cost more

### WhatsApp

- Must use approved message templates for first contact
- Can send free-form messages within 24-hour window

### Email

- Include unsubscribe info for marketing emails
- HTML formatting is supported
