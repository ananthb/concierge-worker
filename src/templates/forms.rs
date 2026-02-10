//! Form editor, responses, and public form rendering templates

use crate::helpers::*;
use crate::types::*;

pub fn form_editor_html(form: Option<&FormConfig>, _admin_email: &str) -> String {
    let (title, form_json, is_new) = match form {
        Some(f) => (
            format!("Edit Form: {}", f.name),
            serde_json::to_string_pretty(f).unwrap_or_else(|_| "{}".into()),
            false,
        ),
        None => (
            "Create New Form".into(),
            serde_json::json!({
                "slug": "",
                "name": "New Form",
                "title": "Contact Us",
                "submit_button_text": "Submit",
                "success_message": "Thank you for your submission!",
                "allowed_origins": [],
                "fields": FormConfig::default_fields(),
                "style": FormStyle::default(),
                "responders": [],
                "digest": DigestConfig::default(),
                "google_sheet_url": null
            }).to_string(),
            true,
        ),
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <link rel="icon" type="image/svg+xml" href="/logo.svg">
    <style>
        * {{ box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; margin: 0; padding: 2rem; background: #f5f5f5; }}
        .container {{ max-width: 900px; margin: 0 auto; }}
        h1 {{ color: #333; }}
        .btn {{ display: inline-block; padding: 0.5rem 1rem; background: #0070f3; color: white; text-decoration: none; border: none; border-radius: 4px; cursor: pointer; font-size: 0.9rem; }}
        .btn:hover {{ background: #0060df; }}
        .btn-secondary {{ background: #6c757d; }}
        .card {{ background: white; padding: 1.5rem; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-bottom: 1rem; }}
        .form-group {{ margin-bottom: 1rem; }}
        label {{ display: block; margin-bottom: 0.5rem; font-weight: 500; }}
        input, textarea, select {{ width: 100%; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px; font-size: 1rem; }}
        textarea {{ min-height: 100px; }}
        .header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; }}
        #editor {{ display: none; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>{title}</h1>
            <div>
                <a href="/admin" class="btn btn-secondary">Back</a>
                <button onclick="saveForm()" class="btn">Save</button>
            </div>
        </div>
        <div class="card">
            <div class="form-group">
                <label>Form Slug (URL path)</label>
                <input type="text" id="slug" value="" pattern="[a-z0-9_-]+" {slug_disabled}>
            </div>
            <div class="form-group">
                <label>Form Name</label>
                <input type="text" id="name" value="">
            </div>
            <div class="form-group">
                <label>Form Title</label>
                <input type="text" id="title-input" value="">
            </div>
            <div class="form-group">
                <label>Submit Button Text</label>
                <input type="text" id="submit_button_text" value="">
            </div>
            <div class="form-group">
                <label>Success Message</label>
                <textarea id="success_message"></textarea>
            </div>
            <div class="form-group">
                <label>Allowed Origins (one per line, leave empty for any)</label>
                <textarea id="allowed_origins" placeholder="https://example.com"></textarea>
            </div>
            <div class="form-group">
                <label>Google Sheet URL (optional)</label>
                <input type="text" id="google_sheet_url" placeholder="https://docs.google.com/spreadsheets/d/...">
            </div>
        </div>
        <div class="card">
            <h3>Fields</h3>
            <p><small>Edit the JSON below to add/remove fields.</small></p>
            <textarea id="fields" style="font-family: monospace; min-height: 200px;"></textarea>
        </div>
        <textarea id="editor">{form_json}</textarea>
    </div>
    <script>
        const formData = JSON.parse(document.getElementById('editor').value);
        const isNew = {is_new};
        document.getElementById('slug').value = formData.slug || '';
        document.getElementById('name').value = formData.name || '';
        document.getElementById('title-input').value = formData.title || '';
        document.getElementById('submit_button_text').value = formData.submit_button_text || 'Submit';
        document.getElementById('success_message').value = formData.success_message || '';
        document.getElementById('allowed_origins').value = (formData.allowed_origins || []).join('\n');
        document.getElementById('google_sheet_url').value = formData.google_sheet_url || '';
        document.getElementById('fields').value = JSON.stringify(formData.fields || [], null, 2);

        async function saveForm() {{
            try {{
                const fields = JSON.parse(document.getElementById('fields').value);
                const data = {{
                    slug: document.getElementById('slug').value,
                    name: document.getElementById('name').value,
                    title: document.getElementById('title-input').value,
                    submit_button_text: document.getElementById('submit_button_text').value,
                    success_message: document.getElementById('success_message').value,
                    allowed_origins: document.getElementById('allowed_origins').value.split('\n').filter(s => s.trim()),
                    google_sheet_url: document.getElementById('google_sheet_url').value || null,
                    fields: fields,
                    style: formData.style || {{}},
                    responders: formData.responders || [],
                    digest: formData.digest || {{}}
                }};

                const url = isNew ? '/admin/forms' : '/admin/forms/' + formData.slug;
                const method = isNew ? 'POST' : 'PUT';

                const resp = await fetch(url, {{
                    method: method,
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify(data),
                    credentials: 'same-origin'
                }});

                if (resp.ok) {{
                    window.location.href = '/admin';
                }} else {{
                    alert('Failed to save: ' + await resp.text());
                }}
            }} catch (e) {{
                alert('Error: ' + e.message);
            }}
        }}
    </script>
</body>
</html>"##,
        title = html_escape(&title),
        form_json = html_escape(&form_json),
        is_new = is_new,
        slug_disabled = if is_new { "" } else { "disabled" },
    )
}

pub fn responses_view_html(form: &FormConfig, submissions: &[Submission]) -> String {
    let rows: String = if submissions.is_empty() {
        format!(
            r#"<tr><td colspan="{}" style="text-align:center;color:#666;padding:2rem;">No submissions yet.</td></tr>"#,
            form.fields.len() + 2
        )
    } else {
        submissions
            .iter()
            .map(|s| {
                let field_cells: String = form
                    .fields
                    .iter()
                    .map(|f| {
                        let value = s.fields_data
                            .get(&f.id)
                            .map(|v| match v {
                                serde_json::Value::String(s) => html_escape(s),
                                _ => html_escape(&v.to_string()),
                            })
                            .unwrap_or_else(|| "-".to_string());
                        format!("<td>{}</td>", value)
                    })
                    .collect();
                format!(
                    "<tr><td>{}</td>{}<td>{}</td></tr>",
                    s.id,
                    field_cells,
                    html_escape(&s.created_at)
                )
            })
            .collect()
    };

    let header_cells: String = form
        .fields
        .iter()
        .map(|f| format!("<th>{}</th>", html_escape(&f.label)))
        .collect();

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Responses - {form_name}</title>
    <link rel="icon" type="image/svg+xml" href="/logo.svg">
    <style>
        * {{ box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; margin: 0; padding: 2rem; background: #f5f5f5; }}
        .container {{ max-width: 1400px; margin: 0 auto; }}
        h1 {{ color: #333; margin: 0; }}
        .btn {{ display: inline-block; padding: 0.5rem 1rem; background: #0070f3; color: white; text-decoration: none; border: none; border-radius: 4px; cursor: pointer; font-size: 0.9rem; }}
        .btn:hover {{ background: #0060df; }}
        .btn-secondary {{ background: #6c757d; }}
        .header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; }}
        table {{ width: 100%; border-collapse: collapse; background: white; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        th, td {{ padding: 0.75rem 1rem; text-align: left; border-bottom: 1px solid #eee; }}
        th {{ background: #f8f9fa; font-weight: 600; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Responses for {form_name}</h1>
            <div>
                <a href="/admin/forms/{slug}" class="btn btn-secondary">Edit Form</a>
                <a href="/admin" class="btn btn-secondary">Dashboard</a>
            </div>
        </div>
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    {header_cells}
                    <th>Date</th>
                </tr>
            </thead>
            <tbody>
                {rows}
            </tbody>
        </table>
    </div>
</body>
</html>"##,
        form_name = html_escape(&form.name),
        slug = html_escape(&form.slug),
        header_cells = header_cells,
        rows = rows
    )
}

pub fn render_form(form: &FormConfig, inline_css: Option<&str>, css_url: Option<&str>, base_url: &str, is_htmx: bool) -> String {
    if is_htmx {
        render_form_fragment(form, base_url)
    } else {
        render_form_html(form, inline_css, css_url, base_url)
    }
}

fn render_form_html(form: &FormConfig, inline_css: Option<&str>, css_url: Option<&str>, base_url: &str) -> String {
    let s = &form.style;
    let has_file = form.fields.iter().any(|f| matches!(f.field_type, FieldType::File));

    let css_link = css_url
        .map(|url| format!(r#"<link rel="stylesheet" href="{}">"#, html_escape(url)))
        .unwrap_or_default();
    let query_css = inline_css.unwrap_or_default();

    let title_html = if s.show_title {
        format!("<h1>{}</h1>", html_escape(&form.title))
    } else {
        String::new()
    };

    let fields_html: String = form
        .fields
        .iter()
        .map(|f| {
            let input = match f.field_type {
                FieldType::Text => format!(
                    r#"<input type="text" id="{id}" name="{id}" {req} placeholder="{ph}">"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" },
                    ph = html_escape(f.placeholder.as_deref().unwrap_or(""))
                ),
                FieldType::Email => format!(
                    r#"<input type="email" id="{id}" name="{id}" {req} placeholder="{ph}">"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" },
                    ph = html_escape(f.placeholder.as_deref().unwrap_or(""))
                ),
                FieldType::Mobile | FieldType::Phone => format!(
                    r#"<input type="tel" id="{id}" name="{id}" {req} placeholder="{ph}">"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" },
                    ph = html_escape(f.placeholder.as_deref().unwrap_or(""))
                ),
                FieldType::LongText => format!(
                    r#"<textarea id="{id}" name="{id}" {req} placeholder="{ph}"></textarea>"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" },
                    ph = html_escape(f.placeholder.as_deref().unwrap_or(""))
                ),
                FieldType::File => format!(
                    r#"<input type="file" id="{id}" name="{id}" {req}>"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" }
                ),
            };
            format!(
                r#"<div class="form-group"><label for="{id}">{label}</label>{input}</div>"#,
                id = html_escape(&f.id),
                label = html_escape(&f.label),
                input = input
            )
        })
        .collect();

    let enctype = if has_file { r#" enctype="multipart/form-data""# } else { "" };

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    {css_link}
    <style>
        *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}
        :root {{
            --cf-font-family: {font_family};
            --cf-font-size: {font_size};
            --cf-text-color: {text_color};
            --cf-bg-color: {bg_color};
            --cf-form-bg: {form_bg};
            --cf-border-color: {border_color};
            --cf-border-radius: {border_radius};
            --cf-primary-color: {primary_color};
            --cf-input-padding: {input_padding};
        }}
        body {{
            font-family: var(--cf-font-family);
            font-size: var(--cf-font-size);
            color: var(--cf-text-color);
            max-width: 600px;
            margin: 0 auto;
            padding: 2rem;
            background: var(--cf-bg-color);
        }}
        h1 {{ margin-bottom: 1.5rem; }}
        .contact-form {{ background: var(--cf-form-bg); padding: 2rem; border-radius: var(--cf-border-radius); }}
        .form-group {{ margin-bottom: 1rem; }}
        label {{ display: block; margin-bottom: 0.5rem; font-weight: 500; }}
        input, textarea {{ width: 100%; padding: var(--cf-input-padding); border: 1px solid var(--cf-border-color); border-radius: var(--cf-border-radius); }}
        textarea {{ min-height: 120px; }}
        button {{ background: var(--cf-primary-color); color: white; padding: var(--cf-input-padding) 1.5rem; border: none; border-radius: var(--cf-border-radius); cursor: pointer; width: 100%; }}
        .success {{ background: #d4edda; color: #155724; padding: 2rem; border-radius: var(--cf-border-radius); text-align: center; }}
        .error {{ background: #f8d7da; color: #721c24; padding: 1rem; border-radius: var(--cf-border-radius); margin-bottom: 1rem; }}
        {custom_css}
        {query_css}
    </style>
</head>
<body>
    {title_html}
    <div id="form-container">
        <form class="contact-form" hx-post="{base_url}/f/{slug}/submit" hx-target="#form-container" hx-swap="innerHTML"{enctype}>
            {fields_html}
            <button type="submit">{submit_button_text}</button>
        </form>
    </div>
</body>
</html>"##,
        title = html_escape(&form.title),
        slug = html_escape(&form.slug),
        fields_html = fields_html,
        submit_button_text = html_escape(&form.submit_button_text),
        enctype = enctype,
        font_family = html_escape(&s.font_family),
        font_size = html_escape(&s.font_size),
        text_color = html_escape(&s.text_color),
        bg_color = html_escape(&s.bg_color),
        form_bg = html_escape(&s.form_bg),
        border_color = html_escape(&s.border_color),
        border_radius = html_escape(&s.border_radius),
        primary_color = html_escape(&s.primary_color),
        input_padding = html_escape(&s.input_padding),
        custom_css = s.custom_css,
        css_link = css_link,
        query_css = query_css,
        title_html = title_html,
        base_url = html_escape(base_url),
    )
}

fn render_form_fragment(form: &FormConfig, base_url: &str) -> String {
    let s = &form.style;
    let has_file = form.fields.iter().any(|f| matches!(f.field_type, FieldType::File));

    let fields_html: String = form
        .fields
        .iter()
        .map(|f| {
            let input = match f.field_type {
                FieldType::Text => format!(
                    r#"<input type="text" id="{id}" name="{id}" {req} placeholder="{ph}">"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" },
                    ph = html_escape(f.placeholder.as_deref().unwrap_or(""))
                ),
                FieldType::Email => format!(
                    r#"<input type="email" id="{id}" name="{id}" {req} placeholder="{ph}">"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" },
                    ph = html_escape(f.placeholder.as_deref().unwrap_or(""))
                ),
                FieldType::Mobile | FieldType::Phone => format!(
                    r#"<input type="tel" id="{id}" name="{id}" {req} placeholder="{ph}">"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" },
                    ph = html_escape(f.placeholder.as_deref().unwrap_or(""))
                ),
                FieldType::LongText => format!(
                    r#"<textarea id="{id}" name="{id}" {req} placeholder="{ph}"></textarea>"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" },
                    ph = html_escape(f.placeholder.as_deref().unwrap_or(""))
                ),
                FieldType::File => format!(
                    r#"<input type="file" id="{id}" name="{id}" {req}>"#,
                    id = html_escape(&f.id),
                    req = if f.required { "required" } else { "" }
                ),
            };
            format!(
                r#"<div class="form-group"><label for="{id}">{label}</label>{input}</div>"#,
                id = html_escape(&f.id),
                label = html_escape(&f.label),
                input = input
            )
        })
        .collect();

    let enctype = if has_file { r#" enctype="multipart/form-data""# } else { "" };

    let title_html = if s.show_title {
        format!("<h1>{}</h1>", html_escape(&form.title))
    } else {
        String::new()
    };

    format!(
        r##"{title_html}
    <div id="form-container">
        <form class="contact-form" hx-post="{base_url}/f/{slug}/submit" hx-target="#form-container" hx-swap="innerHTML"{enctype}>
            {fields_html}
            <button type="submit">{submit_button_text}</button>
        </form>
    </div>"##,
        title_html = title_html,
        slug = html_escape(&form.slug),
        fields_html = fields_html,
        submit_button_text = html_escape(&form.submit_button_text),
        enctype = enctype,
        base_url = html_escape(base_url),
    )
}

pub fn form_success_html(message: &str, slug: &str, base_url: &str) -> String {
    format!(
        r##"<div class="success" hx-get="{base_url}/f/{slug}" hx-select="#form-container form" hx-swap="outerHTML" hx-trigger="load delay:3s">{message}</div>"##,
        message = html_escape(message),
        slug = html_escape(slug),
        base_url = html_escape(base_url)
    )
}

pub fn form_error_html(message: &str) -> String {
    format!(r##"<div class="error"><strong>Error:</strong> {}</div>"##, html_escape(message))
}

pub fn format_digest_email(form: &FormConfig, submissions: &[Submission]) -> String {
    let mut body = format!(
        "New submissions for form: {}\n\nYou have {} new response(s) since the last digest.\n\n",
        form.name,
        submissions.len()
    );

    for (i, sub) in submissions.iter().enumerate() {
        body.push_str(&format!("--- Response #{} ({}) ---\n", i + 1, sub.created_at));
        for field in &form.fields {
            let value = sub.fields_data
                .get(&field.id)
                .map(|v| match v {
                    serde_json::Value::String(s) => s.clone(),
                    _ => v.to_string(),
                })
                .unwrap_or_else(|| "-".to_string());
            body.push_str(&format!("{}: {}\n", field.label, value));
        }
        body.push('\n');
    }

    body
}
