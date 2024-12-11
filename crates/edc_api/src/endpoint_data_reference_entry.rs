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
pub struct EndpointDataReferenceEntry {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
}

impl EndpointDataReferenceEntry {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        at_type: Option<String>,
        at_id: Option<String>,
    ) -> EndpointDataReferenceEntry {
        EndpointDataReferenceEntry {
            context,
            at_type,
            at_id,
        }
    }

    pub fn default() -> EndpointDataReferenceEntry {
        EndpointDataReferenceEntry {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("EndpointDataReferenceEntry".to_string()),
            at_id: None,
        }
    }
}
