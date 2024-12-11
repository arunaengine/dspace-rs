/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, ToSchema)]
pub struct Criterion {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "operandLeft")]
    pub operand_left: serde_json::Value,
    #[serde(rename = "operandRight")]
    pub operand_right: serde_json::Value,
    #[serde(rename = "operator")]
    pub operator: String,
}

impl Criterion {
    pub fn new(
        at_type: Option<String>,
        operand_left: serde_json::Value,
        operand_right: serde_json::Value,
        operator: String,
    ) -> Criterion {
        Criterion {
            at_type,
            operand_left,
            operand_right,
            operator,
        }
    }

    pub fn default() -> Criterion {
        Criterion {
            at_type: Some("Criterion".to_string()),
            operand_left: serde_json::Value::default(),
            operand_right: serde_json::Value::default(),
            operator: String::new(),
        }
    }
}
