extern crate edc_client;
extern crate odrl;

use edc_api::{
    AssetInput, CallbackAddress, ContractDefinitionInput, ContractNegotiation,
    ContractOfferDescription, ContractRequest, Criterion, DataAddress, DataPlaneInstanceSchema,
    DatasetRequest, NegotiationState, Offer, PolicyDefinitionInput, TransferRequest, TransferState,
};
use edc_client::configuration::{ApiKey, Configuration};
use edc_client::{
    asset_api, catalog_api, contract_agreement_api, contract_definition_api,
    contract_negotiation_api, dataplane_selector_api, policy_definition_api, transfer_process_api,
};
use serde::__private::de::IdentifierDeserializer;
use std::collections::HashSet;
use std::{future::Future, time::Duration};
use tokio::time::sleep;

use edc_client::contract_negotiation_api::get_negotiation_state;
use edc_client::policy_definition_api::get_policy_definition;
use edc_client::transfer_process_api::get_transfer_process_state;
use odrl::model::policy::OfferPolicy;
use odrl::model::rule::Permission;
use odrl::name_spaces::{EDC_NS, LD_NS, ODRL_NS};
use uuid::Uuid;

pub const PROVIDER_PROTOCOL: &str = "http://provider-connector:9194/protocol";
pub const PROVIDER_ID: &str = "provider";
pub const DATASPACE_PROTOCOL: &str = "dataspace-protocol-http";

pub fn setup_provider_configuration() -> Configuration {
    let mut provider = Configuration::default();
    provider.base_path = "http://localhost:29193/management".to_string();
    provider.api_key = Some(ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    provider.with_headers()
}

pub fn setup_consumer_configuration() -> Configuration {
    let mut consumer = Configuration::default();
    consumer.base_path = "http://localhost:19193/management".to_owned();
    consumer.api_key = Some(ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    consumer.with_headers()
}

pub async fn setup_random_contract_definition(
    configuration: &Configuration,
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
    consumer: &Configuration,
    provider: &Configuration,
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

pub async fn setup_random_contract_agreement(
    consumer: &Configuration,
    provider: &Configuration,
) -> (String, String, String) {
    let (contract_negotiation_id, asset_id) =
        setup_random_contract_negotiation(&consumer, &provider).await;

    wait_for_negotiation_state(
        &consumer,
        &contract_negotiation_id,
        NegotiationState {
            state: edc_api::ContractNegotiationState::Finalized,
        },
    )
    .await;

    let agreement_id =
        contract_negotiation_api::get_negotiation(&consumer, &contract_negotiation_id)
            .await
            .unwrap()
            .contract_agreement_id
            .unwrap();

    let agreement = contract_agreement_api::get_agreement_by_id(&consumer, &agreement_id)
        .await
        .unwrap();

    (
        agreement.clone().at_id.unwrap(),
        contract_negotiation_id,
        asset_id,
    )
}

pub async fn setup_random_transfer_process(
    consumer: &Configuration,
    provider: &Configuration,
) -> (String, String, String) {
    let (agreement_id, _, asset_id) = setup_random_contract_agreement(&consumer, &provider).await;

    let request = TransferRequest {
        context: std::collections::HashMap::from([(
            "@vocab".to_string(),
            serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
        )]),
        at_type: None,
        asset_id: asset_id.clone(),
        callback_addresses: Some(vec![CallbackAddress {
            at_type: None,
            auth_code_id: None,
            auth_key: None,
            events: Some(vec![
                "contract.negotiation".to_string(),
                "transfer.process".to_string(),
            ]),
            transactional: Some(false),
            uri: Some("http://localhost:80".to_string()),
        }]),
        connector_address: Some(PROVIDER_PROTOCOL.to_string()),
        connector_id: Some(PROVIDER_ID.to_string()),
        contract_id: agreement_id.clone(),
        counter_party_address: PROVIDER_PROTOCOL.to_string(),
        data_destination: Box::new(DataAddress {
            at_type: None,
            r#type: Some("HttpProxy".to_string()),
            base_url: Some(PROVIDER_PROTOCOL.to_string()),
        }),
        private_properties: None,
        protocol: "dataspace-protocol-http".to_string(),
        transfer_type: "HttpData-PULL".to_string(),
    };

    let transfer = transfer_process_api::initiate_transfer_process(&consumer, Some(request))
        .await
        .unwrap();

    (transfer.clone().at_id.unwrap(), agreement_id, asset_id)
}

pub async fn create_dataplane_for_transfer(conf: &Configuration, id: &str, url: &str) {
    let mut properties: std::collections::HashMap<String, serde_json::Value> = Default::default();

    if conf.base_path.contains("19193") {
        // consumer
        properties.insert(
            "publicApiUrl".to_string(),
            serde_json::Value::String("http://consumer-connector:9291/public".to_string()),
        );
    } else {
        // provider
        properties.insert(
            "publicApiUrl".to_string(),
            serde_json::Value::String("http://provider-connector:9291/public".to_string()),
        );
    }

    let dataplane = DataPlaneInstanceSchema {
        context: std::collections::HashMap::from([(
            "@vocab".to_string(),
            serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
        )]),
        at_id: Some(id.to_string()),
        at_type: None,
        allowed_dest_types: vec!["HttpProxy".to_string(), "DataAddress".to_string()],
        allowed_source_types: vec!["HttpData".to_string(), "DataAddress".to_string()],
        allowed_transfer_types: Some(vec![
            "HttpData-PULL".to_string(),
            "HttpData-PUSH".to_string(),
        ]),
        last_active: None,
        turn_count: None,
        url: url.to_string(),
        properties: Some(properties),
    };

    dataplane_selector_api::add_entry(conf, Some(dataplane.clone()))
        .await
        .unwrap();
}

pub async fn wait_for_negotiation_state(conf: &Configuration, id: &str, state: NegotiationState) {
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
                        state.clone().state,
                        s.clone().state
                    ))
                }
            })
    })
    .await
    .unwrap();
}

pub async fn wait_for_transfer_state(conf: &Configuration, id: &str, state: TransferState) {
    let test = wait_for(|| async {
        let i_state = state.clone();
        get_transfer_process_state(conf, id)
            .await
            .map_err(|err| err.to_string())
            .and_then(|s| {
                if s == state {
                    Ok(i_state)
                } else {
                    Err(format!(
                        "State mismatch! Expected: {:?} Got: {:?}",
                        state.clone().state,
                        s.clone().state
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
