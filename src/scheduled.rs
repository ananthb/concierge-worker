use worker::*;

use crate::ai;
use crate::crypto;
use crate::helpers::{generate_id, now_iso, today_date};
use crate::instagram::{self, InstagramClient};
use crate::storage::*;
use crate::templates::format_booking_digest;
use crate::types::*;

pub async fn handle_scheduled(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_log!("Scheduled job started");

    // Run booking digests
    if let Err(e) = send_booking_digests(&env).await {
        console_log!("Booking digest error: {:?}", e);
    }

    // Run Instagram sync for standalone accounts
    if let Err(e) = sync_instagram_accounts(&env).await {
        console_log!("Instagram account sync error: {:?}", e);
    }

    // Legacy: sync calendar-embedded instagram sources
    if let Err(e) = sync_calendar_instagram_sources(&env).await {
        console_log!("Calendar Instagram sync error: {:?}", e);
    }

    // Poll form responses for Form->WhatsApp automation
    if let Err(e) = poll_form_responses(&env).await {
        console_log!("Form response polling error: {:?}", e);
    }

    console_log!("Scheduled job completed");
}

// ============================================================================
// Booking Digests
// ============================================================================

async fn send_booking_digests(env: &Env) -> Result<()> {
    let kv = env.kv("CALENDARS_KV")?;
    let db = env.d1("DB")?;
    let calendars = list_calendars(&kv, "").await?;

    let now = now_iso();
    let now_date = js_sys::Date::new_0();
    let day_of_week = now_date.get_utc_day(); // 0 = Sunday

    let encryption_key = env
        .secret("ENCRYPTION_KEY")
        .map(|s| s.to_string())
        .unwrap_or_default();

    for mut calendar in calendars {
        if calendar.digest.frequency == DigestFrequency::None {
            continue;
        }
        if calendar.digest.responders.is_empty() {
            continue;
        }

        let should_send = match calendar.digest.frequency {
            DigestFrequency::Daily => true,
            DigestFrequency::Weekly => day_of_week == 1,
            DigestFrequency::None => false,
        };

        if !should_send {
            continue;
        }

        let since = calendar
            .digest
            .last_sent_at
            .as_deref()
            .unwrap_or("1970-01-01T00:00:00Z");

        let bookings = get_bookings_since(&db, &calendar.id, since).await?;
        if bookings.is_empty() {
            continue;
        }

        let body = format_booking_digest(&calendar, &bookings);

        // Resolve WhatsApp credentials: prefer per-digest account, fallback to tenant creds
        let (wa_token, wa_phone) = if let Some(ref wa_id) = calendar.digest.whatsapp_account_id {
            match get_whatsapp_credentials(&kv, wa_id, &encryption_key).await? {
                Some(c) => (c.access_token, c.phone_number_id),
                None => {
                    console_log!("WhatsApp account {} not found for digest", wa_id);
                    continue;
                }
            }
        } else {
            // Fallback to legacy tenant credentials
            let creds = get_tenant_credentials(&kv, &calendar.tenant_id, &encryption_key)
                .await
                .unwrap_or_default();
            match (creds.whatsapp_access_token, creds.whatsapp_phone_number_id) {
                (Some(t), Some(p)) => (t, p),
                _ => continue,
            }
        };

        for responder in &calendar.digest.responders {
            if !responder.enabled {
                continue;
            }
            let target = &responder.target_field;
            if target.is_empty() {
                continue;
            }
            if let Err(e) =
                crate::whatsapp::send_whatsapp_message(&wa_token, &wa_phone, target, &body).await
            {
                console_log!(
                    "Booking digest responder error for {}: {:?}",
                    responder.name,
                    e
                );
            }
        }

        calendar.digest.last_sent_at = Some(now.clone());
        if let Err(e) = save_calendar(&kv, &calendar).await {
            console_log!("Failed to update last_sent_at for {}: {:?}", calendar.id, e);
        }

        console_log!(
            "Sent booking digest for {} with {} bookings",
            calendar.name,
            bookings.len()
        );
    }

    Ok(())
}

// ============================================================================
// Instagram Sync for Standalone Accounts (New Resource Model)
// ============================================================================

