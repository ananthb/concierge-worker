use wasm_bindgen::JsValue;
use worker::*;

use crate::types::{
    Booking, BookingStatus, CalendarConfig, GoogleFormResource, InstagramAccount, ProcessedPost,
    ProcessingStatus, Tenant, TenantCredentials, TimeSlot, WhatsAppAccount,
    WhatsAppAccountCredentials,
};

// ============================================================================
// Calendar KV Operations (CALENDARS_KV)
// ============================================================================

pub async fn get_calendar(kv: &kv::KvStore, id: &str) -> Result<Option<CalendarConfig>> {
    kv.get(&format!("calendar:{}", id))
        .json::<CalendarConfig>()
        .await
        .map_err(|e| Error::from(e.to_string()))
}

pub async fn save_calendar(kv: &kv::KvStore, calendar: &CalendarConfig) -> Result<()> {
    // Primary key for public lookups
    kv.put(&format!("calendar:{}", calendar.id), calendar)?
        .execute()
        .await?;
    // Tenant index for admin listing
    if !calendar.tenant_id.is_empty() {
        kv.put(
            &format!("tenant:{}:calendar:{}", calendar.tenant_id, calendar.id),
            "",
        )?
        .execute()
        .await?;
    }
    Ok(())
}

pub async fn delete_calendar(kv: &kv::KvStore, tenant_id: &str, id: &str) -> Result<()> {
    kv.delete(&format!("calendar:{}", id)).await?;
    if !tenant_id.is_empty() {
        kv.delete(&format!("tenant:{}:calendar:{}", tenant_id, id))
            .await?;
    }
    Ok(())
}

pub async fn list_calendars(kv: &kv::KvStore, tenant_id: &str) -> Result<Vec<CalendarConfig>> {
    let prefix = if tenant_id.is_empty() {
        "calendar:".to_string()
    } else {
        format!("tenant:{}:calendar:", tenant_id)
    };

    let list = kv
        .list()
        .prefix(prefix.clone())
        .execute()
        .await
        .map_err(|e| Error::from(e.to_string()))?;
    let mut calendars = Vec::new();

    for key in list.keys {
        // Extract calendar_id from key
        let calendar_id = if tenant_id.is_empty() {
            key.name.strip_prefix("calendar:").unwrap_or("").to_string()
        } else {
            key.name.strip_prefix(&prefix).unwrap_or("").to_string()
        };

        if calendar_id.is_empty() {
            continue;
        }

        // Load full config from primary key
        if let Some(calendar) = kv
            .get(&format!("calendar:{}", calendar_id))
            .json::<CalendarConfig>()
            .await
            .map_err(|e| Error::from(e.to_string()))?
        {
            calendars.push(calendar);
        }
    }

    calendars.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(calendars)
}

// ============================================================================
// Tenant KV Operations
// ============================================================================

pub async fn get_tenant(kv: &kv::KvStore, id: &str) -> Result<Option<Tenant>> {
    kv.get(&format!("tenant:{}", id))
        .json::<Tenant>()
        .await
        .map_err(|e| Error::from(e.to_string()))
}

pub async fn get_tenant_by_email(kv: &kv::KvStore, email: &str) -> Result<Option<Tenant>> {
    // Look up tenant ID by email index
    let tenant_id = kv
        .get(&format!("tenant_email:{}", email.to_lowercase()))
        .text()
        .await
        .map_err(|e| Error::from(e.to_string()))?;

    match tenant_id {
        Some(id) => get_tenant(kv, &id).await,
        None => Ok(None),
    }
}

pub async fn save_tenant(kv: &kv::KvStore, tenant: &Tenant) -> Result<()> {
    kv.put(&format!("tenant:{}", tenant.id), tenant)?
        .execute()
        .await?;
    // Email index for lookups
    kv.put(
        &format!("tenant_email:{}", tenant.email.to_lowercase()),
        &tenant.id,
    )?
    .execute()
    .await?;
    Ok(())
}

pub async fn save_tenant_credentials(
    kv: &kv::KvStore,
    tenant_id: &str,
    credentials: &TenantCredentials,
    encryption_key: &str,
) -> Result<()> {
    let json = serde_json::to_string(credentials)
        .map_err(|e| Error::from(format!("Failed to serialize credentials: {}", e)))?;
    let encrypted = crate::crypto::encrypt_string(&json, encryption_key).await?;
    kv.put(&format!("tenant:{}:credentials", tenant_id), &encrypted)?
        .execute()
        .await?;
    Ok(())
}

