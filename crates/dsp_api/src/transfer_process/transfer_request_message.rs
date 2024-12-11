/*
 * Transfer Request Message
 * Sent by:    Consumer
 * Results in: REQUESTED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Transfer Request Message is sent by a Consumer to initiate a TP.
 *
 * The consumerPid property refers to the transfer id of the Consumer side.
 * The agreementId property refers to an existing contract Agreement between the Consumer and Provider.
 * The dct:format property is a format specified by a Distribution for the Dataset associated with the Agreement. This is generally obtained from the Provider's Catalog.
 * The dataAddress property must only be provided if the dct:format requires a push transfer.
 * The dataAddress contains a transport-specific endpoint address for pushing the data. It may include a temporary authorization via the endpointProperties property.
 * callbackAddress is a URI indicating where messages to the Consumer should be sent. If the address is not understood, the Provider MUST return an UNRECOVERABLE error.
 * The endpointProperties may contain the following optional values:
 * * authorization - An opaque authorization token that clients must present when accessing the transport-specific endpoint address.
 * * authType - The auth token type. For example, the value may be bearer. If present, this value may be used in conjunction with transport rules to define how the client must present an authorization token.
 *
 * Note that Providers should implement idempotent behavior for Transfer Request Messages based on the value of consumerPid.
 * Providers may choose to implement idempotent behavior for a certain period of time.
 * For example, until a TP has completed and been archived after an implementation-specific expiration period, repeated sending of Transfer Request Messages does not change the state of the TP.
 * If a request for the given consumerPid has already been received and the same Consumer sent the original message again, the Provider should respond with an appropriate Transfer Start Message.
 *
 * Once a TP has been created, all associated callback messages must include a consumerPid and providerPid.
 *
 * Providers must include a consumerPid and a providerPid property in the object.
 *
 * Valid states of a TP are REQUESTED, STARTED, TERMINATED, COMPLETED, and SUSPENDED.
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferRequestMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:agreementId")]
    pub agreement_id: String,
    #[serde(rename = "dct:format")]
    pub dct_format: String,
    #[serde(rename = "dspace:dataAddress", skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
    #[serde(rename = "dspace:callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataAddress {
    #[serde(rename = "@type")]
    pub at_type: String,
    #[serde(rename = "dspace:endpointType")]
    pub endpoint_type: String,
    #[serde(rename = "dspace:endpoint")]
    pub endpoint: String,
    #[serde(
        rename = "dspace:endpointProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_properties: Option<Vec<EndpointProperty>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperty {
    #[serde(rename = "@type")]
    pub at_type: String,
    #[serde(rename = "dspace:name")]
    pub name: String,
    #[serde(rename = "dspace:value")]
    pub value: String,
}
