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
pub struct ContractDefinitionInput {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "accessPolicyId")]
    pub access_policy_id: String,
    #[serde(rename = "assetsSelector")]
    pub assets_selector: Vec<crate::Criterion>,
    #[serde(rename = "contractPolicyId")]
    pub contract_policy_id: String,
}

impl ContractDefinitionInput {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_id: Option<String>, at_type: Option<String>,
               access_policy_id: String, assets_selector: Vec<crate::Criterion>, contract_policy_id: String) -> ContractDefinitionInput {
        ContractDefinitionInput {
            context,
            at_id,
            at_type,
            access_policy_id,
            assets_selector,
            contract_policy_id,
        }
    }

    pub fn default() -> ContractDefinitionInput {
        ContractDefinitionInput {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_id: None,
            at_type: Some("ContractDefinition".to_string()),
            access_policy_id: String::new(),
            assets_selector: Vec::new(),
            contract_policy_id: String::new(),
        }
    }

}