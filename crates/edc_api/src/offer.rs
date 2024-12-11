/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */
use utoipa::ToSchema;

/// ODRL offer
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, ToSchema)]
pub struct Offer {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "@id")]
    pub at_id: String,
    #[serde(rename = "assigner")]
    pub assigner: String,
    #[serde(rename = "target")]
    pub target: String,
}

impl Offer {
    pub fn new(
        context: Option<String>,
        at_type: Option<String>,
        at_id: String,
        assigner: String,
        target: String,
    ) -> Offer {
        Offer {
            context,
            at_type,
            at_id,
            assigner,
            target,
        }
    }

    pub fn default() -> Offer {
        Offer {
            context: Some("http://www.w3.org/ns/odrl.jsonld".to_string()),
            at_type: Some("Offer".to_string()),
            at_id: String::new(),
            assigner: String::new(),
            target: String::new(),
        }
    }
}
