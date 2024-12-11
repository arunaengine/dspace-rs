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
pub struct CallbackAddress {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "authCodeId", skip_serializing_if = "Option::is_none")]
    pub auth_code_id: Option<String>,
    #[serde(rename = "authKey", skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "transactional", skip_serializing_if = "Option::is_none")]
    pub transactional: Option<bool>,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl CallbackAddress {
    pub fn new(
        at_type: Option<String>,
        auth_code_id: Option<String>,
        auth_key: Option<String>,
        events: Option<Vec<String>>,
        transactional: Option<bool>,
        uri: Option<String>,
    ) -> CallbackAddress {
        CallbackAddress {
            at_type,
            auth_code_id,
            auth_key,
            events,
            transactional,
            uri,
        }
    }

    pub fn default() -> CallbackAddress {
        CallbackAddress {
            at_type: Some("CallbackAddress".to_string()),
            auth_code_id: None,
            auth_key: None,
            events: None,
            transactional: None,
            uri: None,
        }
    }
}
