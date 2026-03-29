-- Tenants
CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT,
    password_hash TEXT,
    plan TEXT DEFAULT 'free',
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_tenants_email ON tenants(email);

-- Contacts (legacy, kept for compatibility)
CREATE TABLE IF NOT EXISTS contacts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    form_slug TEXT NOT NULL DEFAULT 'default',
    name TEXT NOT NULL DEFAULT '',
    email TEXT NOT NULL DEFAULT '',
    message TEXT NOT NULL DEFAULT '',
    fields_data TEXT,
    attachments TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_contacts_form_slug ON contacts(form_slug);
CREATE INDEX IF NOT EXISTS idx_contacts_created_at ON contacts(created_at);

-- Events
CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    all_day INTEGER DEFAULT 0,
    recurrence_rule TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_events_calendar ON events(calendar_id);
CREATE INDEX IF NOT EXISTS idx_events_start ON events(start_time);
CREATE INDEX IF NOT EXISTS idx_events_end ON events(end_time);

-- Time slots
CREATE TABLE IF NOT EXISTS time_slots (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    day_of_week INTEGER,
    specific_date TEXT,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    slot_duration INTEGER DEFAULT 30,
    buffer_time INTEGER DEFAULT 0,
    max_bookings INTEGER DEFAULT 1,
    tenant_id TEXT NOT NULL DEFAULT '',
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_slots_calendar ON time_slots(calendar_id);
CREATE INDEX IF NOT EXISTS idx_slots_day ON time_slots(day_of_week);
CREATE INDEX IF NOT EXISTS idx_slots_date ON time_slots(specific_date);
CREATE INDEX IF NOT EXISTS idx_time_slots_tenant ON time_slots(tenant_id);

-- Bookings
CREATE TABLE IF NOT EXISTS bookings (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    booking_link_id TEXT NOT NULL,
    slot_date TEXT NOT NULL,
    slot_time TEXT NOT NULL,
    duration INTEGER NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    notes TEXT,
    fields_data TEXT,
    status TEXT DEFAULT 'confirmed',
    confirmation_token TEXT,
    tenant_id TEXT NOT NULL DEFAULT '',
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_bookings_calendar ON bookings(calendar_id);
CREATE INDEX IF NOT EXISTS idx_bookings_link ON bookings(booking_link_id);
CREATE INDEX IF NOT EXISTS idx_bookings_date ON bookings(slot_date);
CREATE INDEX IF NOT EXISTS idx_bookings_status ON bookings(status);
CREATE INDEX IF NOT EXISTS idx_bookings_email ON bookings(email);
CREATE INDEX IF NOT EXISTS idx_bookings_tenant ON bookings(tenant_id);

-- Instagram posts
CREATE TABLE IF NOT EXISTS instagram_posts (
    id TEXT PRIMARY KEY,
    calendar_id TEXT,
    form_slug TEXT,
    instagram_source_id TEXT NOT NULL,
    instagram_post_id TEXT NOT NULL UNIQUE,
    instagram_permalink TEXT,
    caption_hash TEXT NOT NULL,
    event_id TEXT,
    contact_id INTEGER,
    event_signature TEXT,
    processing_status TEXT DEFAULT 'pending',
    ai_response TEXT,
    tenant_id TEXT NOT NULL DEFAULT '',
    processed_at TEXT,
    updated_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_instagram_posts_calendar ON instagram_posts(calendar_id);
CREATE INDEX IF NOT EXISTS idx_instagram_posts_form ON instagram_posts(form_slug);
CREATE INDEX IF NOT EXISTS idx_instagram_posts_source ON instagram_posts(instagram_source_id);
CREATE INDEX IF NOT EXISTS idx_instagram_posts_post_id ON instagram_posts(instagram_post_id);
CREATE INDEX IF NOT EXISTS idx_instagram_posts_signature ON instagram_posts(event_signature);
CREATE INDEX IF NOT EXISTS idx_instagram_posts_tenant ON instagram_posts(tenant_id);

-- Event sources
CREATE TABLE IF NOT EXISTS event_sources (
    id TEXT PRIMARY KEY,
    event_id TEXT,
    contact_id INTEGER,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    external_id TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_event_sources_event ON event_sources(event_id);
CREATE INDEX IF NOT EXISTS idx_event_sources_contact ON event_sources(contact_id);
CREATE INDEX IF NOT EXISTS idx_event_sources_source ON event_sources(source_type, source_id);
CREATE INDEX IF NOT EXISTS idx_event_sources_external ON event_sources(external_id);

-- WhatsApp message logging
CREATE TABLE IF NOT EXISTS whatsapp_messages (
    id TEXT PRIMARY KEY,
    whatsapp_account_id TEXT NOT NULL,
    direction TEXT NOT NULL CHECK (direction IN ('inbound', 'outbound')),
    from_number TEXT NOT NULL,
    to_number TEXT NOT NULL,
    body TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_whatsapp_messages_account
    ON whatsapp_messages(whatsapp_account_id, created_at);
CREATE INDEX IF NOT EXISTS idx_whatsapp_messages_tenant
    ON whatsapp_messages(tenant_id, created_at);

-- Form response tracking
CREATE TABLE IF NOT EXISTS form_response_tracking (
    id TEXT PRIMARY KEY,
    form_resource_id TEXT NOT NULL,
    response_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    processed_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_form_response_unique
    ON form_response_tracking(form_resource_id, response_id);
CREATE INDEX IF NOT EXISTS idx_form_response_tenant
    ON form_response_tracking(tenant_id);
