/*
 * Dataset Request Message
 * Sent by:     Consumer
 * Results in:  TERMINATED
 * Response:    ACK or ERROR
 * Schema:      TTL Shape, JSON Schema
 *
 * The Dataset Request Message is message sent by a Consumer to a Catalog Service.
 * The Catalog Service must respond with a Dataset, which is a valid instance of a DCAT Dataset.
 *
 * The message must have a dataset property which contains the id of the Dataset.
 *
 * The Catalog Service may require an authorization token. Details for including that token can be found in the protocol binding, e.g., Catalog HTTPS Binding.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatasetRequestMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:dataset")]
    pub dataset: String,
}

impl DatasetRequestMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        dataset: String,
    ) -> DatasetRequestMessage {
        DatasetRequestMessage {
            context,
            dsp_type,
            dataset,
        }
    }

    pub fn default() -> DatasetRequestMessage {
        DatasetRequestMessage {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(
                    "https://w3id.org/dspace/2024/1/context.json".to_string(),
                ),
            )]),
            dsp_type: "dspace:DatasetRequestMessage".to_string(),
            dataset: String::new(),
        }
    }
}
