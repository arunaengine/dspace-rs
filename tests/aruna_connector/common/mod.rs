extern crate dsp_client;

use uuid::Uuid;
use edc_api::{AssetInput, ContractDefinitionInput, ContractOfferDescription, ContractRequest, Criterion, DataAddress, Offer, PolicyDefinitionInput};
use edc_client::{asset_api, contract_definition_api, contract_negotiation_api, policy_definition_api};
use edc_client::configuration::ApiKey;
use edc_client::configuration::Configuration;
use odrl::name_spaces::{EDC_NS, LD_NS};

pub const PROVIDER_PROTOCOL: &str = "http://localhost:3000";
pub const PROVIDER_ID: &str = "aruna-connector";
pub const DATASPACE_PROTOCOL: &str = "dataspace-protocol-http";

pub fn setup_provider_configuration() -> Configuration {
    let mut provider = Configuration::default();
    provider.base_path = "http://localhost:3000".to_string();
    provider.api_key = Some(ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    provider.with_headers()
}

pub async fn setup_consumer_configuration() -> Configuration {
    let mut consumer = Configuration::default();
    consumer.base_path = "http://localhost:9194/protocol".to_string();
    consumer.with_headers()
}

pub async fn setup_management_consumer() -> Configuration {
    let mut management_consumer = Configuration::default();
    management_consumer.base_path = "http://localhost:9193/management".to_owned();
    management_consumer.api_key = Some(ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    management_consumer.with_headers()
}

pub async fn setup_dsp_consumer_configuration() -> dsp_client::configuration::Configuration {
    let mut dsp_consumer = dsp_client::configuration::Configuration::default();
    dsp_consumer.base_path = "http://localhost:9194/protocol".to_owned();
    dsp_consumer.api_key = Some(dsp_client::configuration::ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    dsp_consumer.with_headers()
}

pub async fn setup_dsp_provider_configuration() -> dsp_client::configuration::Configuration {
    let mut provider = dsp_client::configuration::Configuration::default();
    provider.base_path = "http://localhost:3000".to_string();
    provider.api_key = Some(dsp_client::configuration::ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    provider.with_headers()
}

pub async fn setup_random_asset(
    configuration: &Configuration,
) -> (String) {
    let asset = AssetInput {
        context: std::collections::HashMap::from([(
            "@vocab".to_string(),
            serde_json::Value::String(EDC_NS.to_string()),
        )]),
        at_id: Some(Uuid::new_v4().to_string()),
        at_type: Some("Asset".to_string()),
        data_address: Box::new(DataAddress {
            at_type: Some("DataAddress".to_string()),
            r#type: Some("HttpData".to_string()),
            base_url: Some("https://jsonplaceholder.typicode.com/users".to_string()),
        }),
        private_properties: None,
        properties: Default::default(),
    };

    let asset_response = asset_api::create_asset(&configuration, Some(asset))
        .await
        .unwrap();

    asset_response.clone().at_id.unwrap()
}

pub async fn setup_random_contract_definition(
    configuration: &Configuration,
) -> (String, String, String) {
    let asset = setup_random_asset(&configuration).await;

    let test_policy = r#"
    {
        "@context": "http://www.w3.org/ns/odrl.jsonld",
        "@type": "Set",
        "uid": "api_test_policy",
        "permission": []
    }
    "#;

    // Create policy with random id
    let policy_definition = PolicyDefinitionInput {
        context: std::collections::HashMap::from([(
            "@vocab".to_string(),
            serde_json::Value::String(EDC_NS.to_string()),
        )]),
        at_id: Some(Uuid::new_v4().to_string()),
        at_type: Some("PolicyDefinition".to_string()),
        policy: serde_json::from_str(test_policy).unwrap(),
    };

    let policy_response =
        policy_definition_api::create_policy_definition(&configuration, Some(policy_definition))
            .await
            .unwrap();

    // Create contract definition with random id containing previously created asset and policy
    let contract_definition = ContractDefinitionInput {
        context: std::collections::HashMap::from([(
            "@vocab".to_string(),
            serde_json::Value::String(EDC_NS.to_string()),
        )]),
        at_id: Some(Uuid::new_v4().to_string()),
        at_type: Some("ContractDefinition".to_string()),
        access_policy_id: policy_response.clone().at_id.unwrap(),
        assets_selector: vec![Criterion {
            at_type: Some("Criterion".to_string()),
            operand_left: serde_json::Value::from(format!("{}{}", EDC_NS, "id")),
            operand_right: serde_json::Value::from(asset.clone()),
            operator: "=".to_string(),
        }],
        contract_policy_id: policy_response.clone().at_id.unwrap(),
    };

    let definition_response = contract_definition_api::create_contract_definition(
        &configuration,
        Some(contract_definition),
    )
        .await
        .unwrap();

    (
        asset.clone(),
        policy_response.clone().at_id.unwrap(),
        definition_response.clone().at_id.unwrap(),
    )
}

pub async fn setup_random_contract_negotiation(
    consumer: &Configuration,
    provider: &Configuration,
) -> (String, String) {
    let asset_id = setup_random_asset(&provider).await;

    let offer_id = Uuid::new_v4().to_string();

    let offer = Offer {
        context: Some(LD_NS.to_string()),
        at_type: Some("Offer".to_string()),
        at_id: offer_id.clone(),
        assigner: PROVIDER_ID.to_string(),
        target: asset_id.clone(),
    };

    let policy: serde_json::Value = serde_json::json!({
        "@context": offer.context,
        "@type": offer.at_type,
        "@id": offer.at_id,
        "assigner": offer.assigner,
        "target": offer.target,
    });

    let offer_description = ContractOfferDescription {
        at_type: Some("OfferDescription".to_string()),
        asset_id: Some(asset_id.clone()),
        offer_id: Some(offer_id),
        policy: Some(policy),
    };

    let contract_request = ContractRequest {
        context: std::collections::HashMap::from([(
            "@vocab".to_string(),
            serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
        )]),
        at_type: Some("ContractRequest".to_string()),
        callback_addresses: None,
        connector_address: None,
        counter_party_address: PROVIDER_PROTOCOL.to_string(),
        offer: Some(offer_description),
        policy: Some(offer),
        protocol: DATASPACE_PROTOCOL.to_string(),
        provider_id: None,
    };

    let response =
        contract_negotiation_api::initiate_contract_negotiation(&consumer, Some(contract_request))
            .await
            .unwrap();

    (response.clone().at_id.unwrap(), asset_id.clone())
}