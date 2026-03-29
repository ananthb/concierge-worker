# Concierge

Automated customer engagement for small businesses. WhatsApp replies, event calendars, Instagram integration — zero effort.

**[Documentation](https://ananthb.github.io/concierge-worker/)**

## What It Does

- **Instagram to Calendar** — Post about an event, AI extracts the details, Google Calendar updates automatically
- **Booking + WhatsApp** — Customers book appointments, get instant WhatsApp confirmations
- **Google Forms** — Embed contact forms, view responses in the admin
- **Daily Digests** — Get a WhatsApp summary of new bookings every morning

## Built With

- [Cloudflare Workers](https://workers.cloudflare.com/) (Rust + WebAssembly)
- Google Calendar API / Google Forms API
- Meta WhatsApp Business API
- Instagram Basic Display API
- Cloudflare AI, D1, KV

## Building Documentation

```bash
nix develop
./docs/extract-docs.sh
mdbook serve docs
```

## License

AGPL-3.0
