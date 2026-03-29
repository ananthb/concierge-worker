# How It Works

Concierge connects four services you already use and makes them work together automatically.

## The Pieces

| Service | What It Does in Concierge |
|---------|--------------------------|
| **Google Calendar** | Your source of truth for events. Concierge reads events from it and creates events in it. |
| **Instagram** | Post about an event and Concierge extracts the details using AI and adds it to your Google Calendar. |
| **WhatsApp** | Customers get instant booking confirmations. You get daily digests of new bookings. |
| **Google Forms** | Contact forms, intake questionnaires — embedded on your site, responses viewable in admin. |

## The Flow

### Events

1. You post on Instagram about an upcoming event
2. Concierge's AI reads the caption, extracts the date/time/title
3. A new event is created in your Google Calendar
4. The event automatically appears on your website's embedded calendar

### Bookings

1. You configure your available time slots once (e.g., Mon-Fri 9am-5pm, 30min slots)
2. A customer visits your booking page and picks a time
3. Concierge checks availability, prevents double-booking
4. The customer gets a WhatsApp confirmation
5. A Google Calendar event is created for the appointment
6. You get a daily digest of all new bookings on WhatsApp

### Forms

1. You create a Google Form (contact form, questionnaire, etc.)
2. You add it as a Form Link in Concierge
3. The form is embeddable on your website
4. You can view responses in the admin dashboard

## What You Need

- A **Google Cloud service account** (free) for Calendar and Forms access
- A **Meta WhatsApp Business** account for messaging
- An **Instagram** account (optional, for event import)
- A **Cloudflare** account to run Concierge

## What You Don't Need

- Any coding skills to use day-to-day
- To check multiple dashboards
- To manually copy event details between platforms
- To respond to booking confirmations yourself
