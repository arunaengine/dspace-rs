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
pub struct ContractDefinitionOutput {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "accessPolicyId", skip_serializing_if = "Option::is_none")]
    pub access_policy_id: Option<String>,
    #[serde(rename = "assetsSelector", skip_serializing_if = "Option::is_none")]
    pub assets_selector: Option<Vec<crate::Criterion>>,
    #[serde(rename = "contractPolicyId", skip_serializing_if = "Option::is_none")]
    pub contract_policy_id: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl ContractDefinitionOutput {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        at_id: Option<String>,
        at_type: Option<String>,
        access_policy_id: Option<String>,
        assets_selector: Option<Vec<crate::Criterion>>,
        contract_policy_id: Option<String>,
        created_at: Option<i64>,
    ) -> ContractDefinitionOutput {
        ContractDefinitionOutput {
            context,
            at_id,
            at_type,
            access_policy_id,
            assets_selector,
            contract_policy_id,
            created_at,
        }
    }

    pub fn default() -> ContractDefinitionOutput {
        ContractDefinitionOutput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_id: None,
            at_type: Some("ContractDefinition".to_string()),
            access_policy_id: None,
            assets_selector: None,
            contract_policy_id: None,
            created_at: None,
        }
    }
}
