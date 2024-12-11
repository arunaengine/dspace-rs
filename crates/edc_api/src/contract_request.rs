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
pub struct ContractRequest {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "callbackAddresses", skip_serializing_if = "Option::is_none")]
    pub callback_addresses: Option<Vec<crate::CallbackAddress>>,
    /// please use counterPartyAddress instead, connectorAddress is deprecated
    #[serde(rename = "connectorAddress", skip_serializing_if = "Option::is_none")]
    #[deprecated(note="Please use counterPartyAddress instead, connectorAddress is deprecated")]
    pub connector_address: Option<String>,
    #[serde(rename = "counterPartyAddress")]
    pub counter_party_address: String,
    #[serde(rename = "offer", skip_serializing_if = "Option::is_none")]
    pub offer: Option<crate::ContractOfferDescription>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<crate::Offer>,
    #[serde(rename = "protocol")]
    pub protocol: String,
    /// please use policy.assigner instead, providerId is deprecated
    #[serde(rename = "providerId", skip_serializing_if = "Option::is_none")]
    #[deprecated(note="Please use policy.assigner instead, providerId is deprecated")]
    pub provider_id: Option<String>,
}

impl ContractRequest {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_type: Option<String>, callback_addresses: Option<Vec<crate::CallbackAddress>>,
               connector_address: Option<String>, counter_party_address: String, offer: Option<crate::ContractOfferDescription>, policy: Option<crate::Offer>,
               protocol: String, provider_id: Option<String>) -> ContractRequest {
        ContractRequest {
            context,
            at_type,
            callback_addresses,
            connector_address,
            counter_party_address,
            offer,
            policy,
            protocol,
            provider_id,
        }
    }

    pub fn default() -> ContractRequest {
        ContractRequest {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("ContractRequest".to_string()),
            callback_addresses: None,
            connector_address: None,
            counter_party_address: String::new(),
            offer: None,
            policy: None,
            protocol: String::new(),
            provider_id: None,
        }
    }

}