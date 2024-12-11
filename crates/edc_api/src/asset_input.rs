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
pub struct AssetInput {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "dataAddress")]
    pub data_address: Box<crate::DataAddress>,
    #[serde(rename = "privateProperties", skip_serializing_if = "Option::is_none")]
    pub private_properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "properties")]
    pub properties: std::collections::HashMap<String, serde_json::Value>,
}

impl AssetInput {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        at_id: Option<String>,
        at_type: Option<String>,
        data_address: crate::DataAddress,
        private_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
        properties: std::collections::HashMap<String, serde_json::Value>,
    ) -> AssetInput {
        AssetInput {
            context,
            at_id,
            at_type,
            data_address: Box::new(data_address),
            private_properties,
            properties,
        }
    }

    pub fn default() -> AssetInput {
        AssetInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_id: None,
            at_type: Some("Asset".to_string()),
            data_address: Box::new(crate::DataAddress::default()),
            private_properties: None,
            properties: std::collections::HashMap::new(),
        }
    }
}
