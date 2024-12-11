/*
 * Catalog [ERROR]
 * Sent by:    Consumer, Provider
 * Schema:     TTL Shape, JSON Schema
 *
 * A Catalog Error is used when an error occurred after a Catalog Request Message or a Dataset Request Message and the Provider cannot provide its Catalog to the requester.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CatalogError {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>, // An optional implementation-specific error code.
    #[serde(rename = "dspace:reason", skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<String>, // An optional array of implementation-specific error objects.
}
