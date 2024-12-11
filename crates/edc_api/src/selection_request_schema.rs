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
pub struct SelectionRequestSchema {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<crate::DataAddress>>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<crate::DataAddress>>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(rename = "transferType", skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
}

impl SelectionRequestSchema {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_type: Option<String>, destination: Option<Box<crate::DataAddress>>,
               source: Option<Box<crate::DataAddress>>, strategy: Option<String>, transfer_type: Option<String>) -> SelectionRequestSchema {
        SelectionRequestSchema {
            context,
            at_type,
            destination,
            source,
            strategy,
            transfer_type,
        }
    }

    pub fn default() -> SelectionRequestSchema {
        SelectionRequestSchema {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("SelectionRequest".to_string()),
            destination: None,
            source: None,
            strategy: None,
            transfer_type: None,
        }
    }

}