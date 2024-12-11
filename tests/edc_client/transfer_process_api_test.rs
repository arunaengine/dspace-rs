mod common;

#[cfg(test)]
mod transfer_process_api_initiate_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration,
        setup_random_contract_agreement, PROVIDER_PROTOCOL,
    };
    use edc_api::{DataAddress, TransferRequest};
    use edc_client::transfer_process_api;

    #[tokio::test]
    async fn test_transfer_process_initiate() {
        let provider_configuration = setup_provider_configuration();
        let consumer_configuration = setup_consumer_configuration();

        let (agreement_id, _, asset_id) =
            setup_random_contract_agreement(&consumer_configuration, &provider_configuration).await;

        let request = TransferRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: None,
            asset_id: asset_id.clone(),
            callback_addresses: None,
            connector_address: None,
            connector_id: None,
            contract_id: agreement_id.clone(),
            counter_party_address: PROVIDER_PROTOCOL.to_string(),
            data_destination: Box::new(DataAddress {
                at_type: Some("HttpProxy".to_string()),
                r#type: Some("HttpProxy".to_string()),
                base_url: None,
            }),
            private_properties: None,
            protocol: "dataspace-protocol-http".to_string(),
            transfer_type: "HttpData-PULL".to_string(),
        };

        let response =
            transfer_process_api::initiate_transfer_process(&consumer_configuration, Some(request))
                .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_transfer_with_wrong_contract() {
        let consumer_configuration = setup_consumer_configuration();

        let request = TransferRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: None,
            asset_id: "unknown-asset-id".to_string(),
            callback_addresses: None,
            connector_address: None,
            connector_id: None,
            contract_id: "unknown-contract-id".to_string(),
            counter_party_address: PROVIDER_PROTOCOL.to_string(),
            data_destination: Box::new(DataAddress {
                at_type: Some("HttpProxy".to_string()),
                r#type: Some("HttpProxy".to_string()),
                base_url: None,
            }),
            private_properties: None,
            protocol: "dataspace-protocol-http".to_string(),
            transfer_type: "HttpData-PULL".to_string(),
        };

        let response =
            transfer_process_api::initiate_transfer_process(&consumer_configuration, Some(request))
                .await;

        match response {
            Err(edc_client::Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::BAD_REQUEST);
            },
            _ => panic!("Expected Status Code 400, because the contract does not exist and therefore the request is invalid"),
        }
    }
}

#[cfg(test)]
mod transfer_process_api_request_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration, setup_random_transfer_process,
    };
    use edc_api::{Criterion, QuerySpec};
    use edc_client::transfer_process_api;
    use odrl::name_spaces::EDC_NS;

    #[tokio::test]
    async fn test_transfer_process_request() {
        let provider_configuration = setup_provider_configuration();
        let consumer_configuration = setup_consumer_configuration();

        let (first_transfer_id, _, _) =
            setup_random_transfer_process(&consumer_configuration, &provider_configuration).await;
        let (second_transfer_id, _, _) =
            setup_random_transfer_process(&consumer_configuration, &provider_configuration).await;

        assert_ne!(first_transfer_id, second_transfer_id);

        let criterion = Criterion {
            at_type: None,
            operand_left: serde_json::Value::from("id"),
            operator: "=".to_string(),
            operand_right: serde_json::Value::from(first_transfer_id.clone()),
        };

        let query = QuerySpec {
            at_context: Some(std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )])),
            at_type: Some("QuerySpec".to_string()),
            filter_expression: vec![criterion],
            limit: None,
            offset: None,
            sort_field: None,
            sort_order: None,
        };

        let response =
            transfer_process_api::query_transfer_processes(&consumer_configuration, Some(query))
                .await
                .unwrap();

        assert_eq!(response.len(), 1);
        assert_eq!(response[0].at_id.as_ref().unwrap(), &first_transfer_id);
    }
}

#[cfg(test)]
mod transfer_process_api_get_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration, setup_random_transfer_process,
    };
    use edc_client::{transfer_process_api, Error};

    #[tokio::test]
    async fn test_transfer_process_get() {
        let provider_configuration = setup_provider_configuration();
        let consumer_configuration = setup_consumer_configuration();

        let (transfer_id, _, _) =
            setup_random_transfer_process(&consumer_configuration, &provider_configuration).await;

        let response =
            transfer_process_api::get_transfer_process(&consumer_configuration, &transfer_id)
                .await
                .unwrap();

        assert_eq!(response.at_id.as_ref().unwrap(), &transfer_id);
    }

    #[tokio::test]
    async fn test_get_state_of_transfer_process() {
        let provider_configuration = setup_provider_configuration();
        let consumer_configuration = setup_consumer_configuration();

        let (transfer_id, _, _) =
            setup_random_transfer_process(&consumer_configuration, &provider_configuration).await;

        let response =
            transfer_process_api::get_transfer_process_state(&consumer_configuration, &transfer_id)
                .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_transfer_process_get_with_unknown_id() {
        let consumer_configuration = setup_consumer_configuration();

        let response =
            transfer_process_api::get_transfer_process(&consumer_configuration, "unknown-id").await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            },
            _ => panic!("Expected Status Code 404, because a transfer process with the given id does not exist"),
        }
    }
}

