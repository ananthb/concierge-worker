//! Google Forms API client for reading form structure and responses.
//!
//! Reuses the same service account auth as Google Calendar.
//! Requires the form to be shared with the service account email.

use serde::Deserialize;
use worker::*;

const GOOGLE_FORMS_API: &str = "https://forms.googleapis.com/v1";

#[derive(Deserialize, Debug)]
pub struct GoogleForm {
    #[serde(rename = "formId")]
    pub form_id: String,
    pub info: FormInfo,
    #[serde(default)]
    pub items: Vec<FormItem>,
}

#[derive(Deserialize, Debug)]
pub struct FormInfo {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct FormItem {
    pub title: Option<String>,
    #[serde(rename = "questionItem")]
    pub question_item: Option<QuestionItem>,
}

#[derive(Deserialize, Debug)]
pub struct QuestionItem {
    pub question: Question,
}

#[derive(Deserialize, Debug)]
pub struct Question {
    #[serde(rename = "questionId")]
    pub question_id: String,
}

#[derive(Deserialize, Debug)]
pub struct FormResponseList {
    #[serde(default)]
    pub responses: Vec<FormResponse>,
}

#[derive(Deserialize, Debug)]
pub struct FormResponse {
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(default)]
    pub answers: std::collections::HashMap<String, FormAnswer>,
}

#[derive(Deserialize, Debug)]
pub struct FormAnswer {
    #[serde(rename = "textAnswers")]
    pub text_answers: Option<TextAnswers>,
}

#[derive(Deserialize, Debug)]
pub struct TextAnswers {
    pub answers: Vec<TextAnswer>,
}

#[derive(Deserialize, Debug)]
pub struct TextAnswer {
    pub value: String,
}

/// Extract the Google Form editor ID from various URL formats.
/// Supports:
/// - https://docs.google.com/forms/d/{ID}/edit
/// - https://docs.google.com/forms/d/e/{ID}/viewform
/// - Raw form ID
pub fn parse_form_id(url_or_id: &str) -> String {
    let url = url_or_id.trim();

    // Editor URL: /forms/d/{ID}/...
    if let Some(rest) = url.strip_prefix("https://docs.google.com/forms/d/") {
        // Check if it's a /d/e/{id} (published) or /d/{id} (editor)
        if let Some(rest) = rest.strip_prefix("e/") {
            return rest.split('/').next().unwrap_or(url).to_string();
        }
        return rest.split('/').next().unwrap_or(url).to_string();
    }

    // Already a raw ID
    url.to_string()
}

/// Build the embeddable Google Form URL from an editor URL or ID.
pub fn embed_url(url_or_id: &str) -> String {
    let form_id = parse_form_id(url_or_id);
    format!(
        "https://docs.google.com/forms/d/e/{}/viewform?embedded=true",
        form_id
    )
}

/// Fetch the form structure (title, fields) from Google Forms API.
pub async fn get_form(
    service_account_email: &str,
    private_key: &str,
    form_id: &str,
) -> Result<GoogleForm> {
    let token = crate::google_calendar::get_access_token_with_scope(
        service_account_email,
        private_key,
        "https://www.googleapis.com/auth/forms.body.readonly",
    )
    .await?;

    let url = format!("{}/forms/{}", GOOGLE_FORMS_API, form_id);

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", token))?;

    let mut init = RequestInit::new();
    init.with_method(Method::Get).with_headers(headers);

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    let text = response.text().await?;

    if response.status_code() != 200 {
        return Err(Error::from(format!(
            "Google Forms API error ({}): {}",
            response.status_code(),
            text
        )));
    }

    serde_json::from_str(&text).map_err(|e| Error::from(format!("Failed to parse form: {}", e)))
}

/// Fetch responses from a Google Form.
pub async fn get_responses(
    service_account_email: &str,
    private_key: &str,
    form_id: &str,
) -> Result<Vec<FormResponse>> {
    let token = crate::google_calendar::get_access_token_with_scope(
        service_account_email,
        private_key,
        "https://www.googleapis.com/auth/forms.responses.readonly",
    )
    .await?;

    let url = format!("{}/forms/{}/responses", GOOGLE_FORMS_API, form_id);

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", token))?;

    let mut init = RequestInit::new();
    init.with_method(Method::Get).with_headers(headers);

    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    let text = response.text().await?;

    if response.status_code() != 200 {
        return Err(Error::from(format!(
            "Google Forms responses API error ({}): {}",
            response.status_code(),
            text
        )));
    }

    let list: FormResponseList = serde_json::from_str(&text)
        .map_err(|e| Error::from(format!("Failed to parse responses: {}", e)))?;

    Ok(list.responses)
}
