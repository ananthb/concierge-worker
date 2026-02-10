# URL Routes Reference

This page is auto-generated from the source code.

## Public Routes

| Route | Method | Description |
|-------|--------|-------------|
| `/f/{slug}` | GET | Render public form |
| `/f/{slug}/submit` | POST | Submit form |
| `/book/{calendar_id}/{slug}` | GET | Render booking form |
| `/book/{calendar_id}/{slug}/submit` | POST | Submit booking |
| `/book/{calendar_id}/{slug}/approve/{booking_id}` | GET | Approve pending booking |
| `/book/{calendar_id}/{slug}/cancel/{booking_id}` | GET | Cancel booking |
| `/view/{calendar_id}/{slug}` | GET | Render calendar view |
| `/feed/{calendar_id}/{slug}` | GET | iCal feed |

## Admin Routes

| Route | Method | Description |
|-------|--------|-------------|
| `/admin` | GET | Admin dashboard |
| `/admin/forms/new` | GET | New form editor |
| `/admin/forms/{slug}` | GET | Edit form |
| `/admin/forms/{slug}` | PUT | Update form |
| `/admin/forms/{slug}` | DELETE | Delete form |
| `/admin/forms/{slug}/responses` | GET | View form responses |
| `/admin/forms/{slug}/archive` | POST | Archive form |
| `/admin/forms/{slug}/unarchive` | POST | Unarchive form |
| `/admin/calendars` | POST | Create calendar |
| `/admin/calendars/{id}` | GET | Edit calendar |
| `/admin/calendars/{id}` | PUT | Update calendar |
| `/admin/calendars/{id}` | DELETE | Delete calendar |
| `/admin/calendars/{id}/archive` | POST | Archive calendar |
| `/admin/calendars/{id}/unarchive` | POST | Unarchive calendar |
| `/admin/calendars/{id}/bookings` | GET | View all bookings |
| `/admin/calendars/{id}/slots` | GET | Configure time slots |
| `/admin/calendars/{id}/events` | GET | Manage events |
| `/admin/calendars/{id}/booking` | POST | Create booking link |
| `/admin/calendars/{id}/booking/{link_id}` | GET | Edit booking link |
| `/admin/calendars/{id}/view` | POST | Create view link |
| `/admin/calendars/{id}/view/{link_id}` | GET | Edit view link |
| `/admin/calendars/{id}/feed` | POST | Create feed link |

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
