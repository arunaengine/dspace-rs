/*
 * Contract Negotiation [ERROR]
 * Sent by:    Consumer, Provider
 * Schema:     TTL Shape, JSON Schema
 *
 * The Contract Negotiation Error is an object returned by a Consumer or Provider indicating an error has occurred.
 * It does not cause a state transition.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractNegotiationError {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")] // The CN unique id on Provider side.
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")] // The CN unique id on Consumer side.
    pub consumer_pid: String,
    #[serde(rename = "dspace:code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>, // An optional implementation-specific error code.
    #[serde(rename = "dspace:reason", skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<String>, // An optional array of implementation-specific error objects.
    #[serde(rename = "dct:description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<DctDescription>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DctDescription {
    #[serde(rename = "@language")]
    pub language: String,
    #[serde(rename = "@value")]
    pub value: String,
}

impl ContractNegotiationError {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
        code: Option<String>,
        reason: Vec<String>,
        description: Option<Vec<DctDescription>>,
    ) -> ContractNegotiationError {
        ContractNegotiationError {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            code,
            reason,
            description,
        }
    }

    pub fn default() -> ContractNegotiationError {
        ContractNegotiationError {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(
                    "https://w3id.org/dspace/2024/1/context.json".to_string(),
                ),
            )]),
            dsp_type: "dspace:ContractNegotiationError".to_string(),
            provider_pid: String::new(),
            consumer_pid: String::new(),
            code: None,
            reason: Vec::new(),
            description: None,
        }
    }
}

impl DctDescription {
    pub fn new(language: String, value: String) -> DctDescription {
        DctDescription { language, value }
    }
}
