//! Admin handlers for Instagram account resources

use worker::*;

use crate::storage::*;
use crate::templates::*;

/// Handle /admin/instagram routes
pub async fn handle_instagram_admin(
    mut req: Request,
    env: Env,
    path: &str,
    _method: Method,
    base_url: &str,
    tenant_id: &str,
) -> Result<Response> {
    let kv = env.kv("CALENDARS_KV")?;

    let path_parts: Vec<&str> = path
        .strip_prefix("/admin/instagram")
        .unwrap_or("")
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let method = req.method();

    match (method, path_parts.as_slice()) {
        // List all Instagram accounts
        (Method::Get, []) => {
            let accounts = list_instagram_accounts(&kv, tenant_id).await?;
            let calendars = list_calendars(&kv, tenant_id).await?;
            Response::from_html(admin_instagram_list_html(&accounts, &calendars, base_url))
        }

        // Edit page for an Instagram account
        (Method::Get, [id]) => {
            let account = match get_instagram_account(&kv, id).await? {
                Some(a) => a,
                None => return Response::error("Instagram account not found", 404),
            };
            if account.tenant_id != tenant_id {
                return Response::error("Not found", 404);
            }
            let calendars = list_calendars(&kv, tenant_id).await?;
            Response::from_html(admin_instagram_edit_html(&account, &calendars, base_url))
        }

        // Update Instagram account
        (Method::Put, [id]) => {
            let mut account = match get_instagram_account(&kv, id).await? {
                Some(a) => a,
                None => return Response::error("Instagram account not found", 404),
            };
            if account.tenant_id != tenant_id {
                return Response::error("Not found", 404);
            }

            let form = req.form_data().await?;
            if let Some(FormEntry::Field(cal_id)) = form.get("target_calendar_id") {
                account.target_calendar_id = if cal_id.is_empty() {
                    None
                } else {
                    Some(cal_id)
                };
            }
            if let Some(FormEntry::Field(prompt)) = form.get("classification_prompt") {
                account.classification_prompt = if prompt.is_empty() {
                    None
                } else {
                    Some(prompt)
                };
            }
            account.enabled = form.get("enabled").is_some();

            save_instagram_account(&kv, &account).await?;
            Response::from_html(calendar_success_html("Instagram account updated"))
        }

        // Disconnect/delete Instagram account
        (Method::Delete, [id]) => {
            let account = match get_instagram_account(&kv, id).await? {
                Some(a) => a,
                None => return Response::error("Instagram account not found", 404),
            };
            if account.tenant_id != tenant_id {
                return Response::error("Not found", 404);
            }
            delete_instagram_account(&kv, tenant_id, id).await?;
            Response::empty()
        }

        _ => Response::error("Not Found", 404),
    }
}
