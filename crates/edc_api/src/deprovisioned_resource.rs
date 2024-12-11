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
pub struct DeprovisionedResource {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "inProcess", skip_serializing_if = "Option::is_none")]
    pub in_process: Option<bool>,
    #[serde(
        rename = "provisionedResourceId",
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioned_resource_id: Option<String>,
}

impl DeprovisionedResource {
    pub fn new(
        error: Option<bool>,
        error_message: Option<String>,
        in_process: Option<bool>,
        provisioned_resource_id: Option<String>,
    ) -> DeprovisionedResource {
        DeprovisionedResource {
            error,
            error_message,
            in_process,
            provisioned_resource_id,
        }
    }

    pub fn default() -> DeprovisionedResource {
        DeprovisionedResource {
            error: None,
            error_message: None,
            in_process: None,
            provisioned_resource_id: None,
        }
    }
}
