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
pub struct TerminateTransfer {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "providerPid", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
}

impl TerminateTransfer {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_type: Option<String>, reason: Option<String>, provider_id: Option<String>, transfer_id: Option<String>) -> TerminateTransfer {
        TerminateTransfer {
            context,
            at_type,
            reason,
            provider_id,
            transfer_id,
        }
    }

    pub fn default() -> TerminateTransfer {
        TerminateTransfer {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("TerminateTransfer".to_string()),
            reason: None,
            provider_id: None,
            transfer_id: None,
        }
    }

}