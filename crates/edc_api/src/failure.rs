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
#[deprecated(note = "Failure is deprecated since management api version 0.5.2-SNAPSHOT")]
pub struct Failure {
    #[serde(rename = "failureDetail", skip_serializing_if = "Option::is_none")]
    pub failure_detail: Option<String>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
}

impl Failure {
    pub fn new() -> Failure {
        Failure {
            failure_detail: None,
            messages: None,
        }
    }
}
