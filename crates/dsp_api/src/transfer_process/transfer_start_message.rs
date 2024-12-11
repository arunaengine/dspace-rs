/*
 * Transfer Start Message
 * Sent by:    Provider
 * Results in: STARTED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Transfer Start Message is sent by the Provider to indicate the data transfer has been initiated.
 *
 * The dataAddress is only provided if the current transfer is a pull transfer and contains a transport-specific endpoint address for obtaining the data.
 * It may include a temporary authorization via the endpointProperties property.
 *
 * The endpointProperties may contain the following optional values:
 * * authorization - An opaque authorization token that clients must present when accessing the transport-specific endpoint address.
 * * authType - The auth token type. For example, the value may be bearer. If present, this value may be used in conjunction with transport rules to define how the client must present an authorization token.
 */

use crate::transfer_process::DataAddress;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferStartMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:dataAddress")]
    pub data_address: DataAddress,
}

impl TransferStartMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
        data_address: DataAddress,
    ) -> TransferStartMessage {
        TransferStartMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            data_address,
        }
    }
}
