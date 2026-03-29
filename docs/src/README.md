# Concierge

**Automated customer engagement for small businesses.**

Concierge connects your Google Calendar, Instagram, WhatsApp, and Google Forms into one system that runs itself. Your customers get instant replies, your events stay up to date, and you don't have to lift a finger.

---

## The Problem

You're running a business. You post events on Instagram, manage bookings in Google Calendar, take inquiries through forms, and reply to customers on WhatsApp. Every one of these is a separate thing you have to check, update, and respond to — and when you're busy, things slip through the cracks.

## The Solution

Concierge wires everything together so it works automatically:

**Post on Instagram** and your event calendar updates itself. **A customer books an appointment** and they get a WhatsApp confirmation instantly. **Someone fills out your contact form** and it flows straight into your workflow. **New bookings come in overnight** and you get a daily digest on WhatsApp so you know what's on your plate.

No dashboards to check. No copy-pasting between apps. It just works.

---

## What You Get

### Instant WhatsApp Replies

When a customer books an appointment or needs approval, Concierge sends them a WhatsApp message automatically. Confirmations, denials, approval requests to you — all handled without you touching your phone.

[Learn more](./whatsapp.md)

### Event Calendars That Update Themselves

Connect your Google Calendar and your events show up on an embeddable calendar on your website. Post about an event on Instagram and it gets added to your calendar automatically using AI that reads your captions.

[Learn more](./calendars.md)

### Booking & Appointments

Let customers book time slots directly from your website. Set your availability once — Concierge handles the rest. Overbooking prevention, buffer times between appointments, capacity limits per slot. Choose auto-accept or review each booking yourself.

[Learn more](./bookings.md)

### Google Forms Integration

Use Google Forms for contact forms, intake questionnaires, or feedback. Embed them on your website with your branding. View responses right from the admin dashboard — no need to open Google Forms separately.

[Learn more](./forms.md)

### Instagram Event Import

Connect your Instagram account and Concierge uses AI to read your post captions, extract event details (date, time, title, description), and create Google Calendar events automatically. Post once, update everywhere.

[Learn more](./instagram.md)

### Daily Digests

Get a WhatsApp summary of new bookings — daily or weekly. Know what's coming up without logging into anything.

[Learn more](./digests.md)

---

## How It Works

```
Instagram Post
     |
     v
  [  AI  ] ──> Google Calendar ──> Your Website
                    ^                    |
                    |                    v
              Booking Form ──> WhatsApp Confirmation
                                   |
                                   v
                             Daily Digest to You
```

1. **Connect your accounts** — Google Calendar, Instagram, WhatsApp
2. **Set your availability** — Configure time slots and booking rules
3. **Embed on your site** — Drop a single line of HTML for calendars, booking forms, and contact forms
4. **Let it run** — Concierge handles the rest automatically

[Get started](./setup.md)

---

## Built For Small Businesses

Concierge is designed for businesses that:

- **Promote events on Instagram** — restaurants, venues, studios, gyms, salons
- **Take appointments** — consultants, therapists, tutors, personal trainers
- **Need a contact form** — service businesses, freelancers, agencies
- **Communicate on WhatsApp** — any business where customers expect instant replies

You don't need technical skills to use it. If you can share a Google Calendar and connect an Instagram account, you're set.

---

## Technical Details

Concierge runs on [Cloudflare Workers](https://workers.cloudflare.com/) — fast, globally distributed, and serverless. It's written in Rust and compiles to WebAssembly.

- **Google Calendar API** for event management
- **Google Forms API** for form responses
- **Meta WhatsApp Business API** for messaging
- **Instagram Basic Display API** for post import
- **Cloudflare AI** for extracting events from Instagram captions
- **Cloudflare D1** (SQLite) for bookings and time slots
- **Cloudflare KV** for configuration

The admin dashboard is protected by [Cloudflare Access](https://www.cloudflare.com/zero-trust/products/access/) (Zero Trust authentication).

Source code: [github.com/ananthb/concierge-worker](https://github.com/ananthb/concierge-worker)

License: AGPL-3.0
