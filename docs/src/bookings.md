# Booking & Appointments

Let customers book time slots directly from your website. Concierge handles availability, prevents double-booking, sends confirmations, and creates Google Calendar events.

## Setting Up

### 1. Configure Time Slots

Time slots define when you're available for bookings.

Go to your calendar editor > **Bookings** tab > **Configure Available Slots**.

| Field | Description |
|-------|-------------|
| Day of Week | Monday, Tuesday, etc. |
| Start Time | When slots begin (e.g., 09:00) |
| End Time | When slots end (e.g., 17:00) |
| Slot Duration | Length of each slot in minutes (e.g., 30) |
| Max Bookings | How many people can book the same slot (e.g., 1) |

You can set different availability for each day of the week.

### 2. Create a Booking Link

Go to **Bookings** tab > **+ Add Booking Link**.

| Setting | Description |
|---------|-------------|
| Name | What customers see (e.g., "30-Minute Consultation") |
| Duration | How long each appointment lasts |
| Min Notice | Minimum hours before a slot can be booked (e.g., 24) |
| Max Advance | How many days ahead customers can book (e.g., 30) |
| Auto-Accept | Confirm bookings instantly, or require your approval |
| Confirmation Message | Shown after booking |
| Hide Title | Hide the name when embedded |

### 3. Embed on Your Website

```html
<iframe src="https://your-worker.workers.dev/book/{calendar_id}/{slug}"
        style="border: none; width: 100%; min-height: 600px;"></iframe>
```

## How Booking Works

### Auto-Accept Flow

1. Customer picks a date and time
2. Booking is confirmed immediately
3. Customer gets a WhatsApp confirmation (if configured)
4. Event is created in Google Calendar (if configured)

### Manual Approval Flow

1. Customer picks a date and time
2. Booking is created as "Pending"
3. You receive a WhatsApp message with approve/deny links
4. You click **Approve** or **Deny**
5. Customer gets a WhatsApp notification of the result
6. If approved, event is created in Google Calendar

## Overbooking Prevention

Concierge automatically prevents overbooking:

- Tracks both confirmed and pending bookings against slot capacity
- Hides fully-booked slots from the booking form
- Enforces buffer times between appointments

## Custom Fields

Add custom fields to your booking form to collect extra information:

- Name, Email (built-in)
- Phone, Notes
- Custom text fields

## Viewing Bookings

Go to your calendar editor > **Bookings** tab > **View All Bookings** to see upcoming bookings and cancel if needed.
