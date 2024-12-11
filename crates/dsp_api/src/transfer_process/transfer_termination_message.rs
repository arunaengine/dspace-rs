/*
 * Transfer Termination Message
 * Sent by:    Consumer, Provider
 * Results in: TERMINATED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Transfer Termination Message is sent by the Provider or Consumer at any point except a terminal state to indicate the TP should stop and be placed in a terminal state.
 * If the termination was due to an error, the sender may include error information.
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferTerminationMessage {
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

impl TransferTerminationMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
        code: Option<String>,
        reason: Vec<String>,
    ) -> TransferTerminationMessage {
        TransferTerminationMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            code,
            reason,
        }
    }
}
