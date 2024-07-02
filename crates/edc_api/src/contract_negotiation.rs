/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
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
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EnumType>,
}

impl ContractNegotiation {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_id: Option<String>, at_type: Option<String>, callback_addresses: Option<Vec<crate::CallbackAddress>>,
               contract_agreement_id: Option<String>, counter_party_address: Option<String>, counter_party_id: Option<String>, error_detail: Option<String>, protocol: Option<String>,
               state: Option<String>, r#type: Option<EnumType>) -> ContractNegotiation {
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
            state: None,
            r#type: None,
        }
    }

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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