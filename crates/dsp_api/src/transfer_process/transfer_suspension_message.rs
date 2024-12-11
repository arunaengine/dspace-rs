/*
 * Transfer Suspension Message
 * Sent by:    Consumer, Provider
 * Results in: SUSPENDED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Transfer Suspension Message is sent by the Provider or Consumer when either of them needs to temporarily suspend the TP.
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferSuspendMessage {
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

impl TransferSuspendMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
        code: Option<String>,
        reason: Vec<String>,
    ) -> TransferSuspendMessage {
        TransferSuspendMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            code,
            reason,
        }
    }
}