pub async fn get_tenant_credentials(
    kv: &kv::KvStore,
    tenant_id: &str,
    encryption_key: &str,
) -> Result<TenantCredentials> {
    let encrypted = kv
        .get(&format!("tenant:{}:credentials", tenant_id))
        .text()
        .await
        .map_err(|e| Error::from(e.to_string()))?;

    match encrypted {
        Some(enc) => {
            let json = crate::crypto::decrypt_string(&enc, encryption_key).await?;
            serde_json::from_str(&json)
                .map_err(|e| Error::from(format!("Failed to deserialize credentials: {}", e)))
        }
        None => Ok(TenantCredentials::default()),
    }
}

// ============================================================================
// Session KV Operations
// ============================================================================

pub async fn save_session(
    kv: &kv::KvStore,
    token: &str,
    tenant_id: &str,
    ttl_seconds: u64,
) -> Result<()> {
    kv.put(&format!("session:{}", token), tenant_id)?
        .expiration_ttl(ttl_seconds)
        .execute()
        .await?;
    Ok(())
}

pub async fn get_session(kv: &kv::KvStore, token: &str) -> Result<Option<String>> {
    kv.get(&format!("session:{}", token))
        .text()
        .await
        .map_err(|e| Error::from(e.to_string()))
}

pub async fn delete_session(kv: &kv::KvStore, token: &str) -> Result<()> {
    kv.delete(&format!("session:{}", token)).await?;
    Ok(())
}

// ============================================================================
// D1 Operations (Time Slots)
// ============================================================================

pub async fn get_time_slots(db: &D1Database, calendar_id: &str) -> Result<Vec<TimeSlot>> {
    let stmt = db.prepare(
        "SELECT id, calendar_id, day_of_week, specific_date, start_time, end_time,
                slot_duration, buffer_time, max_bookings
         FROM time_slots
         WHERE calendar_id = ?
         ORDER BY day_of_week ASC, start_time ASC",
    );

    let results = stmt.bind(&[calendar_id.into()])?.all().await?;

    let mut slots = Vec::new();
    for row in results.results::<serde_json::Value>()? {
        if let Ok(slot) = serde_json::from_value::<TimeSlot>(row) {
            slots.push(slot);
        }
    }
    Ok(slots)
}

pub async fn save_time_slot(db: &D1Database, slot: &TimeSlot, tenant_id: &str) -> Result<()> {
    let stmt = db.prepare(
        "INSERT OR REPLACE INTO time_slots
         (id, calendar_id, day_of_week, specific_date, start_time, end_time, slot_duration, buffer_time, max_bookings, tenant_id, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))",
    );

    stmt.bind(&[
        slot.id.clone().into(),
        slot.calendar_id.clone().into(),
        slot.day_of_week.map(|d| d.into()).unwrap_or(JsValue::NULL),
        slot.specific_date
            .clone()
            .map(|s| s.into())
            .unwrap_or(JsValue::NULL),
        slot.start_time.clone().into(),
        slot.end_time.clone().into(),
        slot.slot_duration.into(),
        slot.buffer_time.into(),
        slot.max_bookings.into(),
        tenant_id.into(),
    ])?
    .run()
    .await?;

    Ok(())
}

pub async fn delete_time_slot(db: &D1Database, id: &str) -> Result<()> {
    let stmt = db.prepare("DELETE FROM time_slots WHERE id = ?");
    stmt.bind(&[id.into()])?.run().await?;
    Ok(())
}

// ============================================================================
// D1 Operations (Bookings)
// ============================================================================

