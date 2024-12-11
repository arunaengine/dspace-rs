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
pub struct PolicyDefinitionInput {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    /// ODRL policy
    #[serde(rename = "policy")]
    pub policy: serde_json::Value,
}

impl PolicyDefinitionInput {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        at_id: Option<String>,
        at_type: Option<String>,
        policy: serde_json::Value,
    ) -> PolicyDefinitionInput {
        PolicyDefinitionInput {
            context,
            at_id,
            at_type,
            policy,
        }
    }

    pub fn default() -> PolicyDefinitionInput {
        PolicyDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_id: None,
            at_type: Some("PolicyDefinition".to_string()),
            policy: serde_json::Value::Null,
        }
    }
}
