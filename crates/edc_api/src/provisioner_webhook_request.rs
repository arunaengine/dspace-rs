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
pub struct ProvisionerWebhookRequest {
    #[serde(rename = "apiKeyJwt", skip_serializing_if = "Option::is_none")]
    pub api_key_jwt: Option<String>,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "contentDataAddress", skip_serializing_if = "Option::is_none")]
    pub content_data_address: Option<Box<crate::DataAddress>>,
    #[serde(rename = "hasToken", skip_serializing_if = "Option::is_none")]
    pub has_token: Option<bool>,
    #[serde(
        rename = "resourceDefinitionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_definition_id: Option<String>,
    #[serde(rename = "resourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

impl ProvisionerWebhookRequest {
    pub fn new(
        api_key_jwt: Option<String>,
        asset_id: Option<String>,
        content_data_address: Option<Box<crate::DataAddress>>,
        has_token: Option<bool>,
        resource_definition_id: Option<String>,
        resource_name: Option<String>,
    ) -> ProvisionerWebhookRequest {
        ProvisionerWebhookRequest {
            api_key_jwt,
            asset_id,
            content_data_address,
            has_token,
            resource_definition_id,
            resource_name,
        }
    }

    pub fn default() -> ProvisionerWebhookRequest {
        ProvisionerWebhookRequest {
            api_key_jwt: None,
            asset_id: None,
            content_data_address: None,
            has_token: None,
            resource_definition_id: None,
            resource_name: None,
        }
    }
}
