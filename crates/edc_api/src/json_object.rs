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
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, ToSchema)]
pub struct JsonObject {
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "valueType", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<crate::ValueType>,
}

impl JsonObject {

    pub fn new(empty: Option<bool>, value_type: Option<crate::ValueType>) -> JsonObject {
        JsonObject {
            empty,
            value_type,
        }
    }

    pub fn default() -> JsonObject {
        JsonObject {
            empty: None,
            value_type: None,
        }
    }

}