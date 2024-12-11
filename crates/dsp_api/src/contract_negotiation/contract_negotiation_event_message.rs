/*
 * Contract Negotiation Event Message
 * Sent by:    Provider, Consumer
 * Results in: FINALIZED, ACCEPTED, TERMINATED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * When the Contract Negotiation Event Message is sent by a Provider with an eventType property set to FINALIZED,
 * an Agreement has been finalized and the associated Dataset is accessible.
 * The state machine is transitioned to the FINALIZED state.
 *
 * Other event types may be defined in the future.
 *
 * A Consumer responds with an error if the contract cannot be validated or is incorrect.
 *
 * The message must contain a consumerPid and a providerPid.
 *
 * When the message is sent by a Consumer with an eventType set to ACCEPTED, the state machine is placed in the ACCEPTED state.
 *
 * It is an error for a Consumer to send the message with an event type FINALIZED to the Provider.
 *
 * It is an error for a Provider to send the message with an event type ACCEPTED to the Consumer.
 *
 * Note that CN events are not intended for propagation of an Agreement state after a CN has entered a terminal state.
 * It is considered an error for a Consumer or Provider to send an event after the CN state machine has entered a terminal state.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractNegotiationEventMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:eventType")]
    pub event_type: EventType,
}

impl ContractNegotiationEventMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
        event_type: EventType,
    ) -> ContractNegotiationEventMessage {
        ContractNegotiationEventMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            event_type,
        }
    }

    pub fn default() -> ContractNegotiationEventMessage {
        ContractNegotiationEventMessage {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(
                    "https://w3id.org/dspace/2024/1/context.json".to_string(),
                ),
            )]),
            dsp_type: "dspace:ContractNegotiationEventMessage".to_string(),
            provider_pid: String::new(),
            consumer_pid: String::new(),
            event_type: EventType::ACCEPTED,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "dspace:ACCEPTED")]
    ACCEPTED,
    #[serde(rename = "dspace:FINALIZED")]
    FINALIZED,
}

impl Default for EventType {
    fn default() -> Self {
        EventType::ACCEPTED
    }
}
