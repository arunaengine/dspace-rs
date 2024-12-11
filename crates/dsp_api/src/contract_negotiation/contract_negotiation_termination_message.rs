/*
 * Contract Negotiation Event Message
 * Sent by:    Consumer, Provider
 * Results in: TERMINATED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Contract Negotiation Termination Message is sent by a Consumer or Provider indicating it has cancelled the CN sequence.
 * The message can be sent at any state of a CN without providing an explanation.
 * Nevertheless, the sender may provide a description to help the receiver.
 *
 * The message must contain a consumerPid and a providerPid.
 *
 * If an error is received in response to the message, the sending party may choose to ignore the error.
 *
 * Note that a CN may be terminated for a variety of reasons, for example, an unrecoverable error was encountered or one of the parties no longer wishes to continue.
 * A Connector's operator may remove terminated CN resources after it has reached the terminated state.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractNegotiationTerminationMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "dspace:reason", skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<String>,
}

impl ContractNegotiationTerminationMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
        code: Option<String>,
        reason: Vec<String>,
    ) -> ContractNegotiationTerminationMessage {
        ContractNegotiationTerminationMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            code,
            reason,
        }
    }

    pub fn default() -> ContractNegotiationTerminationMessage {
        ContractNegotiationTerminationMessage {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(
                    "https://w3id.org/dspace/2024/1/context.json".to_string(),
                ),
            )]),
            dsp_type: "dspace:ContractNegotiationTerminationMessage".to_string(),
            provider_pid: String::new(),
            consumer_pid: String::new(),
            code: None,
            reason: Vec::new(),
        }
    }
}
