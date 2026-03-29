use serde::{Deserialize, Serialize};

// ============================================================================
// Shared Types
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum DigestFrequency {
    #[default]
    None,
    Daily,
    Weekly,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DigestConfig {
    pub frequency: DigestFrequency,
    #[serde(default)]
    pub responders: Vec<Responder>,
    #[serde(default)]
    pub last_sent_at: Option<String>,
    #[serde(default)]
    pub whatsapp_account_id: Option<String>,
}

/// WhatsApp-only responder. Kept backward-compatible with old FormResponder shape:
/// unknown fields (like `channel`, `subject`) are ignored by serde on deserialization.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Responder {
    pub id: String,
    pub name: String,
    /// Phone number (admin/digest) or booking field name (customer responders)
    pub target_field: String,
    pub body: String,
    pub enabled: bool,
    #[serde(default)]
    pub use_ai: bool,
}

fn default_true() -> bool {
    true
}

// ============================================================================
// Booking Field Types
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Text,
    Email,
    Mobile,
    Phone,
    LongText,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BookingField {
    pub id: String,
    pub label: String,
    pub field_type: FieldType,
    pub required: bool,
    pub placeholder: Option<String>,
}

// ============================================================================
// Tenant Types
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tenant {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub plan: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TenantCredentials {
    #[serde(default)]
    pub google_service_account_email: Option<String>,
    #[serde(default)]
    pub google_private_key: Option<String>,
    /// Deprecated: use WhatsAppAccount resources instead. Kept for migration.
    #[serde(default)]
    pub whatsapp_access_token: Option<String>,
    /// Deprecated: use WhatsAppAccount resources instead. Kept for migration.
    #[serde(default)]
    pub whatsapp_phone_number_id: Option<String>,
}

// ============================================================================
// WhatsApp Account Resource
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhatsAppAccount {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub phone_number: String,
    pub auto_reply: AutoReplyConfig,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AutoReplyConfig {
    pub enabled: bool,
    pub mode: AutoReplyMode,
    pub prompt: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AutoReplyMode {
    #[default]
    Static,
    Ai,
}

/// Stored encrypted separately from the account config
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhatsAppAccountCredentials {
    pub access_token: String,
    pub phone_number_id: String,
}

// ============================================================================
// Google Form Resource
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GoogleFormResource {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub slug: String,
    pub google_form_url: String,
    pub enabled: bool,
    #[serde(default)]
    pub whatsapp_account_id: Option<String>,
    #[serde(default)]
    pub phone_field: String,
    #[serde(default)]
    pub reply_prompt: String,
    #[serde(default)]
    pub use_ai: bool,
    #[serde(default)]
    pub last_polled_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ============================================================================
// Instagram Account Resource
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstagramAccount {
    pub id: String,
    pub tenant_id: String,
    pub instagram_user_id: String,
    pub instagram_username: String,
    #[serde(default)]
    pub target_calendar_id: Option<String>,
    #[serde(default)]
    pub classification_prompt: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub last_synced_at: Option<String>,
    pub created_at: String,
}

// ============================================================================
// Calendar Types
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CalendarConfig {
    pub id: String,
    #[serde(default)]
    pub tenant_id: String,
    pub name: String,
    pub description: Option<String>,
    pub timezone: String,
    pub booking_links: Vec<BookingLink>,
    pub view_links: Vec<ViewLink>,
    #[serde(default)]
    pub google_calendar_id: Option<String>,
    #[serde(default)]
    pub form_links: Vec<FormLink>,
    #[serde(default)]
    pub instagram_sources: Vec<InstagramSource>,
    pub style: CalendarStyle,
    pub allowed_origins: Vec<String>,
    #[serde(default)]
    pub digest: DigestConfig,
    #[serde(default)]
    pub archived: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for CalendarConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            tenant_id: String::new(),
            name: String::from("New Calendar"),
            description: None,
            timezone: String::from("UTC"),
            booking_links: Vec::new(),
            view_links: Vec::new(),
            google_calendar_id: None,
            form_links: Vec::new(),
            instagram_sources: Vec::new(),
            style: CalendarStyle::default(),
            allowed_origins: Vec::new(),
            digest: DigestConfig::default(),
            archived: false,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BookingLink {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub duration: i32,
    pub buffer_before: i32,
    pub buffer_after: i32,
    pub min_notice: i32,
    pub max_advance: i32,
    pub fields: Vec<BookingField>,
    pub confirmation_message: String,
    pub enabled: bool,
    #[serde(default)]
    pub responders: Vec<Responder>,
    #[serde(default = "default_true")]
    pub auto_accept: bool,
    #[serde(default)]
    pub admin_responders: Vec<Responder>,
    #[serde(default)]
    pub hide_title: bool,
    #[serde(default)]
    pub whatsapp_account_id: Option<String>,
}

impl Default for BookingLink {
    fn default() -> Self {
        Self {
            id: String::new(),
            slug: String::new(),
            name: String::from("Book a Meeting"),
            description: None,
            duration: 30,
            buffer_before: 0,
            buffer_after: 0,
            min_notice: 24,
            max_advance: 30,
            fields: vec![
                BookingField {
                    id: String::from("name"),
                    label: String::from("Name"),
                    field_type: FieldType::Text,
                    required: true,
                    placeholder: Some(String::from("Your name")),
                },
                BookingField {
                    id: String::from("email"),
                    label: String::from("Email"),
                    field_type: FieldType::Email,
                    required: true,
                    placeholder: Some(String::from("your@email.com")),
                },
            ],
            confirmation_message: String::from("Your booking has been confirmed!"),
            enabled: true,
            responders: Vec::new(),
            auto_accept: true,
            admin_responders: Vec::new(),
            hide_title: false,
            whatsapp_account_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ViewLink {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub view_type: ViewType,
    pub date_range: Option<DateRange>,
    #[serde(default = "default_true")]
    pub show_events: bool,
    #[serde(default = "default_true")]
    pub show_event_details: bool,
    #[serde(default = "default_true")]
    pub show_bookings: bool,
    #[serde(default = "default_true")]
    pub show_booking_details: bool,
    pub enabled: bool,
}

impl Default for ViewLink {
    fn default() -> Self {
        Self {
            id: String::new(),
            slug: String::new(),
            name: String::from("Calendar View"),
            view_type: ViewType::Month,
            date_range: None,
            show_events: true,
            show_event_details: true,
            show_bookings: true,
            show_booking_details: true,
            enabled: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ViewType {
    Week,
    Month,
    Year,
    Endless,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FormLink {
    pub id: String,
    pub slug: String,
    pub name: String,
    /// Google Form URL (editor URL, e.g. https://docs.google.com/forms/d/{id}/edit)
    pub google_form_url: String,
    pub enabled: bool,
}

impl Default for FormLink {
    fn default() -> Self {
        Self {
            id: String::new(),
            slug: String::new(),
            name: String::from("Contact Form"),
            google_form_url: String::new(),
            enabled: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DateRange {
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CalendarStyle {
    pub primary_color: String,
    pub text_color: String,
    pub background_color: String,
    pub border_radius: String,
    pub font_family: String,
    #[serde(default)]
    pub custom_css: String,
}

impl Default for CalendarStyle {
    fn default() -> Self {
        Self {
            primary_color: String::from("#0070f3"),
            text_color: String::from("#333333"),
            background_color: String::from("#ffffff"),
            border_radius: String::from("4px"),
            font_family: String::from("system-ui, -apple-system, sans-serif"),
            custom_css: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CalendarEvent {
    pub id: String,
    pub calendar_id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub all_day: bool,
    pub recurrence_rule: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimeSlot {
    pub id: String,
    pub calendar_id: String,
    pub day_of_week: Option<i32>,
    pub specific_date: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub slot_duration: i32,
    pub buffer_time: i32,
    pub max_bookings: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Booking {
    pub id: String,
    pub calendar_id: String,
    pub booking_link_id: String,
    pub slot_date: String,
    pub slot_time: String,
    pub duration: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub notes: Option<String>,
    pub fields_data: Option<serde_json::Value>,
    pub status: BookingStatus,
    pub confirmation_token: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BookingStatus {
    Pending,
    Confirmed,
    Cancelled,
    Completed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvailableSlot {
    pub date: String,
    pub time: String,
    pub end_time: String,
    pub available: bool,
}

// ============================================================================
// Instagram Integration Types
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstagramSource {
    pub id: String,
    pub instagram_user_id: String,
    pub instagram_username: String,
    pub enabled: bool,
    pub last_synced_at: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingStatus {
    Pending,
    Processed,
    NoEvent,
    Failed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessedPost {
    pub id: String,
    pub calendar_id: Option<String>,
    #[serde(default)]
    pub form_slug: Option<String>,
    pub instagram_source_id: String,
    pub instagram_post_id: String,
    pub instagram_permalink: String,
    pub caption_hash: String,
    pub event_id: Option<String>,
    pub contact_id: Option<i64>,
    pub event_signature: Option<String>,
    pub processing_status: ProcessingStatus,
    pub ai_response: Option<String>,
    pub processed_at: String,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExtractedEvent {
    pub title: Option<String>,
    pub date: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub description: Option<String>,
    pub is_cancellation: bool,
    pub confidence: f32,
}

impl Default for ExtractedEvent {
    fn default() -> Self {
        Self {
            title: None,
            date: None,
            start_time: None,
            end_time: None,
            description: None,
            is_cancellation: false,
            confidence: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstagramToken {
    pub access_token: String,
    pub expires_at: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstagramMedia {
    pub id: String,
    pub caption: Option<String>,
    pub media_type: String,
    pub permalink: String,
    pub timestamp: String,
}

// ============================================================================
// WhatsApp Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct WhatsAppWebhook {
    pub object: String,
    #[serde(default)]
    pub entry: Vec<WebhookEntry>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEntry {
    pub id: String,
    #[serde(default)]
    pub changes: Vec<WebhookChange>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookChange {
    pub field: String,
    pub value: WebhookValue,
}

#[derive(Debug, Deserialize)]
pub struct WebhookValue {
    pub messaging_product: String,
    pub metadata: WebhookMetadata,
    #[serde(default)]
    pub contacts: Vec<WebhookContact>,
    #[serde(default)]
    pub messages: Vec<WhatsAppMessage>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookMetadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookContact {
    pub wa_id: String,
    pub profile: ContactProfile,
}

#[derive(Debug, Deserialize)]
pub struct ContactProfile {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct WhatsAppMessage {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(default)]
    pub text: Option<TextMessage>,
}

#[derive(Debug, Deserialize)]
pub struct TextMessage {
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct IncomingMessage {
    pub from: String,
    pub sender_name: String,
    pub text: String,
    pub message_id: String,
    pub timestamp: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest_frequency_default() {
        let freq = DigestFrequency::default();
        assert_eq!(freq, DigestFrequency::None);
        assert_eq!(
            serde_json::to_string(&DigestFrequency::Daily).unwrap(),
            "\"daily\""
        );
    }

    #[test]
    fn test_booking_status_serialization() {
        assert_eq!(
            serde_json::to_string(&BookingStatus::Confirmed).unwrap(),
            "\"confirmed\""
        );
        let status: BookingStatus = serde_json::from_str("\"completed\"").unwrap();
        assert_eq!(status, BookingStatus::Completed);
    }

    #[test]
    fn test_view_type_serialization() {
        assert_eq!(
            serde_json::to_string(&ViewType::Month).unwrap(),
            "\"month\""
        );
    }

    #[test]
    fn test_calendar_style_default() {
        let style = CalendarStyle::default();
        assert_eq!(style.primary_color, "#0070f3");
        assert!(style.custom_css.is_empty());
    }

    #[test]
    fn test_booking_link_default() {
        let link = BookingLink::default();
        assert_eq!(link.duration, 30);
        assert!(link.auto_accept);
        assert_eq!(link.fields.len(), 2);
    }

    #[test]
    fn test_calendar_config_default() {
        let config = CalendarConfig::default();
        assert_eq!(config.name, "New Calendar");
        assert_eq!(config.timezone, "UTC");
        assert!(!config.archived);
        assert!(config.google_calendar_id.is_none());
        assert!(config.form_links.is_empty());
    }

    #[test]
    fn test_extracted_event_default() {
        let event = ExtractedEvent::default();
        assert!(event.title.is_none());
        assert!(!event.is_cancellation);
        assert_eq!(event.confidence, 0.0);
    }

    #[test]
    fn test_whatsapp_webhook_deserialization() {
        let json = r#"{
            "object": "whatsapp_business_account",
            "entry": [{
                "id": "123456789",
                "changes": [{
                    "field": "messages",
                    "value": {
                        "messaging_product": "whatsapp",
                        "metadata": {
                            "display_phone_number": "+1234567890",
                            "phone_number_id": "phone-123"
                        },
                        "contacts": [{
                            "wa_id": "user123",
                            "profile": {"name": "Test User"}
                        }],
                        "messages": [{
                            "from": "user123",
                            "id": "msg-123",
                            "timestamp": "1234567890",
                            "type": "text",
                            "text": {"body": "Hello!"}
                        }]
                    }
                }]
            }]
        }"#;

        let webhook: WhatsAppWebhook = serde_json::from_str(json).unwrap();
        assert_eq!(webhook.object, "whatsapp_business_account");
        assert_eq!(
            webhook.entry[0].changes[0].value.messages[0].from,
            "user123"
        );
    }

    #[test]
    fn test_auto_reply_mode_serialization() {
        assert_eq!(
            serde_json::to_string(&AutoReplyMode::Static).unwrap(),
            "\"static\""
        );
        assert_eq!(serde_json::to_string(&AutoReplyMode::Ai).unwrap(), "\"ai\"");
        let mode: AutoReplyMode = serde_json::from_str("\"ai\"").unwrap();
        assert_eq!(mode, AutoReplyMode::Ai);
    }

    #[test]
    fn test_whatsapp_account_credentials_roundtrip() {
        let creds = WhatsAppAccountCredentials {
            access_token: "token123".to_string(),
            phone_number_id: "phone456".to_string(),
        };
        let json = serde_json::to_string(&creds).unwrap();
        let parsed: WhatsAppAccountCredentials = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.access_token, "token123");
        assert_eq!(parsed.phone_number_id, "phone456");
    }

    #[test]
    fn test_backward_compat_old_responder_format() {
        // Old FormResponder data with channel field should still deserialize
        let json = r#"{
            "id": "r1",
            "name": "Test",
            "channel": "twilio_sms",
            "target_field": "+1234567890",
            "subject": "Hello",
            "body": "Test message",
            "enabled": true,
            "use_ai": false
        }"#;
        let r: Responder = serde_json::from_str(json).unwrap();
        assert_eq!(r.target_field, "+1234567890");
        assert_eq!(r.body, "Test message");
    }
}
