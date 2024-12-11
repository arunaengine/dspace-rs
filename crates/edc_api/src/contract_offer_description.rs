/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */
use utoipa::ToSchema;

/// Please use policy instead of offer
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, ToSchema)]
pub struct ContractOfferDescription {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "offerId", skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Please use policy instead of offer")]
    pub offer_id: Option<String>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
}

impl ContractOfferDescription {

    pub fn new(at_type: Option<String>, asset_id: Option<String>, offer_id: Option<String>, policy: Option<serde_json::Value>) -> ContractOfferDescription {
        ContractOfferDescription {
            at_type,
            asset_id,
            offer_id,
            policy,
        }
    }

    pub fn default() -> ContractOfferDescription {
        ContractOfferDescription {
            at_type: Some("ContractOfferDescription".to_string()),
            asset_id: None,
            offer_id: None,
            policy: None,
        }
    }

}