async fn sync_instagram_accounts(env: &Env) -> Result<()> {
    let kv = env.kv("CALENDARS_KV")?;
    let db = env.d1("DB")?;

    let encryption_key = env
        .secret("ENCRYPTION_KEY")
        .map(|s| s.to_string())
        .unwrap_or_default();

    if encryption_key.is_empty() {
        return Ok(());
    }

    // Iterate all calendars to find their tenants, then load accounts per tenant
    let calendars = list_calendars(&kv, "").await?;
    let mut seen_tenants = std::collections::HashSet::new();

    for calendar in &calendars {
        if calendar.tenant_id.is_empty() || !seen_tenants.insert(calendar.tenant_id.clone()) {
            continue;
        }

        let accounts = list_instagram_accounts(&kv, &calendar.tenant_id).await?;
        for mut account in accounts {
            if !account.enabled {
                continue;
            }

            let target_calendar_id = match &account.target_calendar_id {
                Some(id) => id.clone(),
                None => continue, // No target calendar configured
            };

            let target_calendar = match get_calendar(&kv, &target_calendar_id).await? {
                Some(c) => c,
                None => continue,
            };

            console_log!(
                "Syncing Instagram account @{} -> calendar {}",
                account.instagram_username,
                target_calendar.name
            );

            // Get token using the account-based key
            let token =
                match get_instagram_token_for_account(&kv, &account.id, &encryption_key).await {
                    Ok(Some(t)) => t,
                    Ok(None) => continue,
                    Err(e) => {
                        console_log!("Token error for Instagram account {}: {:?}", account.id, e);
                        continue;
                    }
                };

            let client = InstagramClient::new(token.access_token);
            let media = match client.get_recent_media().await {
                Ok(m) => m,
                Err(e) => {
                    console_log!("Failed to fetch media for {}: {:?}", account.id, e);
                    continue;
                }
            };

            let today = today_date();
            let gcal_creds = get_tenant_credentials(&kv, &account.tenant_id, &encryption_key)
                .await
                .unwrap_or_default();

            for post in media {
                let caption = match &post.caption {
                    Some(c) if !c.trim().is_empty() => c.clone(),
                    _ => continue,
                };

                if let Err(e) = process_calendar_instagram_post(
                    env,
                    &db,
                    &target_calendar_id,
                    target_calendar.google_calendar_id.as_deref(),
                    &account.id,
                    &account.instagram_username,
                    &target_calendar.timezone,
                    &today,
                    &post.id,
                    &post.permalink,
                    &caption,
                    &account.tenant_id,
                    gcal_creds.google_service_account_email.as_deref(),
                    gcal_creds.google_private_key.as_deref(),
                )
                .await
                {
                    console_log!("Failed to process post {}: {:?}", post.id, e);
                }
            }

            account.last_synced_at = Some(now_iso());
            save_instagram_account(&kv, &account).await?;
        }
    }

    Ok(())
}

// ============================================================================
// Instagram Sync for Calendars (Legacy - calendar.instagram_sources)
// ============================================================================

async fn sync_calendar_instagram_sources(env: &Env) -> Result<()> {
    let kv = env.kv("CALENDARS_KV")?;
    let db = env.d1("DB")?;

    let encryption_key = env
        .secret("ENCRYPTION_KEY")
        .map(|s| s.to_string())
        .unwrap_or_default();

    if encryption_key.is_empty() {
        return Ok(());
    }

    let calendars = list_calendars(&kv, "").await?;

    for mut calendar in calendars {
        if calendar.instagram_sources.is_empty() {
            continue;
        }

        let calendar_id = calendar.id.clone();
        let timezone = calendar.timezone.clone();
        let mut synced_source_ids: Vec<String> = Vec::new();

        for source in &calendar.instagram_sources {
            if !source.enabled {
                continue;
            }

            let source_id = source.id.clone();
            console_log!(
                "Syncing Instagram source {} for calendar {}",
                source.instagram_username,
                calendar_id
            );

            let token = match get_instagram_token(
                env,
                &kv,
                &calendar_id,
                &source_id,
                &encryption_key,
            )
            .await
            {
                Ok(Some(t)) => t,
                Ok(None) => continue,
                Err(e) => {
                    console_log!("Token error for {}: {:?}", source_id, e);
                    continue;
                }
            };

            let client = InstagramClient::new(token.access_token);
            let media = match client.get_recent_media().await {
                Ok(m) => m,
                Err(e) => {
                    console_log!("Failed to fetch media for {}: {:?}", source_id, e);
                    continue;
                }
            };

            let today = today_date();
            let gcal_creds = get_tenant_credentials(&kv, &calendar.tenant_id, &encryption_key)
                .await
                .unwrap_or_default();

            for post in media {
                let caption = match &post.caption {
                    Some(c) if !c.trim().is_empty() => c.clone(),
                    _ => continue,
                };

                if let Err(e) = process_calendar_instagram_post(
                    env,
                    &db,
                    &calendar_id,
                    calendar.google_calendar_id.as_deref(),
                    &source_id,
                    &source.instagram_username,
                    &timezone,
                    &today,
                    &post.id,
                    &post.permalink,
                    &caption,
                    &calendar.tenant_id,
                    gcal_creds.google_service_account_email.as_deref(),
                    gcal_creds.google_private_key.as_deref(),
                )
                .await
                {
                    console_log!("Failed to process post {}: {:?}", post.id, e);
                }
            }

            synced_source_ids.push(source_id);
        }

        for source_id in synced_source_ids {
            if let Some(source_mut) = calendar
                .instagram_sources
                .iter_mut()
                .find(|s| s.id == source_id)
            {
                source_mut.last_synced_at = Some(now_iso());
            }
        }

        calendar.updated_at = now_iso();
        save_calendar(&kv, &calendar).await?;
    }

    Ok(())
}

