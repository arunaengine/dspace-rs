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
pub struct CatalogRequest {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "counterPartyAddress")]
    pub counter_party_address: String,
    #[serde(rename = "counterPartyId", skip_serializing_if = "Option::is_none")]
    pub counter_party_id: Option<String>,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "querySpec", skip_serializing_if = "Option::is_none")]
    pub query_spec: Option<crate::QuerySpec>,
}

impl CatalogRequest {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        at_type: Option<String>,
        counter_party_address: String,
        counter_party_id: Option<String>,
        protocol: String,
        query_spec: Option<crate::QuerySpec>,
    ) -> CatalogRequest {
        CatalogRequest {
            context,
            at_type,
            counter_party_address,
            counter_party_id,
            protocol,
            query_spec,
        }
    }

    pub fn default() -> CatalogRequest {
        CatalogRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("CatalogRequest".to_string()),
            counter_party_address: String::new(),
            counter_party_id: None,
            protocol: String::new(),
            query_spec: None,
        }
    }
}
