extern crate dsp_client;

use dsp_client::configuration::Configuration;
use dsp_client::contract_negotiation::negotiation_provider_api::GetNegotiationError;
use dsp_client::Error;
use edc_api::{
    AssetInput, ContractDefinitionInput, ContractOfferDescription, ContractRequest, Criterion,
    DataAddress, DatasetRequest, Offer, PolicyDefinitionInput,
};
use edc_client::configuration::ApiKey;
use edc_client::{
    asset_api, catalog_api, contract_definition_api, contract_negotiation_api,
    policy_definition_api,
};
use odrl::name_spaces::{EDC_NS, LD_NS};
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

pub const PROVIDER_PROTOCOL: &str = "http://provider-connector:9194/protocol";
pub const PROVIDER_ID: &str = "provider";
pub const DATASPACE_PROTOCOL: &str = "dataspace-protocol-http";

pub fn setup_provider_configuration() -> Configuration {
    let mut provider = Configuration::default();
    provider.base_path = "http://localhost:29194/protocol".to_string();
    provider.with_headers()
}

pub fn setup_consumer_configuration() -> Configuration {
    let mut consumer = Configuration::default();
    consumer.base_path = "http://localhost:19194/protocol".to_string();
    consumer.with_headers()
}

pub async fn setup_management_provider() -> edc_client::configuration::Configuration {
    let mut management_provider = edc_client::configuration::Configuration::default();
    management_provider.base_path = "http://localhost:29193/management".to_owned();
    management_provider.api_key = Some(ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    management_provider.with_headers()
}

pub async fn setup_management_consumer() -> edc_client::configuration::Configuration {
    let mut management_consumer = edc_client::configuration::Configuration::default();
    management_consumer.base_path = "http://localhost:19193/management".to_owned();
    management_consumer.api_key = Some(ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    management_consumer.with_headers()
}

pub async fn setup_random_contract_definition(
    configuration: &edc_client::configuration::Configuration,
) -> (String, String, String) {
    // Create asset with random id
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
            operand_right: serde_json::Value::from(asset_response.clone().at_id.unwrap()),
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
        asset_response.clone().at_id.unwrap(),
        policy_response.clone().at_id.unwrap(),
        definition_response.clone().at_id.unwrap(),
    )
}

pub async fn setup_random_contract_negotiation(
    consumer: &edc_client::configuration::Configuration,
    provider: &edc_client::configuration::Configuration,
) -> (String, String) {
    let (asset_id, policy_id, _) = setup_random_contract_definition(&provider).await;

    let dataset_request = DatasetRequest {
        context: std::collections::HashMap::from([(
            "@vocab".to_string(),
            serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
        )]),
        at_type: Some("DatasetRequest".to_string()),
        at_id: Some(asset_id.clone()),
        counter_party_address: Some(PROVIDER_PROTOCOL.to_string()),
        counter_party_id: Some(PROVIDER_ID.to_string()),
        protocol: Some(DATASPACE_PROTOCOL.to_string()),
        query_spec: None,
    };

    let dataset = catalog_api::get_dataset(&consumer, Some(dataset_request))
        .await
        .unwrap();

    let offer_id = dataset
        .get("hasPolicy")
        .unwrap()
        .get("@id")
        .unwrap()
        .to_string()
        .replace("\"", "");

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
        connector_address: Some(PROVIDER_PROTOCOL.to_string()),
        counter_party_address: PROVIDER_PROTOCOL.to_string(),
        offer: Some(offer_description),
        policy: Some(offer),
        protocol: DATASPACE_PROTOCOL.to_string(),
        provider_id: Some(PROVIDER_ID.to_string()),
    };

    let response =
        contract_negotiation_api::initiate_contract_negotiation(&consumer, Some(contract_request))
            .await
            .unwrap();

    (response.clone().at_id.unwrap(), asset_id.clone())
}

pub async fn get_negotiation_state(
    conf: &Configuration,
    id: &str,
) -> Result<
    dsp_api::contract_negotiation::contract_negotiation::NegotiationState,
    Error<GetNegotiationError>,
> {
    let negotiation =
        dsp_client::contract_negotiation::negotiation_provider_api::get_negotiation(conf, id)
            .await?;
    let state = negotiation.state;
    Ok(state)
}

pub async fn wait_for_negotiation_state(
    conf: &Configuration,
    id: &str,
    state: dsp_api::contract_negotiation::contract_negotiation::NegotiationState,
) {
    wait_for(|| async {
        let i_state = state.clone();
        get_negotiation_state(conf, id)
            .await
            .map_err(|err| err.to_string())
            .and_then(|s| {
                if s == state {
                    Ok(i_state)
                } else {
                    Err(format!(
                        "State mismatch! Expected: {:?} Got: {:?}",
                        state.clone(),
                        s.clone()
                    ))
                }
            })
    })
    .await
    .unwrap();
}

pub async fn wait_for<F, Fut, R, E>(f: F) -> Result<R, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<R, E>>,
    E: std::fmt::Display,
{
    let timeout = tokio::time::timeout(Duration::from_secs(30), async move {
        loop {
            match f().await {
                Ok(r) => break Ok(r),
                Err(_) => {
                    sleep(Duration::from_millis(200)).await;
                }
            }
        }
    })
    .await;

    timeout.unwrap()
}