// ============================================================================
// Form Response Polling (Form -> WhatsApp automation)
// ============================================================================

async fn poll_form_responses(env: &Env) -> Result<()> {
    let kv = env.kv("CALENDARS_KV")?;
    let db = env.d1("DB")?;

    let encryption_key = env
        .secret("ENCRYPTION_KEY")
        .map(|s| s.to_string())
        .unwrap_or_default();

    // Iterate tenants via calendars (same pattern as above)
    let calendars = list_calendars(&kv, "").await?;
    let mut seen_tenants = std::collections::HashSet::new();

    for calendar in &calendars {
        if calendar.tenant_id.is_empty() || !seen_tenants.insert(calendar.tenant_id.clone()) {
            continue;
        }

        let forms = list_form_resources(&kv, &calendar.tenant_id).await?;
        for mut form in forms {
            if !form.enabled || form.whatsapp_account_id.is_none() || form.phone_field.is_empty() {
                continue;
            }

            let form_id = crate::google_forms::parse_form_id(&form.google_form_url);
            if form_id.is_empty() {
                continue;
            }

            // Get Google SA credentials
            let creds = get_tenant_credentials(&kv, &form.tenant_id, &encryption_key)
                .await
                .unwrap_or_default();
            let (sa_email, sa_key) = match (
                &creds.google_service_account_email,
                &creds.google_private_key,
            ) {
                (Some(e), Some(k)) => (e.as_str(), k.as_str()),
                _ => continue,
            };

            // Get WhatsApp credentials
            let wa_account_id = form.whatsapp_account_id.as_deref().unwrap();
            let wa_creds =
                match get_whatsapp_credentials(&kv, wa_account_id, &encryption_key).await? {
                    Some(c) => c,
                    None => continue,
                };

            // Fetch form responses
            let responses =
                match crate::google_forms::get_responses(sa_email, sa_key, &form_id).await {
                    Ok(r) => r,
                    Err(e) => {
                        console_log!("Failed to fetch form responses for {}: {:?}", form.id, e);
                        continue;
                    }
                };

            // Get form structure to map question IDs to titles
            let gform = match crate::google_forms::get_form(sa_email, sa_key, &form_id).await {
                Ok(f) => f,
                Err(e) => {
                    console_log!("Failed to fetch form structure for {}: {:?}", form.id, e);
                    continue;
                }
            };

            // Build question ID -> title mapping
            let question_map = build_question_map(&gform);

            // Process each response
            for response in &responses {
                let response_id = &response.response_id;

                // Skip already processed
                if is_form_response_processed(&db, &form.id, response_id).await? {
                    continue;
                }

                // Extract phone number from the configured phone_field
                let phone_number =
                    extract_field_value_from_response(response, &question_map, &form.phone_field);

                if let Some(phone) = phone_number {
                    // Build context from all answers
                    let context = build_response_context_from_response(response, &question_map);

                    let message = if form.use_ai && !form.reply_prompt.is_empty() {
                        match ai::generate_response(env, &form.reply_prompt, &context).await {
                            Ok(r) => r,
                            Err(e) => {
                                console_log!("AI error for form {}: {:?}", form.id, e);
                                if form.reply_prompt.is_empty() {
                                    "Thank you for your submission!".to_string()
                                } else {
                                    crate::helpers::interpolate_template(
                                        &form.reply_prompt,
                                        &context,
                                    )
                                }
                            }
                        }
                    } else if !form.reply_prompt.is_empty() {
                        crate::helpers::interpolate_template(&form.reply_prompt, &context)
                    } else {
                        "Thank you for your submission!".to_string()
                    };

                    if let Err(e) = crate::whatsapp::send_whatsapp_message(
                        &wa_creds.access_token,
                        &wa_creds.phone_number_id,
                        &phone,
                        &message,
                    )
                    .await
                    {
                        console_log!(
                            "Failed to send WhatsApp for form response {}: {:?}",
                            response_id,
                            e
                        );
                    }
                }

                // Mark as processed
                mark_form_response_processed(
                    &db,
                    &generate_id(),
                    &form.id,
                    response_id,
                    &form.tenant_id,
                )
                .await?;
            }

            // Update last_polled_at
            form.last_polled_at = Some(now_iso());
            save_form_resource(&kv, &form).await?;
        }
    }

    Ok(())
}

