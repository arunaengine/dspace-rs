/*
 * Contract Negotiation [ACK]
 * Sent by:    Consumer, Provider
 * Schema:     TTL Shape, JSON Schema
 *
 * The Contract Negotiation is an object returned by a Consumer or Provider indicating a successful state change happened.
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractNegotiation {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:state")]
    pub state: NegotiationState,
}

impl ContractNegotiation {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
        state: NegotiationState,
    ) -> ContractNegotiation {
        ContractNegotiation {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            state,
        }
    }

    pub fn default() -> ContractNegotiation {
        ContractNegotiation {
            context: std::collections::HashMap::from([(
                "dspace".to_string(),
                serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()),
            )]),
            dsp_type: "dspace:ContractNegotiation".to_string(),
            provider_pid: String::new(),
            consumer_pid: String::new(),
            state: NegotiationState::REQUESTED,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NegotiationState {
    #[serde(
        rename = "dspace:REQUESTED",
        alias = "https://w3id.org/dspace/v0.8/REQUESTED"
    )]
    REQUESTED,
    #[serde(
        rename = "dspace:OFFERED",
        alias = "https://w3id.org/dspace/v0.8/OFFERED"
    )]
    OFFERED,
    #[serde(
        rename = "dspace:ACCEPTED",
        alias = "https://w3id.org/dspace/v0.8/ACCEPTED"
    )]
    ACCEPTED,
    #[serde(
        rename = "dspace:AGREED",
        alias = "https://w3id.org/dspace/v0.8/AGREED"
    )]
    AGREED,
    #[serde(
        rename = "dspace:VERIFIED",
        alias = "https://w3id.org/dspace/v0.8/VERIFIED"
    )]
    VERIFIED,
    #[serde(
        rename = "dspace:FINALIZED",
        alias = "https://w3id.org/dspace/v0.8/FINALIZED"
    )]
    FINALIZED,
    #[serde(
        rename = "dspace:TERMINATED",
        alias = "https://w3id.org/dspace/v0.8/TERMINATED"
    )]
    TERMINATED,
}
