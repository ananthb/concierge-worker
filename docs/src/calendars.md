# Calendars & Bookings

Create calendars with time slots for appointments, events, or venue bookings.

## Creating a Calendar

1. Go to the admin dashboard (`/admin`)
2. Click **+ New Calendar**
3. Configure calendar settings

## Calendar Settings

### Basic Settings

| Field | Description |
|-------|-------------|
| Name | Calendar name |
| Description | Optional description |
| Timezone | Calendar timezone |
| Allowed Domains | Domains that can embed |
| Custom CSS | Additional styling |

## Time Slots

Configure when bookings are available:

1. Go to calendar editor
2. Click **Bookings** tab
3. Click **Configure Available Slots**

### Recurring Slots

Set weekly availability:

| Field | Description |
|-------|-------------|
| Day of Week | Monday, Tuesday, etc. |
| Start Time | When slots begin |
| End Time | When slots end |
| Duration | Length of each slot (minutes) |
| Capacity | Max bookings per slot |

### Specific Date Slots

Add availability for specific dates (overrides weekly pattern).

## Booking Links

Public pages where users can book time slots.

### Creating a Booking Link

1. Go to calendar editor
2. Click **Bookings** tab
3. Click **+ Add Booking Link**

### Booking Link Settings

| Field | Description |
|-------|-------------|
| Name | Link name |
| Slug | URL path |
| Duration | Booking duration (minutes) |
| Confirmation Message | Shown after booking |
| Hide Title | Hide name when embedded |
| Auto-Accept | Confirm immediately or require approval |

### Custom Fields

Add fields to collect information during booking:

- Name, Email, Phone (built-in)
- Custom text fields
- Notes/comments

## Booking Approval

When **Auto-Accept** is disabled:

1. Booking is created with "Pending" status
2. Admin receives notification (if configured)
3. Admin clicks approval link
4. Customer receives confirmation

## View Links

Public calendar views that can be embedded or shared.

### View Types

| Type | Description |
|------|-------------|
| Week | 7-day view with time grid |
| Month | Monthly calendar grid |
| Year | Yearly overview |
| List | Scrolling list of events |

### Creating a View Link

1. Go to calendar editor
2. Click **Settings** tab
3. Click **+ Add View Link**

## iCal Feeds

Subscribe to the calendar from other apps (Google Calendar, Apple Calendar, etc.):

1. Go to calendar editor
2. Click **Settings** tab
3. Click **+ Add Feed Link**
4. Copy the feed URL with token

## Events

### Manual Events

Create events directly:

1. Go to calendar editor
2. Click **Events** tab
3. Click **Open Event Editor**

### Instagram Integration

Automatically import events from Instagram posts:

1. Configure Instagram secrets (see [Configuration](./configuration.md))
2. Go to **Events** tab
3. Click **Connect Instagram Account**
4. Authorize access
5. Events are synced hourly via cron trigger