/// Build a map from question ID to question title
fn build_question_map(
    form: &crate::google_forms::GoogleForm,
) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for item in &form.items {
        let title = item.title.as_deref().unwrap_or("");
        if let Some(qi) = &item.question_item {
            map.insert(qi.question.question_id.clone(), title.to_string());
        }
    }
    map
}

/// Extract a field value from a FormResponse by matching question title
fn extract_field_value_from_response(
    response: &crate::google_forms::FormResponse,
    question_map: &std::collections::HashMap<String, String>,
    field_title: &str,
) -> Option<String> {
    let field_title_lower = field_title.to_lowercase();

    for (qid, answer) in &response.answers {
        if let Some(title) = question_map.get(qid) {
            if title.to_lowercase().contains(&field_title_lower) {
                if let Some(ref ta) = answer.text_answers {
                    if let Some(first) = ta.answers.first() {
                        if !first.value.is_empty() {
                            return Some(first.value.clone());
                        }
                    }
                }
            }
        }
    }
    None
}

/// Build context map from all form response answers
fn build_response_context_from_response(
    response: &crate::google_forms::FormResponse,
    question_map: &std::collections::HashMap<String, String>,
) -> serde_json::Map<String, serde_json::Value> {
    let mut context = serde_json::Map::new();
    for (qid, answer) in &response.answers {
        let title = question_map
            .get(qid)
            .cloned()
            .unwrap_or_else(|| qid.clone());
        let value = answer
            .text_answers
            .as_ref()
            .and_then(|ta| ta.answers.first())
            .map(|a| a.value.as_str())
            .unwrap_or("");
        context.insert(title, serde_json::Value::String(value.to_string()));
    }
    context
}

// ============================================================================
// Instagram Post Processing (shared between legacy and new model)
// ============================================================================

