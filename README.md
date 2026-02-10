# Concierge Worker

A Cloudflare Worker in Rust that provides forms, calendars, and event/venue booking.
Embed forms and calendars in your sites using HTMX or iframes.

**[Documentation](https://ananthb.github.io/concierge-worker/)**

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
