# Concierge Worker

A Cloudflare Worker in Rust that provides forms, calendars, and event/venue booking.
Embed forms and calendars in your sites using HTMX or iframes.

**[Documentation](https://ananthb.github.io/concierge-worker/)**

## Features

### Forms
- Custom fields (text, email, phone, file upload)
- Auto-responders via SMS, WhatsApp, or email
- Daily/weekly digest notifications
- Google Sheets sync
- Full CSS customization

### Calendars & Bookings
- Time slot management with capacity limits
- Booking approval workflow (auto-accept or manual)
- Multiple embeddable views (week, month, year, list)
- iCal feeds for external calendar apps
- Instagram event import using AI

### Notifications
- WhatsApp Business API
- Twilio (SMS, WhatsApp, Email via SendGrid)
- Resend Email
- Template placeholders for personalization

### Embedding
- HTMX for seamless integration
- iframe for simple embedding
- CSS variables for theming
- Query parameters for customization

### Admin
- Cloudflare Access authentication
- Dashboard for managing forms and calendars
- Automatic dark mode

## Building Documentation

```bash
# Enter dev environment
nix develop

# Extract info from code and build
./docs/extract-docs.sh
mdbook build docs

# Or preview locally
mdbook serve docs
```

## License

AGPL3 - see [LICENSE](LICENSE) for the full license text.
