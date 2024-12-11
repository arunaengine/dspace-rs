/*
 * Contract Request Message
 * Sent by:    Consumer
 * Results in: REQUESTED, TERMINATED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Contract Request Message is sent by a Consumer to initiate a CN or to respond to a Contract Offer Message sent by a Provider.
 *
 * The Consumer must include an offer property, which itself must have an @id property.
 *
 * If the message includes a providerPid property, the request will be associated with an existing CN
 * and a Consumer Offer will be created using either the offer or offer.@id properties.
 *
 * If the message does not include a providerPid, a new CN will be created on Provider side using
 * either the offer or offer.@id properties and the Provider selects an appropriate providerPid.
 *
 * An offer.@id will generally refer to an Offer contained in a Catalog.
 * If the Provider is not aware of the offer.@id value, it must respond with an error message.
 *
 * The callbackAddress is a URL indicating where messages to the Consumer should be sent in asynchronous settings.
 * If the address is not understood, the Provider MUST return an UNRECOVERABLE error.
 *
 * Different to a Catalog or Dataset, the Offer inside a Contract Request Message must have an odrl:target attribute.
 * However, it's contained Rules must not have any odrl:target attributes to prevent inconsistencies with the ODRL inferencing rules for compact policies.
 */

use crate::contract_negotiation::MessageOffer;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractRequestMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid", skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<String>,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:offer")]
    pub offer: MessageOffer,
    #[serde(rename = "dspace:callbackAddress")]
    pub callback_address: String,
}

impl ContractRequestMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: Option<String>,
        consumer_pid: String,
        offer: MessageOffer,
        callback_address: String,
    ) -> ContractRequestMessage {
        ContractRequestMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            offer,
            callback_address,
        }
    }
}