#[allow(clippy::too_many_arguments)]
async fn process_calendar_instagram_post(
    env: &Env,
    db: &D1Database,
    calendar_id: &str,
    google_calendar_id: Option<&str>,
    source_id: &str,
    username: &str,
    timezone: &str,
    today: &str,
    post_id: &str,
    permalink: &str,
    caption: &str,
    tenant_id: &str,
    google_sa_email: Option<&str>,
    google_sa_key: Option<&str>,
) -> Result<()> {
    let caption_hash = crypto::sha256_hex(caption);

    let existing = get_instagram_post_by_post_id(db, post_id).await?;
    if let Some(ref existing_post) = existing {
        if existing_post.caption_hash == caption_hash {
            return Ok(());
        }
        console_log!("Caption changed for post {}, re-processing", post_id);
    }

    let (extracted, ai_response) =
        match ai::extract_event_from_caption(env, caption, timezone, today).await {
            Ok(result) => result,
            Err(e) => {
                let failed_post = ProcessedPost {
                    id: existing
                        .as_ref()
                        .map(|p| p.id.clone())
                        .unwrap_or_else(generate_id),
                    calendar_id: Some(calendar_id.to_string()),
                    form_slug: None,
                    instagram_source_id: source_id.to_string(),
                    instagram_post_id: post_id.to_string(),
                    instagram_permalink: permalink.to_string(),
                    caption_hash,
                    event_id: None,
                    contact_id: None,
                    event_signature: None,
                    processing_status: ProcessingStatus::Failed,
                    ai_response: Some(format!("Error: {:?}", e)),
                    processed_at: now_iso(),
                    updated_at: Some(now_iso()),
                };
                save_instagram_post(db, &failed_post, tenant_id).await?;
                return Err(e);
            }
        };

    if !ai::event_is_valid(&extracted) {
        let no_event_post = ProcessedPost {
            id: existing
                .as_ref()
                .map(|p| p.id.clone())
                .unwrap_or_else(generate_id),
            calendar_id: Some(calendar_id.to_string()),
            form_slug: None,
            instagram_source_id: source_id.to_string(),
            instagram_post_id: post_id.to_string(),
            instagram_permalink: permalink.to_string(),
            caption_hash,
            event_id: None,
            contact_id: None,
            event_signature: None,
            processing_status: ProcessingStatus::NoEvent,
            ai_response: Some(ai_response),
            processed_at: now_iso(),
            updated_at: Some(now_iso()),
        };
        save_instagram_post(db, &no_event_post, tenant_id).await?;
        return Ok(());
    }

    let event_signature = ai::generate_event_signature(&extracted);

    if extracted.is_cancellation {
        if let Some(ref sig) = event_signature {
            let existing_posts = get_instagram_posts_by_signature(db, calendar_id, sig).await?;
            for ep in existing_posts {
                if let Some(ref event_id) = ep.event_id {
                    console_log!(
                        "Cancellation detected for event {} from Instagram post",
                        event_id
                    );
                }
            }
        }

        let cancelled_post = ProcessedPost {
            id: existing
                .as_ref()
                .map(|p| p.id.clone())
                .unwrap_or_else(generate_id),
            calendar_id: Some(calendar_id.to_string()),
            form_slug: None,
            instagram_source_id: source_id.to_string(),
            instagram_post_id: post_id.to_string(),
            instagram_permalink: permalink.to_string(),
            caption_hash,
            event_id: None,
            contact_id: None,
            event_signature,
            processing_status: ProcessingStatus::Processed,
            ai_response: Some(ai_response),
            processed_at: now_iso(),
            updated_at: Some(now_iso()),
        };
        save_instagram_post(db, &cancelled_post, tenant_id).await?;
        return Ok(());
    }

    // Check for duplicate
    let mut is_duplicate = false;
    if let Some(ref sig) = event_signature {
        let existing_posts = get_instagram_posts_by_signature(db, calendar_id, sig).await?;
        for ep in &existing_posts {
            if ep.instagram_post_id != post_id && ep.event_id.is_some() {
                is_duplicate = true;
                break;
            }
        }
    }

    if is_duplicate {
        let dup_post = ProcessedPost {
            id: existing
                .as_ref()
                .map(|p| p.id.clone())
                .unwrap_or_else(generate_id),
            calendar_id: Some(calendar_id.to_string()),
            form_slug: None,
            instagram_source_id: source_id.to_string(),
            instagram_post_id: post_id.to_string(),
            instagram_permalink: permalink.to_string(),
            caption_hash,
            event_id: None,
            contact_id: None,
            event_signature,
            processing_status: ProcessingStatus::Processed,
            ai_response: Some(ai_response),
            processed_at: now_iso(),
            updated_at: Some(now_iso()),
        };
        save_instagram_post(db, &dup_post, tenant_id).await?;
        return Ok(());
    }

    // Create event in Google Calendar
    let date = extracted.date.as_deref().unwrap_or(today);
    let start_time = extracted.start_time.as_deref().unwrap_or("00:00");
    let end_time = extracted.end_time.as_deref().unwrap_or("23:59");

    let start_datetime = format!("{}T{}:00", date, start_time);
    let end_datetime = format!("{}T{}:00", date, end_time);

    let title = extracted
        .title
        .unwrap_or_else(|| "Instagram Event".to_string());
    let description = format!(
        "{}\n\nSource: Instagram (@{})\n{}",
        extracted.description.as_deref().unwrap_or(""),
        username,
        permalink
    );

    let mut event_id = existing.as_ref().and_then(|p| p.event_id.clone());

    if let Some(gcal_id) = google_calendar_id {
        match (google_sa_email, google_sa_key) {
            (Some(email), Some(key)) => {
                match crate::google_calendar::create_event(
                    email,
                    key,
                    gcal_id,
                    &title,
                    Some(&description),
                    &start_datetime,
                    &end_datetime,
                    timezone,
                )
                .await
                {
                    Ok(gcal_event_id) => {
                        event_id = Some(gcal_event_id.clone());
                        console_log!(
                            "Created Google Calendar event {} from post {}",
                            gcal_event_id,
                            post_id
                        );
                    }
                    Err(e) => {
                        console_log!(
                            "Failed to create Google Calendar event from post {}: {:?}",
                            post_id,
                            e
                        );
                    }
                }
            }
            _ => {
                console_log!("No Google credentials for calendar {}", calendar_id);
            }
        }
    }

    let processed_post = ProcessedPost {
        id: existing
            .as_ref()
            .map(|p| p.id.clone())
            .unwrap_or_else(generate_id),
        calendar_id: Some(calendar_id.to_string()),
        form_slug: None,
        instagram_source_id: source_id.to_string(),
        instagram_post_id: post_id.to_string(),
        instagram_permalink: permalink.to_string(),
        caption_hash,
        event_id,
        contact_id: None,
        event_signature,
        processing_status: ProcessingStatus::Processed,
        ai_response: Some(ai_response),
        processed_at: now_iso(),
        updated_at: Some(now_iso()),
    };
    save_instagram_post(db, &processed_post, tenant_id).await?;

    Ok(())
}

