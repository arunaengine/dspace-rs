extern crate dsp_api;

use dsp_api::contract_negotiation::{AbstractPolicyRule, MessageOffer, PolicyClass, Target};
use dsp_api::transfer_process::TransferRequestMessage;
use dsp_client::configuration::Configuration;
use dsp_client::contract_negotiation::negotiation_provider_api::GetNegotiationError;
use dsp_client::Error;
use std::collections::HashMap;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

extern crate edc_api;
use edc_api::{
    AssetInput, ContractDefinitionInput, ContractOfferDescription, ContractRequest, Criterion,
    DataAddress, DatasetRequest, NegotiationState, Offer, PolicyDefinitionInput,
};

extern crate edc_client;
use edc_client::configuration::ApiKey;
use edc_client::{
    asset_api, catalog_api, contract_agreement_api, contract_definition_api,
    contract_negotiation_api, policy_definition_api,
};
use odrl::name_spaces::{EDC_NS, LD_NS};

pub const PROVIDER_PROTOCOL: &str = "http://provider-connector:9194/protocol";
pub const PROVIDER_ID: &str = "provider";
pub const PROVIDER_DSP_HOST: &str = "provider-connector:9194";
pub const CONSUMER_ID: &str = "consumer";
pub const CONSUMER_DSP_HOST: &str = "consumer-connector:9194";
pub const DATASPACE_PROTOCOL: &str = "dataspace-protocol-http";

fn setup_consumer_conf() -> Configuration {
    let mut client = Configuration::default();
    client.base_path = "http://localhost:19194/protocol".to_string();
    client.with_headers()
}

