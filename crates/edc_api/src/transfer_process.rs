/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

use serde_with::{formats::PreferMany, serde_as, OneOrMany};
use utoipa::ToSchema;
use crate::transfer_state::TransferProcessState;

#[serde_as]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, ToSchema)]
pub struct TransferProcess {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(rename = "callbackAddresses", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub callback_addresses: Vec<crate::CallbackAddress>,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "contractId", skip_serializing_if = "Option::is_none")]
    pub contract_agreement_id: Option<String>,
    #[serde(rename = "counterPartyAddress", skip_serializing_if = "Option::is_none")]
    pub counter_party_address: Option<String>,
    #[serde(rename = "counterPartyId", skip_serializing_if = "Option::is_none")]
    pub counter_party_id: Option<String>,
    #[serde(rename = "dataDestination", skip_serializing_if = "Option::is_none")]
    pub data_destination: Option<Box<crate::DataAddress>>,
    #[serde(rename = "errorDetail", skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    #[serde(rename = "privateProperties", default)]
    pub private_properties: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<TransferProcessState>,
    #[serde(rename = "stateTimestamp", skip_serializing_if = "Option::is_none")]
    pub state_timestamp: Option<i64>,
    #[serde(rename = "transferType", skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
}

impl TransferProcess {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_type: Option<String>, at_id: Option<String>, correlation_id: Option<String>, callback_addresses: Vec<crate::CallbackAddress>,
               asset_id: Option<String>, contract_agreement_id: Option<String>, counter_party_address: Option<String>, counter_party_id: Option<String>, data_destination: Option<Box<crate::DataAddress>>,
               error_detail: Option<String>, private_properties: Option<::std::collections::HashMap<String, String>>, protocol: Option<String>, state: Option<TransferProcessState>,
               state_timestamp: Option<i64>, transfer_type: Option<String>, r#type: Option<RHashType>) -> TransferProcess {
        TransferProcess {
            context,
            at_type,
            at_id,
            correlation_id,
            callback_addresses,
            asset_id,
            contract_agreement_id,
            counter_party_address,
            counter_party_id,
            data_destination,
            error_detail,
            private_properties,
            protocol,
            state,
            state_timestamp,
            transfer_type,
            r#type,
        }
    }

    pub fn default() -> TransferProcess {
        TransferProcess {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("TransferProcess".to_string()),
            at_id: None,
            correlation_id: None,
            callback_addresses: Vec::new(),
            asset_id: None,
            contract_agreement_id: None,
            counter_party_address: None,
            counter_party_id: None,
            data_destination: None,
            error_detail: None,
            private_properties: None,
            protocol: None,
            state: None,
            state_timestamp: None,
            transfer_type: None,
            r#type: None,
        }
    }

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum RHashType {
    #[serde(rename = "CONSUMER")]
    Consumer,
    #[serde(rename = "PROVIDER")]
    Provider,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Consumer
    }
}