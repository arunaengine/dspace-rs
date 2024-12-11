/*
 * Transfer Completion Message
 * Sent by:    Consumer, Provider
 * Results in: COMPLETED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Transfer Completion Message is sent by the Provider or Consumer when a data transfer has completed.
 * Note that some Connector implementations may optimize completion notification by performing it as part of their wire protocol.
 * In those cases, a Transfer Completion Message does not need to be sent.
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferCompletionMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
}

impl TransferCompletionMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
    ) -> TransferCompletionMessage {
        TransferCompletionMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
        }
    }
}
