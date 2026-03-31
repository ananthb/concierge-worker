use worker::*;

use crate::types::{InstagramAccount, LeadCaptureForm, Tenant, WhatsAppAccount};

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
    kv.put(
        &format!("tenant_email:{}", tenant.email.to_lowercase()),
        &tenant.id,
    )?
    .execute()
    .await?;
    Ok(())
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
    // Reverse index: phone_number_id -> whatsapp account id (for webhook routing)
    if !account.phone_number_id.is_empty() {
        kv.put(
            &format!("wa_phone:{}", account.phone_number_id),
            &account.id,
        )?
        .execute()
        .await?;
    }
    Ok(())
}

pub async fn delete_whatsapp_account(kv: &kv::KvStore, tenant_id: &str, id: &str) -> Result<()> {
    // Load account first to clean up phone index
    if let Some(account) = get_whatsapp_account(kv, id).await? {
        if !account.phone_number_id.is_empty() {
            kv.delete(&format!("wa_phone:{}", account.phone_number_id))
                .await?;
        }
    }
    kv.delete(&format!("whatsapp:{}", id)).await?;
    if !tenant_id.is_empty() {
        kv.delete(&format!("tenant:{}:whatsapp:{}", tenant_id, id))
            .await?;
    }
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
// Instagram Account KV Operations
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
    // Reverse index: page_id -> instagram account id (for webhook routing)
    if !account.page_id.is_empty() {
        kv.put(&format!("ig_page:{}", account.page_id), &account.id)?
            .execute()
            .await?;
    }
    Ok(())
}

pub async fn delete_instagram_account(kv: &kv::KvStore, tenant_id: &str, id: &str) -> Result<()> {
    if let Some(account) = get_instagram_account(kv, id).await? {
        if !account.page_id.is_empty() {
            kv.delete(&format!("ig_page:{}", account.page_id)).await?;
        }
    }
    kv.delete(&format!("instagram:{}", id)).await?;
    if !tenant_id.is_empty() {
        kv.delete(&format!("tenant:{}:instagram:{}", tenant_id, id))
            .await?;
    }
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

pub async fn get_instagram_account_by_page(
    kv: &kv::KvStore,
    page_id: &str,
) -> Result<Option<String>> {
    kv.get(&format!("ig_page:{}", page_id))
        .text()
        .await
        .map_err(|e| Error::from(e.to_string()))
}

// ============================================================================
// Lead Form KV Operations
// ============================================================================

pub async fn get_lead_form(kv: &kv::KvStore, id: &str) -> Result<Option<LeadCaptureForm>> {
    kv.get(&format!("lead_form:{}", id))
        .json::<LeadCaptureForm>()
        .await
        .map_err(|e| Error::from(e.to_string()))
}

pub async fn save_lead_form(kv: &kv::KvStore, form: &LeadCaptureForm) -> Result<()> {
    kv.put(&format!("lead_form:{}", form.id), form)?
        .execute()
        .await?;
    if !form.tenant_id.is_empty() {
        kv.put(
            &format!("tenant:{}:lead_form:{}", form.tenant_id, form.id),
            "",
        )?
        .execute()
        .await?;
    }
    Ok(())
}

pub async fn delete_lead_form(kv: &kv::KvStore, tenant_id: &str, id: &str) -> Result<()> {
    kv.delete(&format!("lead_form:{}", id)).await?;
    if !tenant_id.is_empty() {
        kv.delete(&format!("tenant:{}:lead_form:{}", tenant_id, id))
            .await?;
    }
    Ok(())
}

pub async fn list_lead_forms(kv: &kv::KvStore, tenant_id: &str) -> Result<Vec<LeadCaptureForm>> {
    let prefix = format!("tenant:{}:lead_form:", tenant_id);
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
        if let Some(form) = get_lead_form(kv, &form_id).await? {
            forms.push(form);
        }
    }
    forms.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(forms)
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
// D1 Operations (Lead Form Submissions)
// ============================================================================

#[allow(clippy::too_many_arguments)]
pub async fn save_lead_form_submission(
    db: &D1Database,
    id: &str,
    lead_form_id: &str,
    phone_number: &str,
    whatsapp_account_id: &str,
    message_sent: &str,
    reply_mode: &str,
    tenant_id: &str,
) -> Result<()> {
    let stmt = db.prepare(
        "INSERT INTO lead_form_submissions (id, lead_form_id, phone_number, whatsapp_account_id, message_sent, reply_mode, tenant_id, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))",
    );
    stmt.bind(&[
        id.into(),
        lead_form_id.into(),
        phone_number.into(),
        whatsapp_account_id.into(),
        message_sent.into(),
        reply_mode.into(),
        tenant_id.into(),
    ])?
    .run()
    .await?;
    Ok(())
}

// ============================================================================
// D1 Operations (Instagram Messages)
// ============================================================================

#[allow(clippy::too_many_arguments)]
pub async fn save_instagram_message(
    db: &D1Database,
    id: &str,
    instagram_account_id: &str,
    direction: &str,
    sender_id: &str,
    recipient_id: &str,
    body: &str,
    tenant_id: &str,
) -> Result<()> {
    let stmt = db.prepare(
        "INSERT INTO instagram_messages (id, instagram_account_id, direction, sender_id, recipient_id, body, tenant_id, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))",
    );
    stmt.bind(&[
        id.into(),
        instagram_account_id.into(),
        direction.into(),
        sender_id.into(),
        recipient_id.into(),
        body.into(),
        tenant_id.into(),
    ])?
    .run()
    .await?;
    Ok(())
}