#[cfg(test)]
mod transfer_process_api_deprovision_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::setup_consumer_configuration;
    use edc_client::{transfer_process_api, Error};

    #[tokio::test]
    async fn test_transfer_process_deprovision_unknown_id() {
        let consumer_configuration = setup_consumer_configuration();

        let response = transfer_process_api::deprovision_transfer_process(
            &consumer_configuration,
            "unknown-id",
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            },
            _ => panic!("Expected Status Code 404, because a transfer process with the given id does not exist"),
        }
    }
}

#[cfg(test)]
mod transfer_process_api_resume_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        create_dataplane_for_transfer, setup_consumer_configuration, setup_provider_configuration,
        setup_random_contract_agreement, setup_random_transfer_process, wait_for_transfer_state,
        PROVIDER_ID, PROVIDER_PROTOCOL,
    };
    use edc_api::transfer_state::TransferProcessState;
    use edc_api::{
        CallbackAddress, DataAddress, SuspendTransfer, TerminateTransfer, TransferRequest,
        TransferState,
    };
    use edc_client::{transfer_process_api, Error};
    use odrl::name_spaces::EDC_NS;

    #[tokio::test]
    async fn test_transfer_process_pause_and_resume() {
        let provider_configuration = setup_provider_configuration();
        let consumer_configuration = setup_consumer_configuration();

        create_dataplane_for_transfer(
            &provider_configuration,
            "dataplane",
            "http://provider-connector:9192/control/transfer",
        )
        .await;

        let (agreement_id, _, asset_id) =
            setup_random_contract_agreement(&consumer_configuration, &provider_configuration).await;

        let request = TransferRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
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
                uri: Some("http://provider-connector:9194/protocol".to_string()),
            }]),
            connector_address: Some(PROVIDER_PROTOCOL.to_string()),
            connector_id: None,
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

        let transfer =
            transfer_process_api::initiate_transfer_process(&consumer_configuration, Some(request))
                .await
                .unwrap();
        let transfer_id = transfer.at_id.clone().unwrap();

        wait_for_transfer_state(
            &consumer_configuration,
            &transfer_id.clone(),
            TransferState {
                state: TransferProcessState::Started,
            },
        )
        .await;

        let suspend_transfer = SuspendTransfer {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_type: Some("SuspendTransfer".to_string()),
            reason: Some("Suspending for testing purposes".to_string()),
        };

        let response = transfer_process_api::suspend_transfer_process(
            &consumer_configuration,
            &transfer_id.clone(),
            Some(suspend_transfer),
        )
        .await;

        assert!(response.is_ok());

        wait_for_transfer_state(
            &consumer_configuration,
            &transfer_id.clone(),
            TransferState {
                state: TransferProcessState::Suspended,
            },
        )
        .await;

        let response = transfer_process_api::resume_transfer_process(
            &consumer_configuration,
            &transfer_id.clone(),
        )
        .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_suspend_already_terminated_transfer_process() {
        let provider_configuration = setup_provider_configuration();
        let consumer_configuration = setup_consumer_configuration();

        create_dataplane_for_transfer(
            &provider_configuration,
            "dataplane",
            "http://provider-connector:9192/control/transfer",
        )
        .await;

        let (transfer_id, _, _) =
            setup_random_transfer_process(&consumer_configuration, &provider_configuration).await;

        wait_for_transfer_state(
            &consumer_configuration,
            &transfer_id.clone(),
            TransferState {
                state: TransferProcessState::Started,
            },
        )
        .await;

        let terminate_transfer = TerminateTransfer {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("TerminateTransfer".to_string()),
            reason: Some("Terminating for testing purposes".to_string()),
            provider_id: Some(PROVIDER_ID.to_string()),
            transfer_id: Some(transfer_id.clone()),
        };

        let response = transfer_process_api::terminate_transfer_process(
            &consumer_configuration,
            &transfer_id,
            Some(terminate_transfer),
        )
        .await;

        wait_for_transfer_state(
            &consumer_configuration,
            &transfer_id.clone(),
            TransferState {
                state: TransferProcessState::Terminated,
            },
        )
        .await;

        assert!(response.is_ok());

        let suspend_transfer = SuspendTransfer {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_type: Some("SuspendTransfer".to_string()),
            reason: Some("Suspending for testing purposes should fail".to_string()),
        };

        let response = transfer_process_api::suspend_transfer_process(
            &consumer_configuration,
            &transfer_id.clone(),
            Some(suspend_transfer),
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::CONFLICT);
            },
            _ => panic!("Expected Status Code 409, because a transfer process with the given id is already terminated"),
        }
    }

    #[tokio::test]
    async fn test_suspend_transfer_with_unknown_id() {
        let consumer_configuration = setup_consumer_configuration();

        let suspend_transfer = SuspendTransfer {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_type: Some("SuspendTransfer".to_string()),
            reason: Some("Suspending for testing purposes".to_string()),
        };

        let response = transfer_process_api::suspend_transfer_process(
            &consumer_configuration,
            "unknown-id",
            Some(suspend_transfer),
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            },
            _ => panic!("Expected Status Code 404, because a transfer process with the given id does not exist"),
        }
    }
}

