# WhatsApp Automation

Concierge uses the Meta WhatsApp Business API to send messages to your customers automatically.

## What Gets Sent

### Booking Confirmations

When a customer books an appointment and auto-accept is enabled, they receive a WhatsApp message immediately with their booking details.

### Booking Approval Requests

When auto-accept is disabled, you (the admin) receive a WhatsApp message with the booking details and links to approve or deny.

### Booking Denials

If you deny a booking, the customer receives a WhatsApp message explaining that their request couldn't be approved.

### Digest Notifications

Daily or weekly summaries of new bookings sent to your WhatsApp. See [Digests](./digests.md).

## Setting Up Responders

Each booking link has two types of responders:

### Customer Responders

Notify the customer after their booking is confirmed.

| Field | Description |
|-------|-------------|
| **Name** | Label for this responder |
| **Target Field** | Which booking field contains the customer's phone number (e.g., `phone`) |
| **Body** | Message template. Use `{{name}}`, `{{date}}`, `{{time}}` placeholders. |
| **AI** | Enable to generate personalized messages using AI |

### Admin Responders

Notify you when a booking needs approval.

| Field | Description |
|-------|-------------|
| **Name** | Label for this responder |
| **Target Field** | Your WhatsApp phone number |
| **Body** | Message template. Use `{{name}}`, `{{email}}`, `{{date}}`, `{{time}}`, `{{approve_url}}`, `{{deny_url}}` placeholders. |

## Message Templates

Use `{{field_name}}` placeholders in your message body:

```
Hi {{name}}, your appointment on {{date}} at {{time}} is confirmed!
```

Available placeholders for customer messages:
- `{{name}}`, `{{email}}`, `{{date}}`, `{{time}}`
- Any custom booking field by its ID

Available placeholders for admin messages:
- All of the above plus `{{approve_url}}`, `{{deny_url}}`, `{{event}}`, `{{duration}}`

## AI-Generated Messages

Enable the **AI** toggle on a responder to have Concierge generate personalized messages using the body as a prompt. The AI uses the booking details as context.

## Requirements

- Meta WhatsApp Business API access token
- WhatsApp Business phone number ID

See [Configuration](./configuration.md) for setup instructions.
