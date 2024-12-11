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
pub struct TerminateNegotiationSchema {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id")]
    pub at_id: String,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl TerminateNegotiationSchema {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_id: String, at_type: Option<String>, reason: Option<String>) -> TerminateNegotiationSchema {
        TerminateNegotiationSchema {
            context,
            at_id,
            at_type,
            reason,
        }
    }

    pub fn default() -> TerminateNegotiationSchema {
        TerminateNegotiationSchema {
            context: std::collections::HashMap::new(),
            at_id: String::new(),
            at_type: Some("TerminateNegotiation".to_string()),
            reason: None,
        }
    }

}