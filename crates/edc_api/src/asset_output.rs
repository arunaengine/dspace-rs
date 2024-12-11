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
pub struct AssetOutput {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "dataAddress", skip_serializing_if = "Option::is_none")]
    pub data_address: Option<Box<crate::DataAddress>>,
    #[serde(rename = "privateProperties", skip_serializing_if = "Option::is_none")]
    pub private_properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl AssetOutput {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_id: Option<String>, at_type: Option<String>,
               created_at: Option<i64>, data_address: crate::DataAddress, private_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
               properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> AssetOutput {
        AssetOutput {
            context,
            at_id,
            at_type,
            created_at,
            data_address: Some(Box::new(data_address)),
            private_properties,
            properties,
        }
    }

    pub fn default() -> AssetOutput {
        AssetOutput {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_id: None,
            at_type: Some("Asset".to_string()),
            created_at: None,
            data_address: None,
            private_properties: None,
            properties: None,
        }
    }

}