fn setup_provider_conf() -> Configuration {
    let mut provider = Configuration::default();
    provider.base_path = "http://localhost:29194/protocol".to_string();
    provider.with_headers()
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

    println!(
        "**** GIST ASSET: {}",
        serde_json::to_string_pretty(&asset).unwrap()
    );

    let asset_response = asset_api::create_asset(&configuration, Some(asset))
        .await
        .unwrap();

    let test_policy = r#"
    {
        "@context": "http://www.w3.org/ns/odrl.jsonld",
        "@type": "Set",
        "uid": "api_test_policy",
        "permission": [
        ]
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

    println!(
        "**** GIST POLICY DEFINITION: {}",
        serde_json::to_string_pretty(&policy_definition).unwrap()
    );

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

    let pol = policy_definition_api::get_policy_definition(&provider, &policy_id)
        .await
        .unwrap();
    println!("Policy: {}", serde_json::to_string_pretty(&pol).unwrap());

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

    println!(
        "Dataset: {}",
        serde_json::to_string_pretty(&dataset).unwrap()
    );

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
        "permission": [
            {
            "action": "use"
            }
        ]
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

    println!(
        "**** GIST CONTRACT REQUEST: {}",
        serde_json::to_string_pretty(&contract_request).unwrap()
    );

    let response =
        contract_negotiation_api::initiate_contract_negotiation(&consumer, Some(contract_request))
            .await
            .unwrap();

    (response.clone().at_id.unwrap(), asset_id.clone())
}

pub async fn setup_random_contract_agreement(
    consumer: &edc_client::configuration::Configuration,
    provider: &edc_client::configuration::Configuration,
) -> (String, String, String) {
    let (contract_negotiation_id, asset_id) =
        setup_random_contract_negotiation(&consumer, &provider).await;

    let neg = contract_negotiation_api::get_negotiation(&consumer, &contract_negotiation_id)
        .await
        .unwrap();
    println!(
        "Negotiation: {}",
        serde_json::to_string_pretty(&neg).unwrap()
    );

    wait_for_management_negotiation_state(
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

pub async fn wait_for_management_negotiation_state(
    conf: &edc_client::configuration::Configuration,
    id: &str,
    state: edc_api::contract_negotiation::NegotiationState,
) {
    wait_for(|| async {
        let i_state = state.clone();
        edc_client::contract_negotiation_api::get_negotiation_state(conf, id)
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

#[tokio::main]
pub async fn main() {
    let provider = setup_provider_conf();

    let management_provider = setup_management_provider().await;
    let management_consumer = setup_management_consumer().await;

    let (agreement_id, negotiation_id, asset_id) =
        setup_random_contract_agreement(&management_consumer, &management_provider).await;

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

    let dataset = catalog_api::get_dataset(&management_consumer, Some(dataset_request))
        .await
        .unwrap();

    let offer_id = dataset
        .get("hasPolicy")
        .unwrap()
        .get("@id")
        .unwrap()
        .to_string()
        .replace("\"", "");

    let permission = dsp_api::contract_negotiation::Permission {
        abstract_policy_rule: AbstractPolicyRule {
            assigner: None,
            assignee: None,
        },
        action: dsp_api::contract_negotiation::Action::Use,
        constraint: vec![],
        duty: None,
    };

    let mut context: HashMap<String, serde_json::Value> = std::collections::HashMap::from([(
        "dspace".to_string(),
        serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()),
    )]);
    context.insert(
        "odrl".to_string(),
        serde_json::Value::String("http://www.w3.org/ns/odrl/2/".to_string()),
    );

    let request_message = dsp_api::contract_negotiation::ContractRequestMessage {
        context,
        dsp_type: "dspace:ContractRequestMessage".to_string(),
        provider_pid: None,
        consumer_pid: negotiation_id.clone(),
        offer: MessageOffer {
            policy_class: PolicyClass {
                abstract_policy_rule: AbstractPolicyRule {
                    assigner: Some(PROVIDER_ID.to_string()),
                    assignee: None,
                },
                id: offer_id.clone(),
                profile: vec![],
                permission: vec![permission],
                obligation: vec![],
                target: Target::new(asset_id.clone()),
            },
            odrl_type: "odrl:Offer".to_string(),
        },
        callback_address: "http://consumer-connector:9194/protocol".to_string(),
    };

    println!(
        "request_message: {}",
        serde_json::to_string_pretty(&request_message).unwrap()
    );
    println!(
        "**** GIST CONTRACT REQUEST MESSAGE: {}",
        serde_json::to_string_pretty(&request_message).unwrap()
    );

    let response = dsp_client::contract_negotiation::negotiation_provider_api::request_negotiation(
        &provider,
        request_message,
    )
    .await;

    match response {
        Ok(negotiation) => {
            println!("Negotiation requested: {:#?}", negotiation);

            let mut context: HashMap<String, serde_json::Value> =
                std::collections::HashMap::from([(
                    "dspace".to_string(),
                    serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()),
                )]);
            context.insert(
                "odrl".to_string(),
                serde_json::Value::String("http://www.w3.org/ns/odrl/2/".to_string()),
            );

            let verification_message =
                dsp_api::contract_negotiation::ContractAgreementVerificationMessage {
                    context,
                    dsp_type: "dspace:ContractAgreementVerificationMessage".to_string(),
                    provider_pid: negotiation.provider_pid.clone(),
                    consumer_pid: negotiation.consumer_pid.clone(),
                };

            println!(
                "**** GIST CONTRACT AGREEMENT VERIFICATION MESSAGE: {}",
                serde_json::to_string_pretty(&verification_message).unwrap()
            );

            wait_for_negotiation_state(
                &provider,
                &negotiation.provider_pid.clone(),
                dsp_api::contract_negotiation::contract_negotiation::NegotiationState::AGREED,
            )
            .await;

            let response =
                dsp_client::contract_negotiation::negotiation_provider_api::verify_agreement(
                    &provider,
                    verification_message,
                    negotiation.provider_pid.clone().as_str(),
                )
                .await;

            if let Ok(_) = response {
                wait_for_negotiation_state(&provider, &negotiation.provider_pid.clone(), dsp_api::contract_negotiation::contract_negotiation::NegotiationState::FINALIZED).await;

                let mut context: HashMap<String, serde_json::Value> =
                    std::collections::HashMap::from([(
                        "dspace".to_string(),
                        serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()),
                    )]);
                context.insert(
                    "odrl".to_string(),
                    serde_json::Value::String("http://www.w3.org/ns/odrl/2/".to_string()),
                );
                context.insert(
                    "dct".to_string(),
                    serde_json::Value::String("http://purl.org/dc/terms/".to_string()),
                );
                context.insert(
                    "@vocab".to_string(),
                    serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
                );
                context.insert(
                    "edc".to_string(),
                    serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
                );
                context.insert(
                    "dcat".to_string(),
                    serde_json::Value::String("https://www.w3.org/ns/dcat#".to_string()),
                );

                let mut nego =
                    dsp_client::contract_negotiation::negotiation_provider_api::get_negotiation(
                        &provider,
                        &negotiation.provider_pid.clone(),
                    )
                    .await
                    .unwrap();
                println!(
                    "Negotiation retrieved: {}",
                    serde_json::to_string_pretty(&nego).unwrap()
                );
                wait_for_management_negotiation_state(
                    &management_consumer,
                    negotiation_id.clone().as_str(),
                    edc_api::contract_negotiation::NegotiationState {
                        state: edc_api::ContractNegotiationState::Finalized,
                    },
                )
                .await;

                let agreement = contract_agreement_api::get_agreement_by_id(
                    &management_consumer,
                    &agreement_id,
                )
                .await
                .unwrap();
                println!(
                    "Agreement retrieved: {}",
                    serde_json::to_string_pretty(&agreement).unwrap()
                );

                let consumer_tp_id = Uuid::new_v4().to_string();

                let transfer_request_message = TransferRequestMessage {
                    context,
                    // dsp_type: "dspace:TransferRequestMessage".to_string(),
                    dsp_type: "https://w3id.org/dspace/v0.8/TransferRequestMessage".to_string(),
                    agreement_id: agreement_id.clone(),
                    dct_format: "HttpData-PULL".to_string(),
                    data_address: Some(dsp_api::transfer_process::DataAddress {
                        at_type: "dspace:DataAddress".to_string(),
                        endpoint_type: "https://w3id.org/idsa/v4.1/HTTP".to_string(),
                        endpoint: "http://host.docker.internal:19999".to_string(),
                        endpoint_properties: None,
                    }),
                    callback_address: "http://consumer-connector:9194/protocol".to_string(),
                    consumer_pid: consumer_tp_id.clone(), //negotiation.consumer_pid.clone(),
                };

                println!(
                    "transfer_request_message: {}",
                    serde_json::to_string_pretty(&transfer_request_message).unwrap()
                );

                let transfer_response =
                    dsp_client::transfer_process::transfer_provider_api::request_transfer(
                        &provider,
                        transfer_request_message,
                    )
                    .await;

                match transfer_response {
                    Ok(transfer) => {
                        println!("Transfer requested: {:#?}", transfer);

                        let id = transfer.provider_pid.clone();

                        let tp =
                            dsp_client::transfer_process::transfer_provider_api::get_negotiation(
                                &provider, &id,
                            )
                            .await;

                        match tp {
                            Ok(process) => {
                                println!(
                                    "Transfer retrieved: {}",
                                    serde_json::to_string_pretty(&process).unwrap()
                                );
                            }
                            Err(error) => {
                                println!("Error: {:#?}", error);
                            }
                        }
                    }
                    Err(error) => {
                        println!("Error: {:#?}", error);
                    }
                }
            } else {
                println!("Error verifying agreement");
            }
        }
        Err(error) => {
            println!("Error: {:#?}", error);
        }
    }
}
