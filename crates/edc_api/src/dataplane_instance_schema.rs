/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */


use serde::{Deserialize, Deserializer};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, ToSchema)]
pub struct DataPlaneInstanceSchema {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "allowedDestTypes", deserialize_with = "string_or_vec")]
    pub allowed_dest_types: Vec<String>,
    #[serde(rename = "allowedSourceTypes", deserialize_with = "string_or_vec")]
    pub allowed_source_types: Vec<String>,
    #[serde(rename = "allowedTransferTypes", skip_serializing_if = "Option::is_none")]
    pub allowed_transfer_types: Option<Vec<String>>,
    #[serde(rename = "lastActive", skip_serializing_if = "Option::is_none")]
    pub last_active: Option<i64>,
    #[serde(rename = "turnCount", skip_serializing_if = "Option::is_none")]
    pub turn_count: Option<i32>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    if let Some(s) = v.as_str() {
        Ok(vec![s.to_string()])
    } else if let Some(arr) = v.as_array() {
        let strings: Result<Vec<String>, _> = arr
            .iter()
            .map(|val| {
                val.as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| serde::de::Error::custom("Expected a string in the array"))
            })
            .collect();
        strings
    } else {
        Err(serde::de::Error::custom("Expected either a string or an array of strings"))
    }
}

impl DataPlaneInstanceSchema {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_id: Option<String>, at_type: Option<String>, allowed_dest_types: Vec<String>,
               allowed_source_types: Vec<String>, allowed_transfer_types: Option<Vec<String>>, last_active: Option<i64>, turn_count: Option<i32>, url: String,
               properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> DataPlaneInstanceSchema {
        DataPlaneInstanceSchema {
            context,
            at_id,
            at_type,
            allowed_dest_types,
            allowed_source_types,
            allowed_transfer_types,
            last_active,
            turn_count,
            url,
            properties,
        }
    }

    pub fn default() -> DataPlaneInstanceSchema {
        DataPlaneInstanceSchema {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("DataPlaneInstance".to_string()),
            at_id: None,
            allowed_dest_types: Vec::new(),
            allowed_source_types: Vec::new(),
            allowed_transfer_types: None,
            last_active: None,
            turn_count: None,
            url: String::new(),
            properties: None,
        }
    }

}