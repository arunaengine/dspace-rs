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
pub struct JsonValue {
    #[serde(rename = "valueType", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<crate::ValueType>,
}

impl JsonValue {
    pub fn new(value_type: Option<crate::ValueType>) -> JsonValue {
        JsonValue { value_type }
    }

    pub fn default() -> JsonValue {
        JsonValue { value_type: None }
    }
}
