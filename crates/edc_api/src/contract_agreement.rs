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
pub struct ContractAgreement {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "consumerId", skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>,
    #[serde(
        rename = "contractSigningDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_signing_date: Option<i64>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
    #[serde(rename = "providerId", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
}

impl ContractAgreement {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        at_id: Option<String>,
        at_type: Option<String>,
        asset_id: Option<String>,
        consumer_id: Option<String>,
        contract_signing_date: Option<i64>,
        policy: Option<serde_json::Value>,
        provider_id: Option<String>,
    ) -> ContractAgreement {
        ContractAgreement {
            context,
            at_id,
            at_type,
            asset_id,
            consumer_id,
            contract_signing_date,
            policy,
            provider_id,
        }
    }

    pub fn default() -> ContractAgreement {
        ContractAgreement {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_id: None,
            at_type: Some("ContractAgreement".to_string()),
            asset_id: None,
            consumer_id: None,
            contract_signing_date: None,
            policy: None,
            provider_id: None,
        }
    }
}
