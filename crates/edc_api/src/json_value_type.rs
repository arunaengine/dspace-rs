/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */
use utoipa::ToSchema;

/// Definition is declared but never used in management api version 0.7.0
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum ValueType {
    #[serde(rename = "ARRAY")]
    Array,
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "NUMBER")]
    Number,
    #[serde(rename = "TRUE")]
    True,
    #[serde(rename = "FALSE")]
    False,
    #[serde(rename = "NULL")]
    Null,
}

impl Default for ValueType {
    fn default() -> ValueType {
        Self::Array
    }
}