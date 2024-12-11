/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorDetail {
    #[serde(rename = "invalidValue", skip_serializing_if = "Option::is_none")]
    pub invalid_value: Option<serde_json::Value>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ApiErrorDetail {
    pub fn new() -> ApiErrorDetail {
        ApiErrorDetail {
            invalid_value: None,
            message: None,
            path: None,
            r#type: None,
        }
    }
}
