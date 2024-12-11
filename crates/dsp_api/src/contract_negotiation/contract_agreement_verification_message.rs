/*
 * Contract Agreement Verification Message
 * Sent by:    Consumer
 * Results in: VERIFIED, TERMINATED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Contract Agreement Verification Message is sent by a Consumer to verify the acceptance of an Agreement.
 *
 * A Provider responds with an error if the contract cannot be validated or is incorrect.
 *
 * The message must contain a consumerPid and a providerPid.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractAgreementVerificationMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
}

impl ContractAgreementVerificationMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
    ) -> ContractAgreementVerificationMessage {
        ContractAgreementVerificationMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
        }
    }

    pub fn default() -> ContractAgreementVerificationMessage {
        ContractAgreementVerificationMessage {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(
                    "https://w3id.org/dspace/2024/1/context.json".to_string(),
                ),
            )]),
            dsp_type: "dspace:ContractAgreementVerificationMessage".to_string(),
            provider_pid: String::new(),
            consumer_pid: String::new(),
        }
    }
}
