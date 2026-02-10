# Concierge Worker

A Cloudflare Worker in Rust that provides forms, calendars, and event/venue booking. Embed forms and calendars in your sites using HTMX or iframes.

## Features

### Forms
- **Dynamic Forms** - Create forms with custom fields via admin dashboard
- **Field Types** - Text, email, mobile, long text, file upload
- **Customizable Styling** - Full CSS variable control per form
- **File Uploads** - R2 storage with configurable size limits
- **Auto-Responders** - Send acknowledgement messages via SMS, WhatsApp, or Email
- **Digest Notifications** - Receive daily or weekly summaries of submissions

### Calendars & Bookings
- **Multiple Calendars** - Create and manage independent calendars
- **Booking Links** - Public forms for users to book time slots
- **Booking Approval** - Auto-accept or require manual admin approval
- **View Links** - Embeddable calendar views (week, month, year, list)
- **iCal Feeds** - Subscribe to calendars from other apps
- **Instagram Integration** - Auto-import events from posts using AI

### Admin
- **Dashboard** - Manage all forms and calendars in one place
- **Dark Mode** - Automatic dark mode based on system preference
- **Cloudflare Access** - Secure authentication via Zero Trust

## Quick Start

1. [Deploy to Cloudflare](./deployment.md)
2. [Configure secrets](./configuration.md)
3. [Create your first form](./forms.md)
4. [Embed in your site](./embedding-iframe.md)

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Cloudflare Edge                       │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  │
│  │   Worker    │  │     D1      │  │       KV        │  │
│  │   (Rust)    │──│  (SQLite)   │  │   (Key-Value)   │  │
│  └─────────────┘  └─────────────┘  └─────────────────┘  │
│         │                                    │          │
│  ┌─────────────┐                    ┌─────────────────┐ │
│  │     R2      │                    │  Cloudflare AI  │ │
│  │  (Storage)  │                    │   (Instagram)   │ │
│  └─────────────┘                    └─────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

- **D1** - Stores bookings and form submissions
- **KV** - Stores form/calendar configurations
- **R2** - Stores file uploads
- **AI** - Extracts events from Instagram posts