// ============================================================================
// Helpers
// ============================================================================

/// Get Instagram token for a standalone InstagramAccount resource
async fn get_instagram_token_for_account(
    kv: &kv::KvStore,
    account_id: &str,
    encryption_key: &str,
) -> Result<Option<InstagramToken>> {
    let token_key = format!("instagram_token:{}", account_id);
    let encrypted_token = match kv.get(&token_key).text().await? {
        Some(t) => t,
        None => return Ok(None),
    };

    let mut token = match crypto::decrypt_token(&encrypted_token, encryption_key).await {
        Ok(t) => t,
        Err(e) => {
            console_log!(
                "Failed to decrypt token for account {}: {:?}",
                account_id,
                e
            );
            return Ok(None);
        }
    };

    if instagram::token_is_expired(&token) {
        return Ok(None);
    }

    if instagram::token_needs_refresh(&token) {
        match instagram::refresh_token(&token.access_token).await {
            Ok(new_token) => {
                let encrypted = crypto::encrypt_token(&new_token, encryption_key).await?;
                kv.put(&token_key, encrypted)?.execute().await?;
                token = new_token;
            }
            Err(e) => {
                console_log!(
                    "Failed to refresh token for account {}: {:?}",
                    account_id,
                    e
                );
                return Ok(None);
            }
        }
    }

    Ok(Some(token))
}

/// Get Instagram token for legacy calendar-based sources
async fn get_instagram_token(
    _env: &Env,
    kv: &kv::KvStore,
    owner_id: &str,
    source_id: &str,
    encryption_key: &str,
) -> Result<Option<InstagramToken>> {
    let token_key = format!("instagram_token:{}:{}", owner_id, source_id);
    let encrypted_token = match kv.get(&token_key).text().await? {
        Some(t) => t,
        None => {
            console_log!("No token found for source {}", source_id);
            return Ok(None);
        }
    };

    let mut token = match crypto::decrypt_token(&encrypted_token, encryption_key).await {
        Ok(t) => t,
        Err(e) => {
            console_log!("Failed to decrypt token for {}: {:?}", source_id, e);
            return Ok(None);
        }
    };

    if instagram::token_is_expired(&token) {
        console_log!("Token expired for source {}", source_id);
        return Ok(None);
    }

    if instagram::token_needs_refresh(&token) {
        match instagram::refresh_token(&token.access_token).await {
            Ok(new_token) => {
                let encrypted = crypto::encrypt_token(&new_token, encryption_key).await?;
                kv.put(&token_key, encrypted)?.execute().await?;
                token = new_token;
                console_log!("Refreshed token for source {}", source_id);
            }
            Err(e) => {
                console_log!("Failed to refresh token for {}: {:?}", source_id, e);
                return Ok(None);
            }
        }
    }

    Ok(Some(token))
}