pub async fn get_bookings(
    db: &D1Database,
    calendar_id: &str,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<Booking>> {
    let stmt = db.prepare(
        "SELECT id, calendar_id, booking_link_id, slot_date, slot_time, duration,
                name, email, phone, notes, fields_data, status, confirmation_token,
                created_at, updated_at
         FROM bookings
         WHERE calendar_id = ? AND slot_date >= ? AND slot_date <= ? AND status != 'cancelled'
         ORDER BY slot_date ASC, slot_time ASC",
    );

    let results = stmt
        .bind(&[calendar_id.into(), start_date.into(), end_date.into()])?
        .all()
        .await?;

    let mut bookings = Vec::new();
    for row in results.results::<serde_json::Value>()? {
        if let Ok(booking) = serde_json::from_value::<Booking>(row) {
            bookings.push(booking);
        }
    }
    Ok(bookings)
}

pub async fn get_booking(db: &D1Database, id: &str) -> Result<Option<Booking>> {
    let stmt = db.prepare(
        "SELECT id, calendar_id, booking_link_id, slot_date, slot_time, duration,
                name, email, phone, notes, fields_data, status, confirmation_token,
                created_at, updated_at
         FROM bookings WHERE id = ?",
    );

    let results = stmt.bind(&[id.into()])?.all().await?;
    let rows = results.results::<serde_json::Value>()?;

    if let Some(row) = rows.into_iter().next() {
        Ok(serde_json::from_value(row).ok())
    } else {
        Ok(None)
    }
}

pub async fn get_bookings_since(
    db: &D1Database,
    calendar_id: &str,
    since: &str,
) -> Result<Vec<Booking>> {
    let stmt = db.prepare(
        "SELECT id, calendar_id, booking_link_id, slot_date, slot_time, duration,
                name, email, phone, notes, fields_data, status, confirmation_token,
                created_at, updated_at
         FROM bookings
         WHERE calendar_id = ? AND created_at > ?
         ORDER BY created_at ASC",
    );

    let results = stmt
        .bind(&[calendar_id.into(), since.into()])?
        .all()
        .await?;

    let mut bookings = Vec::new();
    for row in results.results::<serde_json::Value>()? {
        if let Ok(booking) = serde_json::from_value::<Booking>(row) {
            bookings.push(booking);
        }
    }
    Ok(bookings)
}

pub async fn save_booking(db: &D1Database, booking: &Booking, tenant_id: &str) -> Result<()> {
    let status_str = match booking.status {
        BookingStatus::Pending => "pending",
        BookingStatus::Confirmed => "confirmed",
        BookingStatus::Cancelled => "cancelled",
        BookingStatus::Completed => "completed",
    };

    let stmt = db.prepare(
        "INSERT OR REPLACE INTO bookings
         (id, calendar_id, booking_link_id, slot_date, slot_time, duration, name, email, phone, notes, fields_data, status, confirmation_token, tenant_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    );

    stmt.bind(&[
        booking.id.clone().into(),
        booking.calendar_id.clone().into(),
        booking.booking_link_id.clone().into(),
        booking.slot_date.clone().into(),
        booking.slot_time.clone().into(),
        booking.duration.into(),
        booking.name.clone().into(),
        booking.email.clone().into(),
        booking.phone.clone().unwrap_or_default().into(),
        booking.notes.clone().unwrap_or_default().into(),
        booking
            .fields_data
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_default()
            .into(),
        status_str.into(),
        booking
            .confirmation_token
            .clone()
            .unwrap_or_default()
            .into(),
        tenant_id.into(),
        booking.created_at.clone().into(),
        booking.updated_at.clone().into(),
    ])?
    .run()
    .await?;

    Ok(())
}

pub async fn count_bookings_for_slot(
    db: &D1Database,
    calendar_id: &str,
    date: &str,
    time: &str,
) -> Result<i32> {
    let stmt = db.prepare(
        "SELECT COUNT(*) as count FROM bookings
         WHERE calendar_id = ? AND slot_date = ? AND slot_time = ? AND status IN ('confirmed', 'pending')",
    );

    let results = stmt
        .bind(&[calendar_id.into(), date.into(), time.into()])?
        .all()
        .await?;

    let rows = results.results::<serde_json::Value>()?;
    if let Some(row) = rows.into_iter().next() {
        Ok(row.get("count").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
    } else {
        Ok(0)
    }
}

// ============================================================================
// WhatsApp Account KV Operations
// ============================================================================

pub async fn get_whatsapp_account(kv: &kv::KvStore, id: &str) -> Result<Option<WhatsAppAccount>> {
    kv.get(&format!("whatsapp:{}", id))
        .json::<WhatsAppAccount>()
        .await
        .map_err(|e| Error::from(e.to_string()))
}

pub async fn save_whatsapp_account(kv: &kv::KvStore, account: &WhatsAppAccount) -> Result<()> {
    kv.put(&format!("whatsapp:{}", account.id), account)?
        .execute()
        .await?;
    if !account.tenant_id.is_empty() {
        kv.put(
            &format!("tenant:{}:whatsapp:{}", account.tenant_id, account.id),
            "",
        )?
        .execute()
        .await?;
    }
    Ok(())
}

pub async fn delete_whatsapp_account(kv: &kv::KvStore, tenant_id: &str, id: &str) -> Result<()> {
    kv.delete(&format!("whatsapp:{}", id)).await?;
    if !tenant_id.is_empty() {
        kv.delete(&format!("tenant:{}:whatsapp:{}", tenant_id, id))
            .await?;
    }
    // Clean up credentials and reverse index
    kv.delete(&format!("whatsapp:{}:credentials", id)).await?;
    Ok(())
}

pub async fn list_whatsapp_accounts(
    kv: &kv::KvStore,
    tenant_id: &str,
) -> Result<Vec<WhatsAppAccount>> {
    let prefix = format!("tenant:{}:whatsapp:", tenant_id);
    let list = kv
        .list()
        .prefix(prefix.clone())
        .execute()
        .await
        .map_err(|e| Error::from(e.to_string()))?;

    let mut accounts = Vec::new();
    for key in list.keys {
        let account_id = key.name.strip_prefix(&prefix).unwrap_or("").to_string();
        if account_id.is_empty() {
            continue;
        }
        if let Some(account) = get_whatsapp_account(kv, &account_id).await? {
            accounts.push(account);
        }
    }
    accounts.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(accounts)
}

pub async fn get_whatsapp_credentials(
    kv: &kv::KvStore,
    account_id: &str,
    encryption_key: &str,
) -> Result<Option<WhatsAppAccountCredentials>> {
    let encrypted = kv
        .get(&format!("whatsapp:{}:credentials", account_id))
        .text()
        .await
        .map_err(|e| Error::from(e.to_string()))?;

    match encrypted {
        Some(enc) => {
            let json = crate::crypto::decrypt_string(&enc, encryption_key).await?;
            let creds: WhatsAppAccountCredentials = serde_json::from_str(&json)
                .map_err(|e| Error::from(format!("Failed to deserialize WA credentials: {}", e)))?;
            Ok(Some(creds))
        }
        None => Ok(None),
    }
}

pub async fn save_whatsapp_credentials(
    kv: &kv::KvStore,
    account_id: &str,
    credentials: &WhatsAppAccountCredentials,
    encryption_key: &str,
) -> Result<()> {
    let json = serde_json::to_string(credentials)
        .map_err(|e| Error::from(format!("Failed to serialize WA credentials: {}", e)))?;
    let encrypted = crate::crypto::encrypt_string(&json, encryption_key).await?;
    kv.put(&format!("whatsapp:{}:credentials", account_id), &encrypted)?
        .execute()
        .await?;
    // Reverse index: phone_number_id -> whatsapp account id (for webhook routing)
    kv.put(
        &format!("wa_phone:{}", credentials.phone_number_id),
        account_id,
    )?
    .execute()
    .await?;
    Ok(())
}

pub async fn get_whatsapp_account_by_phone(
    kv: &kv::KvStore,
    phone_number_id: &str,
) -> Result<Option<String>> {
    kv.get(&format!("wa_phone:{}", phone_number_id))
        .text()
        .await
        .map_err(|e| Error::from(e.to_string()))
}

// ============================================================================
// Google Form Resource KV Operations
// ============================================================================

pub async fn get_form_resource(kv: &kv::KvStore, id: &str) -> Result<Option<GoogleFormResource>> {
    kv.get(&format!("form:{}", id))
        .json::<GoogleFormResource>()
        .await
        .map_err(|e| Error::from(e.to_string()))
}

pub async fn save_form_resource(kv: &kv::KvStore, form: &GoogleFormResource) -> Result<()> {
    kv.put(&format!("form:{}", form.id), form)?
        .execute()
        .await?;
    if !form.tenant_id.is_empty() {
        kv.put(&format!("tenant:{}:form:{}", form.tenant_id, form.id), "")?
            .execute()
            .await?;
    }
    Ok(())
}

pub async fn delete_form_resource(kv: &kv::KvStore, tenant_id: &str, id: &str) -> Result<()> {
    kv.delete(&format!("form:{}", id)).await?;
    if !tenant_id.is_empty() {
        kv.delete(&format!("tenant:{}:form:{}", tenant_id, id))
            .await?;
    }
    Ok(())
}

pub async fn list_form_resources(
    kv: &kv::KvStore,
    tenant_id: &str,
) -> Result<Vec<GoogleFormResource>> {
    let prefix = format!("tenant:{}:form:", tenant_id);
    let list = kv
        .list()
        .prefix(prefix.clone())
        .execute()
        .await
        .map_err(|e| Error::from(e.to_string()))?;

    let mut forms = Vec::new();
    for key in list.keys {
        let form_id = key.name.strip_prefix(&prefix).unwrap_or("").to_string();
        if form_id.is_empty() {
            continue;
        }
        if let Some(form) = get_form_resource(kv, &form_id).await? {
            forms.push(form);
        }
    }
    forms.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(forms)
}

// ============================================================================
// Instagram Account Resource KV Operations
// ============================================================================

pub async fn get_instagram_account(kv: &kv::KvStore, id: &str) -> Result<Option<InstagramAccount>> {
    kv.get(&format!("instagram:{}", id))
        .json::<InstagramAccount>()
        .await
        .map_err(|e| Error::from(e.to_string()))
}

pub async fn save_instagram_account(kv: &kv::KvStore, account: &InstagramAccount) -> Result<()> {
    kv.put(&format!("instagram:{}", account.id), account)?
        .execute()
        .await?;
    if !account.tenant_id.is_empty() {
        kv.put(
            &format!("tenant:{}:instagram:{}", account.tenant_id, account.id),
            "",
        )?
        .execute()
        .await?;
    }
    Ok(())
}

pub async fn delete_instagram_account(kv: &kv::KvStore, tenant_id: &str, id: &str) -> Result<()> {
    kv.delete(&format!("instagram:{}", id)).await?;
    if !tenant_id.is_empty() {
        kv.delete(&format!("tenant:{}:instagram:{}", tenant_id, id))
            .await?;
    }
    // Clean up token
    kv.delete(&format!("instagram_token:{}", id)).await?;
    Ok(())
}

pub async fn list_instagram_accounts(
    kv: &kv::KvStore,
    tenant_id: &str,
) -> Result<Vec<InstagramAccount>> {
    let prefix = format!("tenant:{}:instagram:", tenant_id);
    let list = kv
        .list()
        .prefix(prefix.clone())
        .execute()
        .await
        .map_err(|e| Error::from(e.to_string()))?;

    let mut accounts = Vec::new();
    for key in list.keys {
        let account_id = key.name.strip_prefix(&prefix).unwrap_or("").to_string();
        if account_id.is_empty() {
            continue;
        }
        if let Some(account) = get_instagram_account(kv, &account_id).await? {
            accounts.push(account);
        }
    }
    accounts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(accounts)
}

// ============================================================================
// D1 Operations (WhatsApp Messages)
// ============================================================================

#[allow(clippy::too_many_arguments)]
pub async fn save_whatsapp_message(
    db: &D1Database,
    id: &str,
    whatsapp_account_id: &str,
    direction: &str,
    from_number: &str,
    to_number: &str,
    body: &str,
    tenant_id: &str,
) -> Result<()> {
    let stmt = db.prepare(
        "INSERT INTO whatsapp_messages (id, whatsapp_account_id, direction, from_number, to_number, body, tenant_id, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))",
    );
    stmt.bind(&[
        id.into(),
        whatsapp_account_id.into(),
        direction.into(),
        from_number.into(),
        to_number.into(),
        body.into(),
        tenant_id.into(),
    ])?
    .run()
    .await?;
    Ok(())
}

// ============================================================================
// D1 Operations (Form Response Tracking)
// ============================================================================

pub async fn is_form_response_processed(
    db: &D1Database,
    form_resource_id: &str,
    response_id: &str,
) -> Result<bool> {
    let stmt = db.prepare(
        "SELECT COUNT(*) as count FROM form_response_tracking WHERE form_resource_id = ? AND response_id = ?",
    );
    let results = stmt
        .bind(&[form_resource_id.into(), response_id.into()])?
        .all()
        .await?;
    let rows = results.results::<serde_json::Value>()?;
    if let Some(row) = rows.into_iter().next() {
        Ok(row.get("count").and_then(|v| v.as_i64()).unwrap_or(0) > 0)
    } else {
        Ok(false)
    }
}

pub async fn mark_form_response_processed(
    db: &D1Database,
    id: &str,
    form_resource_id: &str,
    response_id: &str,
    tenant_id: &str,
) -> Result<()> {
    let stmt = db.prepare(
        "INSERT OR IGNORE INTO form_response_tracking (id, form_resource_id, response_id, tenant_id, processed_at)
         VALUES (?, ?, ?, ?, datetime('now'))",
    );
    stmt.bind(&[
        id.into(),
        form_resource_id.into(),
        response_id.into(),
        tenant_id.into(),
    ])?
    .run()
    .await?;
    Ok(())
}

// ============================================================================
// D1 Operations (Instagram Posts)
// ============================================================================

pub async fn get_instagram_post_by_post_id(
    db: &D1Database,
    instagram_post_id: &str,
) -> Result<Option<ProcessedPost>> {
    let stmt = db.prepare(
        "SELECT id, calendar_id, form_slug, instagram_source_id, instagram_post_id, instagram_permalink,
                caption_hash, event_id, contact_id, event_signature, processing_status, ai_response,
                processed_at, updated_at
         FROM instagram_posts WHERE instagram_post_id = ?",
    );

    let results = stmt.bind(&[instagram_post_id.into()])?.all().await?;
    let rows = results.results::<serde_json::Value>()?;

    if let Some(row) = rows.into_iter().next() {
        Ok(serde_json::from_value(row).ok())
    } else {
        Ok(None)
    }
}

pub async fn get_instagram_posts_by_signature(
    db: &D1Database,
    calendar_id: &str,
    event_signature: &str,
) -> Result<Vec<ProcessedPost>> {
    let stmt = db.prepare(
        "SELECT id, calendar_id, form_slug, instagram_source_id, instagram_post_id, instagram_permalink,
                caption_hash, event_id, contact_id, event_signature, processing_status, ai_response,
                processed_at, updated_at
         FROM instagram_posts
         WHERE calendar_id = ? AND event_signature = ?",
    );

    let results = stmt
        .bind(&[calendar_id.into(), event_signature.into()])?
        .all()
        .await?;

    let mut posts = Vec::new();
    for row in results.results::<serde_json::Value>()? {
        if let Ok(post) = serde_json::from_value::<ProcessedPost>(row) {
            posts.push(post);
        }
    }
    Ok(posts)
}

pub async fn save_instagram_post(
    db: &D1Database,
    post: &ProcessedPost,
    tenant_id: &str,
) -> Result<()> {
    let status_str = match post.processing_status {
        ProcessingStatus::Pending => "pending",
        ProcessingStatus::Processed => "processed",
        ProcessingStatus::NoEvent => "no_event",
        ProcessingStatus::Failed => "failed",
    };

    let stmt = db.prepare(
        "INSERT OR REPLACE INTO instagram_posts
         (id, calendar_id, form_slug, instagram_source_id, instagram_post_id, instagram_permalink,
          caption_hash, event_id, contact_id, event_signature, processing_status, ai_response,
          tenant_id, processed_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    );

    stmt.bind(&[
        post.id.clone().into(),
        post.calendar_id.clone().unwrap_or_default().into(),
        post.form_slug.clone().unwrap_or_default().into(),
        post.instagram_source_id.clone().into(),
        post.instagram_post_id.clone().into(),
        post.instagram_permalink.clone().into(),
        post.caption_hash.clone().into(),
        post.event_id.clone().unwrap_or_default().into(),
        post.contact_id.map(|id| id.into()).unwrap_or(JsValue::NULL),
        post.event_signature.clone().unwrap_or_default().into(),
        status_str.into(),
        post.ai_response.clone().unwrap_or_default().into(),
        tenant_id.into(),
        post.processed_at.clone().into(),
        post.updated_at.clone().unwrap_or_default().into(),
    ])?
    .run()
    .await?;

    Ok(())
}
