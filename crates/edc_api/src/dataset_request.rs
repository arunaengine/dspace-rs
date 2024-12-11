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
pub struct DatasetRequest {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "counterPartyAddress", skip_serializing_if = "Option::is_none")]
    pub counter_party_address: Option<String>,
    #[serde(rename = "counterPartyId", skip_serializing_if = "Option::is_none")]
    pub counter_party_id: Option<String>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "querySpec", skip_serializing_if = "Option::is_none")]
    pub query_spec: Option<crate::QuerySpec>,
}

impl DatasetRequest {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_type: Option<String>, at_id: Option<String>, counter_party_address: Option<String>,
               counter_party_id: Option<String>, protocol: Option<String>, query_spec: Option<crate::QuerySpec>) -> DatasetRequest {
        DatasetRequest {
            context,
            at_type,
            at_id,
            counter_party_address,
            counter_party_id,
            protocol,
            query_spec,
        }
    }

    pub fn default() -> DatasetRequest {
        DatasetRequest {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("DatasetRequest".to_string()),
            at_id: None,
            counter_party_address: None,
            counter_party_id: None,
            protocol: None,
            query_spec: None,
        }
    }

}