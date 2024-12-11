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
#[deprecated(note = "HealthStatus is deprecated since management api version 0.5.2-SNAPSHOT")]
pub struct HealthStatus {
    #[serde(rename = "componentResults", skip_serializing_if = "Option::is_none")]
    pub component_results: Option<Vec<crate::HealthCheckResult>>,
    #[serde(rename = "isSystemHealthy", skip_serializing_if = "Option::is_none")]
    pub is_system_healthy: Option<bool>,
}

impl HealthStatus {

    pub fn new() -> HealthStatus {
        HealthStatus {
            component_results: None,
            is_system_healthy: None,
        }
    }

}