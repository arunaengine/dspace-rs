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
pub struct IdResponse {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl IdResponse {

    pub fn new(at_id: Option<String>, created_at: Option<i64>) -> IdResponse {
        IdResponse {
            at_id,
            created_at,
        }
    }

    pub fn default() -> IdResponse {
        IdResponse {
            at_id: None,
            created_at: None,
        }
    }

}