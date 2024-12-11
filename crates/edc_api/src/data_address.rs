/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct DataAddress {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "baseUrl", skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

impl DataAddress {

    pub fn new(at_type: Option<String>, r#type: Option<String>, base_url: Option<String>) -> DataAddress {
        DataAddress {
            at_type,
            r#type,
            base_url,
        }
    }

    pub fn default() -> DataAddress {
        DataAddress {
            at_type: Some("DataAddress".to_string()),
            r#type: None,
            base_url: None,
        }
    }

}