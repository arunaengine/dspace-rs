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
pub struct ContractNegotiation {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "callbackAddresses", skip_serializing_if = "Option::is_none")]
    pub callback_addresses: Option<Vec<crate::CallbackAddress>>,
    #[serde(rename = "contractAgreementId", skip_serializing_if = "Option::is_none")]
    pub contract_agreement_id: Option<String>,
    #[serde(rename = "counterPartyAddress", skip_serializing_if = "Option::is_none")]
    pub counter_party_address: Option<String>,
    #[serde(rename = "counterPartyId", skip_serializing_if = "Option::is_none")]
    pub counter_party_id: Option<String>,
    #[serde(rename = "errorDetail", skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "state")]
    pub state: ContractNegotiationState,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EnumType>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl ContractNegotiation {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_id: Option<String>, at_type: Option<String>, callback_addresses: Option<Vec<crate::CallbackAddress>>,
               contract_agreement_id: Option<String>, counter_party_address: Option<String>, counter_party_id: Option<String>, error_detail: Option<String>, protocol: Option<String>,
               state: ContractNegotiationState, r#type: Option<EnumType>, created_at: Option<i64>) -> ContractNegotiation {
        ContractNegotiation {
            context,
            at_id,
            at_type,
            callback_addresses,
            contract_agreement_id,
            counter_party_address,
            counter_party_id,
            error_detail,
            protocol,
            state,
            r#type,
            created_at,
        }
    }

    pub fn default() -> ContractNegotiation {
        ContractNegotiation {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_id: None,
            at_type: Some("ContractNegotiation".to_string()),
            callback_addresses: None,
            contract_agreement_id: None,
            counter_party_address: None,
            counter_party_id: None,
            error_detail: None,
            protocol: None,
            state: ContractNegotiationState::Initial,
            r#type: None,
            created_at: None,
        }
    }

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum EnumType {
    #[serde(rename = "CONSUMER")]
    Consumer,
    #[serde(rename = "PROVIDER")]
    Provider,
}

impl Default for EnumType {
    fn default() -> EnumType {
        Self::Consumer
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub struct NegotiationState {
    pub state: ContractNegotiationState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum ContractNegotiationState {
    #[serde(rename = "INITIAL")]
    Initial,        // Consumer and Provider
    #[serde(rename = "REQUESTING")]
    Requesting,     // Consumer
    #[serde(rename = "REQUESTED")]
    Requested,      // Consumer and Provider
    #[serde(rename = "OFFERING")]
    Offering,       // Provider
    #[serde(rename = "OFFERED")]
    Offered,        // Consumer and Provider
    #[serde(rename = "ACCEPTING")]
    Accepting,      // Consumer
    #[serde(rename = "ACCEPTED")]
    Accepted,       // Consumer and Provider
    #[serde(rename = "AGREEING")]
    Agreeing,       // Provider
    #[serde(rename = "AGREED")]
    Agreed,         // Consumer and Provider
    #[serde(rename = "VERIFYING")]
    Verifying,      // Consumer
    #[serde(rename = "VERIFIED")]
    Verified,       // Consumer and Provider
    #[serde(rename = "FINALIZING")]
    Finalizing,     // Provider
    #[serde(rename = "FINALIZED")]
    Finalized,      // Consumer and Provider
    #[serde(rename = "TERMINATING")]
    Terminating,    // Consumer and Provider
    #[serde(rename = "TERMINATED")]
    Terminated,     // Consumer and Provider
}

impl Default for ContractNegotiationState {
    fn default() -> ContractNegotiationState {
        Self::Initial
    }
}