#[cfg(test)]
mod transfer_process_api_terminate_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        create_dataplane_for_transfer, setup_consumer_configuration, setup_provider_configuration,
        setup_random_transfer_process, wait_for_transfer_state, PROVIDER_ID,
    };
    use edc_api::transfer_state::TransferProcessState;
    use edc_api::{TerminateTransfer, TransferState};
    use edc_client::{transfer_process_api, Error};

    #[tokio::test]
    async fn test_terminate_transfer_process() {
        let provider_configuration = setup_provider_configuration();
        let consumer_configuration = setup_consumer_configuration();

        create_dataplane_for_transfer(
            &provider_configuration,
            "dataplane",
            "http://provider-connector:9192/control/transfer",
        )
        .await;

        let (transfer_id, _, _) =
            setup_random_transfer_process(&consumer_configuration, &provider_configuration).await;

        wait_for_transfer_state(
            &consumer_configuration,
            &transfer_id.clone(),
            TransferState {
                state: TransferProcessState::Started,
            },
        )
        .await;

        let terminate_transfer = TerminateTransfer {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("TerminateTransfer".to_string()),
            reason: Some("Terminating for testing purposes".to_string()),
            provider_id: Some(PROVIDER_ID.to_string()),
            transfer_id: Some(transfer_id.clone()),
        };

        let response = transfer_process_api::terminate_transfer_process(
            &consumer_configuration,
            &transfer_id,
            Some(terminate_transfer),
        )
        .await;

        wait_for_transfer_state(
            &consumer_configuration,
            &transfer_id.clone(),
            TransferState {
                state: TransferProcessState::Terminated,
            },
        )
        .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_terminate_transfer_with_unknown_id() {
        let consumer_configuration = setup_consumer_configuration();

        let terminate_transfer = TerminateTransfer {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("TerminateTransfer".to_string()),
            reason: Some("Terminating for testing purposes".to_string()),
            provider_id: Some(PROVIDER_ID.to_string()),
            transfer_id: Some("unknown-id".to_string()),
        };

        let response = transfer_process_api::terminate_transfer_process(
            &consumer_configuration,
            "unknown-id",
            Some(terminate_transfer),
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            },
            _ => panic!("Expected Status Code 404, because a transfer process with the given id does not exist"),
        }
    }

    #[tokio::test]
    async fn test_terminate_already_terminated_transfer() {
        let provider_configuration = setup_provider_configuration();
        let consumer_configuration = setup_consumer_configuration();

        let (transfer_id, _, _) =
            setup_random_transfer_process(&consumer_configuration, &provider_configuration).await;

        wait_for_transfer_state(
            &consumer_configuration,
            &transfer_id.clone(),
            TransferState {
                state: TransferProcessState::Started,
            },
        )
        .await;

        let terminate_transfer = TerminateTransfer {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("TerminateTransfer".to_string()),
            reason: Some("Terminating for testing purposes".to_string()),
            provider_id: Some(PROVIDER_ID.to_string()),
            transfer_id: Some(transfer_id.clone()),
        };

        let response = transfer_process_api::terminate_transfer_process(
            &consumer_configuration,
            &transfer_id,
            Some(terminate_transfer.clone()),
        )
        .await;

        wait_for_transfer_state(
            &consumer_configuration,
            &transfer_id.clone(),
            TransferState {
                state: TransferProcessState::Terminated,
            },
        )
        .await;

        assert!(response.is_ok());

        let response = transfer_process_api::terminate_transfer_process(
            &consumer_configuration,
            &transfer_id,
            Some(terminate_transfer.clone()),
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::CONFLICT);
            }
            _ => panic!(
                "Expected Status Code 409, because the transfer process is already terminated"
            ),
        }
    }
}
