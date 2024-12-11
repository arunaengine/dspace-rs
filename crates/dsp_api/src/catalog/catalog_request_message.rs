/*
 * Catalog Request Message
 * Sent by:     Consumer
 * Results in:  TERMINATED
 * Response:    ACK or ERROR
 * Schema:      TTL Shape, JSON Schema
 *
 * The Catalog Request Message is message sent by a Consumer to a Catalog Service.
 * The Catalog Service must respond with a Catalog, which is a valid instance of a DCAT Catalog.
 *
 * The message may have a filter property which contains an implementation-specific query or filter expression type supported by the Catalog Service.
 *
 * The Catalog Service may require an authorization token. Details for including that token can be found in the protocol binding, e.g.,
 * Catalog HTTPS Binding. Similarly, pagination may be defined in the protocol binding.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CatalogRequestMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:filter", skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<String>, // TODO: Items are not defined in the spec, is String correct?
}

impl CatalogRequestMessage {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        filter: Vec<String>,
    ) -> CatalogRequestMessage {
        CatalogRequestMessage {
            context,
            dsp_type,
            filter,
        }
    }

    pub fn default() -> CatalogRequestMessage {
        CatalogRequestMessage {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(
                    "https://w3id.org/dspace/2024/1/context.json".to_string(),
                ),
            )]),
            dsp_type: "dspace:CatalogRequestMessage".to_string(),
            filter: Vec::new(),
        }
    }
}
