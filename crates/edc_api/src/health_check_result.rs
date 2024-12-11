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
#[deprecated(note = "HealthCheckResult is deprecated since management api version 0.5.2-SNAPSHOT")]
pub struct HealthCheckResult {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "failure", skip_serializing_if = "Option::is_none")]
    pub failure: Option<crate::Failure>,
    #[serde(rename = "isHealthy", skip_serializing_if = "Option::is_none")]
    pub is_healthy: Option<bool>,
}

impl HealthCheckResult {
    pub fn new() -> HealthCheckResult {
        HealthCheckResult {
            component: None,
            failure: None,
            is_healthy: None,
        }
    }
}
