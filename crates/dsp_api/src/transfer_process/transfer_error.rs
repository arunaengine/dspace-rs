/*
 * Transfer [ERROR]
 * Sent by:    Consumer, Provider
 * Schema:     TTL Shape, JSON Schema
 *
 * The Transfer Error is an object returned by a Consumer or Provider indicating an error has occurred.
 * It does not cause a state transition.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferError {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String, // The TF unique id on Provider side.
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String, // The TF unique id on Consumer side.
    #[serde(rename = "dspace:code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>, // An optional implementation-specific error code.
    #[serde(rename = "dspace:reason", skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<String>, // An optional array of implementation-specific error objects.
}

impl TransferError {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
        code: Option<String>,
        reason: Vec<String>,
    ) -> TransferError {
        TransferError {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            code,
            reason,
        }
    }
}
