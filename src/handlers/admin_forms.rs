//! Admin handlers for Google Form resources

use worker::*;

use crate::helpers::*;
use crate::storage::*;
use crate::templates::*;
use crate::types::*;

/// Handle /admin/forms routes
pub async fn handle_forms_admin(
    mut req: Request,
    env: Env,
    path: &str,
    _method: Method,
    base_url: &str,
    tenant_id: &str,
) -> Result<Response> {
    let kv = env.kv("CALENDARS_KV")?;

    let path_parts: Vec<&str> = path
        .strip_prefix("/admin/forms")
        .unwrap_or("")
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let method = req.method();

    match (method, path_parts.as_slice()) {
        // List all form resources
        (Method::Get, []) => {
            let forms = list_form_resources(&kv, tenant_id).await?;
            Response::from_html(admin_forms_list_html(&forms, base_url))
        }

        // Create new form resource
        (Method::Post, []) => {
            let now = now_iso();
            let form = GoogleFormResource {
                id: generate_id(),
                tenant_id: tenant_id.to_string(),
                name: String::from("New Form"),
                slug: generate_slug(),
                google_form_url: String::new(),
                enabled: true,
                whatsapp_account_id: None,
                phone_field: String::new(),
                reply_prompt: String::new(),
                use_ai: false,
                last_polled_at: None,
                created_at: now.clone(),
                updated_at: now,
            };
            save_form_resource(&kv, &form).await?;

            let headers = Headers::new();
            headers.set("Location", &format!("{}/admin/forms/{}", base_url, form.id))?;
            Ok(Response::empty()?.with_status(302).with_headers(headers))
        }

        // Edit page for a form resource
        (Method::Get, [id]) => {
            let form = match get_form_resource(&kv, id).await? {
                Some(f) => f,
                None => return Response::error("Form not found", 404),
            };
            if form.tenant_id != tenant_id {
                return Response::error("Not found", 404);
            }
            let whatsapp_accounts = list_whatsapp_accounts(&kv, tenant_id).await?;
            Response::from_html(admin_form_edit_html(&form, &whatsapp_accounts, base_url))
        }

        // Update form resource
        (Method::Put, [id]) => {
            let mut form = match get_form_resource(&kv, id).await? {
                Some(f) => f,
                None => return Response::error("Form not found", 404),
            };
            if form.tenant_id != tenant_id {
                return Response::error("Not found", 404);
            }

            let data = req.form_data().await?;
            if let Some(FormEntry::Field(name)) = data.get("name") {
                form.name = name;
            }
            if let Some(FormEntry::Field(url)) = data.get("google_form_url") {
                form.google_form_url = url;
            }
            form.enabled = data.get("enabled").is_some();
            if let Some(FormEntry::Field(wa_id)) = data.get("whatsapp_account_id") {
                form.whatsapp_account_id = if wa_id.is_empty() { None } else { Some(wa_id) };
            }
            if let Some(FormEntry::Field(pf)) = data.get("phone_field") {
                form.phone_field = pf;
            }
            if let Some(FormEntry::Field(prompt)) = data.get("reply_prompt") {
                form.reply_prompt = prompt;
            }
            form.use_ai = data.get("use_ai").is_some();

            form.updated_at = now_iso();
            save_form_resource(&kv, &form).await?;
            Response::from_html(calendar_success_html("Form updated"))
        }

        // Delete form resource
        (Method::Delete, [id]) => {
            let form = match get_form_resource(&kv, id).await? {
                Some(f) => f,
                None => return Response::error("Form not found", 404),
            };
            if form.tenant_id != tenant_id {
                return Response::error("Not found", 404);
            }
            delete_form_resource(&kv, tenant_id, id).await?;
            Response::empty()
        }

        // Form responses viewer
        (Method::Get, [id, "responses"]) => {
            let form = match get_form_resource(&kv, id).await? {
                Some(f) => f,
                None => return Response::error("Form not found", 404),
            };
            if form.tenant_id != tenant_id {
                return Response::error("Not found", 404);
            }

            let form_id = crate::google_forms::parse_form_id(&form.google_form_url);
            if form_id.is_empty() {
                return Response::from_html(calendar_error_html("No Google Form URL configured"));
            }

            let encryption_key = env
                .secret("ENCRYPTION_KEY")
                .map(|s| s.to_string())
                .unwrap_or_default();
            let creds = get_tenant_credentials(&kv, tenant_id, &encryption_key)
                .await
                .unwrap_or_default();
            let (sa_email, sa_key) = match (
                &creds.google_service_account_email,
                &creds.google_private_key,
            ) {
                (Some(e), Some(k)) => (e.as_str(), k.as_str()),
                _ => {
                    return Response::from_html(calendar_error_html(
                        "Google service account not configured. Add credentials in Settings.",
                    ))
                }
            };

            let form_result = crate::google_forms::get_form(sa_email, sa_key, &form_id).await;
            let responses_result =
                crate::google_forms::get_responses(sa_email, sa_key, &form_id).await;

            match (form_result, responses_result) {
                (Ok(gform), Ok(responses)) => Response::from_html(
                    admin_form_resource_responses_html(&form, &gform, &responses, base_url),
                ),
                (Err(e), _) | (_, Err(e)) => Response::from_html(calendar_error_html(&format!(
                    "Failed to fetch form data: {}. Make sure the form is shared with your service account email.",
                    e
                ))),
            }
        }

        _ => Response::error("Not Found", 404),
    }
}
