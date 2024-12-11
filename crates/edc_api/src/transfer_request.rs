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
pub struct TransferRequest {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "callbackAddresses", skip_serializing_if = "Option::is_none")]
    pub callback_addresses: Option<Vec<crate::CallbackAddress>>,
    /// please use counterPartyAddress instead, connectorAddress is deprecated
    #[serde(rename = "connectorAddress", skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "please use counterPartyAddress instead, connectorAddress is deprecated")]
    pub connector_address: Option<String>,
    /// Provider connector id is stored in the contract agreement, connectorId is deprecated
    #[serde(rename = "connectorId", skip_serializing_if = "Option::is_none")]
    #[deprecated(
        note = "Provider connector id is stored in the contract agreement, connectorId is deprecated"
    )]
    pub connector_id: Option<String>,
    #[serde(rename = "contractId")]
    pub contract_id: String,
    #[serde(rename = "counterPartyAddress")]
    pub counter_party_address: String,
    #[serde(rename = "dataDestination")]
    pub data_destination: Box<crate::DataAddress>,
    #[serde(rename = "privateProperties", skip_serializing_if = "Option::is_none")]
    pub private_properties: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "transferType")]
    pub transfer_type: String,
}

impl TransferRequest {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        at_type: Option<String>,
        asset_id: String,
        callback_addresses: Option<Vec<crate::CallbackAddress>>,
        connector_address: Option<String>,
        connector_id: Option<String>,
        contract_id: String,
        counter_party_address: String,
        data_destination: Box<crate::DataAddress>,
        private_properties: Option<::std::collections::HashMap<String, String>>,
        protocol: String,
        transfer_type: String,
    ) -> TransferRequest {
        TransferRequest {
            context,
            at_type,
            asset_id,
            callback_addresses,
            connector_address,
            connector_id,
            contract_id,
            counter_party_address,
            data_destination,
            private_properties,
            protocol,
            transfer_type,
        }
    }

    pub fn default() -> TransferRequest {
        TransferRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("TransferRequest".to_string()),
            asset_id: String::new(),
            callback_addresses: None,
            connector_address: None,
            connector_id: None,
            contract_id: String::new(),
            counter_party_address: String::new(),
            data_destination: Box::new(crate::DataAddress::default()),
            private_properties: None,
            protocol: String::new(),
            transfer_type: String::new(),
        }
    }
}
