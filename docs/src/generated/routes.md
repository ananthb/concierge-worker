# URL Routes Reference

## Public Routes

| Route | Method | Description |
|-------|--------|-------------|
| `/form/{calendar_id}/{slug}` | GET | Embed Google Form |
| `/book/{calendar_id}/{slug}` | GET | Render booking form |
| `/book/{calendar_id}/{slug}/submit` | POST | Submit booking |
| `/book/{calendar_id}/{slug}/approve/{booking_id}` | POST | Approve pending booking |
| `/book/{calendar_id}/{slug}/deny/{booking_id}` | POST | Deny pending booking |
| `/view/{calendar_id}/{slug}` | GET | Render calendar view |

## Admin Routes

| Route | Method | Description |
|-------|--------|-------------|
| `/admin` | GET | Admin dashboard |
| `/admin/calendars` | POST | Create calendar |
| `/admin/calendars/{id}` | GET | Edit calendar |
| `/admin/calendars/{id}` | PUT | Update calendar |
| `/admin/calendars/{id}` | DELETE | Delete calendar |
| `/admin/calendars/{id}/archive` | POST | Archive calendar |
| `/admin/calendars/{id}/unarchive` | POST | Unarchive calendar |
| `/admin/calendars/{id}/bookings` | GET | View all bookings |
| `/admin/calendars/{id}/slots` | GET | Configure time slots |
| `/admin/calendars/{id}/booking` | POST | Create booking link |
| `/admin/calendars/{id}/booking/{link_id}` | GET | Edit booking link |
| `/admin/calendars/{id}/form` | POST | Create form link |
| `/admin/calendars/{id}/form/{link_id}` | GET | Edit form link |
| `/admin/calendars/{id}/form/{link_id}/responses` | GET | View form responses |
| `/admin/calendars/{id}/view` | POST | Create view link |
| `/admin/calendars/{id}/view/{link_id}` | GET | Edit view link |

## Instagram Routes

| Route | Method | Description |
|-------|--------|-------------|
| `/instagram/auth/{calendar_id}` | GET | Start Instagram OAuth |
| `/instagram/callback` | GET | OAuth callback |
| `/instagram/disconnect/{calendar_id}/{source_id}` | DELETE | Disconnect account |

## Static Assets

| Route | Description |
|-------|-------------|
| `/logo.svg` | Application logo |
