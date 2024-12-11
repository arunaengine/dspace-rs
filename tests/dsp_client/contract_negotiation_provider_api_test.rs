mod common;

#[cfg(test)]
mod contract_negotiation_provider_side_api_test {

    use std::collections::HashMap;

    use crate::common::{
        setup_management_consumer, setup_management_provider, setup_provider_configuration,
        setup_random_contract_negotiation, wait_for_negotiation_state, DATASPACE_PROTOCOL,
        PROVIDER_ID, PROVIDER_PROTOCOL,
    };

    use dsp_api::contract_negotiation::{AbstractPolicyRule, MessageOffer, PolicyClass};
    use dsp_client::Error;
    use edc_api::DatasetRequest;
    use edc_client::catalog_api;

    #[tokio::test]
    async fn test_get_unknown_negotiation() {
        let provider = setup_provider_configuration();
        let id = "1234";
        let response = dsp_client::contract_negotiation::negotiation_provider_api::get_negotiation(
            &provider, id,
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because no negotiation with that ID exists"),
        }
    }

    #[tokio::test]
    async fn test_request_negotiation() {
        let management_provider = setup_management_provider().await;
        let management_consumer = setup_management_consumer().await;
        let provider = setup_provider_configuration();

        let (negotiation_id, asset_id) =
            setup_random_contract_negotiation(&management_consumer, &management_provider).await;

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
                    target: dsp_api::contract_negotiation::Target::new(asset_id.clone()),
                },
                odrl_type: "odrl:Offer".to_string(),
            },
            callback_address: "http://consumer-connector:9194/protocol".to_string(),
        };

        let response =
            dsp_client::contract_negotiation::negotiation_provider_api::request_negotiation(
                &provider,
                request_message,
            )
            .await;

        assert!(response.is_ok());

        match response {
            Ok(negotiation) => {
                assert_eq!(negotiation.state, dsp_api::contract_negotiation::contract_negotiation::NegotiationState::REQUESTED);
                assert!(negotiation.provider_pid.len() > 0);
                assert!(negotiation.consumer_pid.len() > 0);
            }
            _ => panic!("Expected a ContractNegotiation"),
        }
    }

    #[tokio::test]
    async fn test_verify_agreement() {
        let management_provider = setup_management_provider().await;
        let management_consumer = setup_management_consumer().await;
        let provider = setup_provider_configuration();

        let (negotiation_id, asset_id) =
            setup_random_contract_negotiation(&management_consumer, &management_provider).await;

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
                    target: dsp_api::contract_negotiation::Target::new(asset_id.clone()),
                },
                odrl_type: "odrl:Offer".to_string(),
            },
            callback_address: "http://consumer-connector:9194/protocol".to_string(),
        };

        let response =
            dsp_client::contract_negotiation::negotiation_provider_api::request_negotiation(
                &provider,
                request_message,
            )
            .await;

        assert!(response.is_ok());

        match response {
            Ok(negotiation) => {
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

                assert!(response.is_ok());
            }
            _ => panic!("Expected a ContractNegotiation"),
        }
    }

    #[tokio::test]
    async fn test_terminate_negotiation() {
        let management_provider = setup_management_provider().await;
        let management_consumer = setup_management_consumer().await;
        let provider = setup_provider_configuration();

        let (negotiation_id, asset_id) =
            setup_random_contract_negotiation(&management_consumer, &management_provider).await;

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
                    target: dsp_api::contract_negotiation::Target::new(asset_id.clone()),
                },
                odrl_type: "odrl:Offer".to_string(),
            },
            callback_address: "http://consumer-connector:9194/protocol".to_string(),
        };

        let response =
            dsp_client::contract_negotiation::negotiation_provider_api::request_negotiation(
                &provider,
                request_message,
            )
            .await;

        assert!(response.is_ok());

        match response {
            Ok(negotiation) => {
                let mut context: HashMap<String, serde_json::Value> =
                    std::collections::HashMap::from([(
                        "dspace".to_string(),
                        serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()),
                    )]);
                context.insert(
                    "odrl".to_string(),
                    serde_json::Value::String("http://www.w3.org/ns/odrl/2/".to_string()),
                );

                let termination_message =
                    dsp_api::contract_negotiation::ContractNegotiationTerminationMessage {
                        context,
                        dsp_type: "dspace:ContractNegotiationTerminationMessage".to_string(),
                        provider_pid: negotiation.provider_pid.clone(),
                        consumer_pid: negotiation.consumer_pid.clone(),
                        code: None,
                        reason: vec!["Terminating for testing purposes".to_string()],
                    };

                let response = dsp_client::contract_negotiation::negotiation_provider_api::terminate_negotiation(&provider, termination_message, negotiation.provider_pid.clone().as_str()).await;

                assert!(response.is_ok());
            }
            _ => panic!("Expected a ContractNegotiation"),
        }
    }

    #[tokio::test]
    async fn test_malformed_termination_message() {
        let management_provider = setup_management_provider().await;
        let management_consumer = setup_management_consumer().await;
        let provider = setup_provider_configuration();

        let (negotiation_id, asset_id) =
            setup_random_contract_negotiation(&management_consumer, &management_provider).await;

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
                    target: dsp_api::contract_negotiation::Target::new(asset_id.clone()),
                },
                odrl_type: "odrl:Offer".to_string(),
            },
            callback_address: "http://consumer-connector:9194/protocol".to_string(),
        };

        let response =
            dsp_client::contract_negotiation::negotiation_provider_api::request_negotiation(
                &provider,
                request_message,
            )
            .await;

        assert!(response.is_ok());

        match response {
            Ok(negotiation) => {
                let mut context: HashMap<String, serde_json::Value> =
                    std::collections::HashMap::from([(
                        "dspace".to_string(),
                        serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()),
                    )]);
                context.insert(
                    "odrl".to_string(),
                    serde_json::Value::String("http://www.w3.org/ns/odrl/2/".to_string()),
                );

                let termination_message =
                    dsp_api::contract_negotiation::ContractNegotiationTerminationMessage {
                        context,
                        dsp_type: "dspace:ContractNegotiationTerminationMessage".to_string(),
                        provider_pid: "".to_string(),
                        consumer_pid: "".to_string(),
                        code: None,
                        reason: vec!["Terminating for testing purposes".to_string()],
                    };

                let response = dsp_client::contract_negotiation::negotiation_provider_api::terminate_negotiation(&provider, termination_message, negotiation.provider_pid.clone().as_str()).await;

                assert!(response.is_err());
                match response {
                    Err(Error::ResponseError(response)) => {
                        assert_eq!(response.status, reqwest::StatusCode::BAD_REQUEST);
                    },
                    _ => panic!("Expected Status Code 400, because the termination message was malformed (provider pid and consumer pid are empty)"),
                }
            }
            _ => panic!("Expected a ContractNegotiation"),
        }
    }
}
