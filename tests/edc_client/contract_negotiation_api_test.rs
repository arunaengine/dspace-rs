mod common;

#[cfg(test)]
mod contract_negotiation_api_initiate_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration,
        setup_random_contract_definition, DATASPACE_PROTOCOL, PROVIDER_ID, PROVIDER_PROTOCOL,
    };
    use edc_api::{ContractOfferDescription, ContractRequest, DatasetRequest, Offer};
    use edc_client::{catalog_api, contract_negotiation_api};
    use odrl::name_spaces::LD_NS;

    #[tokio::test]
    async fn test_initiate_contract_definition() {
        let provider = setup_provider_configuration();
        let consumer = setup_consumer_configuration();

        let (asset_id, policy_id, _) = setup_random_contract_definition(&provider).await;

        let dataset_request = DatasetRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("DatasetRequest".to_string()),
            at_id: Some(asset_id.clone()),
            counter_party_address: Some(PROVIDER_PROTOCOL.to_string()),
            counter_party_id: None,
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
            connector_address: None,
            counter_party_address: PROVIDER_PROTOCOL.to_string(),
            offer: Some(offer_description),
            policy: Some(offer.clone()),
            protocol: DATASPACE_PROTOCOL.to_string(),
            provider_id: None,
        };

        let response = contract_negotiation_api::initiate_contract_negotiation(
            &consumer,
            Some(contract_request),
        )
        .await;

        assert!(response.is_ok());
    }
}

#[cfg(test)]
mod contract_negotiation_api_request_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration,
        setup_random_contract_negotiation,
    };
    use edc_api::{Criterion, QuerySpec};
    use edc_client::contract_negotiation_api;
    use odrl::name_spaces::EDC_NS;

    #[tokio::test]
    async fn test_request_contract_negotiations() {
        let provider = setup_provider_configuration();
        let consumer = setup_consumer_configuration();

        let (first_negotiation_id, first_asset_id) =
            setup_random_contract_negotiation(&consumer, &provider).await;
        let (second_negotiation_id, second_asset_it) =
            setup_random_contract_negotiation(&consumer, &provider).await;

        let criterion = Criterion {
            at_type: None,
            operand_left: serde_json::Value::from("id"),
            operator: "=".to_string(),
            operand_right: serde_json::Value::from(first_negotiation_id.clone()),
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

        let negotiations = contract_negotiation_api::query_negotiations(&consumer, Some(query))
            .await
            .unwrap();

        assert_eq!(1, negotiations.len());

        let pulled_negotiation_id = negotiations.first().unwrap().at_id.clone().unwrap();
        assert_eq!(first_negotiation_id, pulled_negotiation_id);
    }
}

#[cfg(test)]
mod contract_negotiation_api_get_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration,
        setup_random_contract_negotiation,
    };
    use edc_client::{contract_negotiation_api, Error};

    #[tokio::test]
    async fn test_get_negotiation_by_id() {
        let provider = setup_provider_configuration();
        let consumer = setup_consumer_configuration();

        let (negotiation_id, asset_id) =
            setup_random_contract_negotiation(&consumer, &provider).await;

        let negotiation =
            contract_negotiation_api::get_negotiation(&consumer, &negotiation_id.clone()).await;

        assert!(negotiation.is_ok());
    }

    #[tokio::test]
    async fn test_get_negotiation_with_unknown_id() {
        let consumer = setup_consumer_configuration();

        let id = "unknown_id";

        let response = contract_negotiation_api::get_negotiation(&consumer, &id).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => {
                panic!("Expected Status Code 404, because the contract negotiation does not exist")
            }
        }
    }
}

#[cfg(test)]
mod contract_negotiation_api_agreement_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration, setup_random_contract_agreement,
    };
    use edc_client::{contract_negotiation_api, Error};

    #[tokio::test]
    async fn test_get_agreement_by_id() {
        let provider = setup_provider_configuration();
        let consumer = setup_consumer_configuration();

        let (agreement_id, negotiation_id, asset_id) =
            setup_random_contract_agreement(&consumer, &provider).await;

        let agreement = contract_negotiation_api::get_agreement_for_negotiation(
            &consumer,
            &negotiation_id.clone(),
        )
        .await
        .unwrap();

        assert_eq!(agreement_id, agreement.at_id.unwrap());
    }

    #[tokio::test]
    async fn test_get_agreement_with_unknown_id() {
        let consumer = setup_consumer_configuration();

        let id = "unknown_id";

        let response =
            contract_negotiation_api::get_agreement_for_negotiation(&consumer, &id).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => {
                panic!("Expected Status Code 404, because the contract negotiation does not exist")
            }
        }
    }
}

#[cfg(test)]
mod contract_negotiation_api_state_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration,
        setup_random_contract_negotiation,
    };
    use edc_client::{contract_negotiation_api, Error};

    #[tokio::test]
    async fn test_get_state_of_negotiation() {
        let provider = setup_provider_configuration();
        let consumer = setup_consumer_configuration();

        let (negotiation_id, _) = setup_random_contract_negotiation(&consumer, &provider).await;

        let state =
            contract_negotiation_api::get_negotiation_state(&consumer, &negotiation_id).await;

        assert!(state.is_ok());
    }

    #[tokio::test]
    async fn test_get_state_of_negotiation_with_unknown_id() {
        let consumer = setup_consumer_configuration();

        let id = "unknown_id";

        let response = contract_negotiation_api::get_negotiation_state(&consumer, &id).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => {
                panic!("Expected Status Code 404, because the contract negotiation does not exist")
            }
        }
    }
}

#[cfg(test)]
mod contract_negotiation_api_terminate_test {
    extern crate edc_api;
    extern crate edc_client;
    extern crate odrl;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration,
        setup_random_contract_negotiation,
    };
    use edc_api::TerminateNegotiationSchema;
    use edc_client::{contract_negotiation_api, Error};
    use odrl::name_spaces::EDC_NS;

    #[tokio::test]
    async fn test_terminate_negotiation() {
        let provider = setup_provider_configuration();
        let consumer = setup_consumer_configuration();

        let (negotiation_id, _) = setup_random_contract_negotiation(&consumer, &provider).await;

        let state =
            contract_negotiation_api::get_negotiation_state(&consumer, &negotiation_id.clone())
                .await
                .unwrap();

        assert_ne!(state.state, edc_api::ContractNegotiationState::Terminated);

        let terminate_schema = TerminateNegotiationSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: negotiation_id.clone(),
            at_type: None,
            reason: Some("Terminating for testing purposes".to_string()),
        };

        let response = contract_negotiation_api::terminate_negotiation(
            &consumer,
            &negotiation_id,
            Some(terminate_schema),
        )
        .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_terminate_negotiation_with_unknown_id() {
        let consumer = setup_consumer_configuration();

        let id = "unknown_id";

        let terminate_schema = TerminateNegotiationSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: id.clone().to_string(),
            at_type: None,
            reason: Some("Terminating for testing purposes should fail".to_string()),
        };

        let response =
            contract_negotiation_api::terminate_negotiation(&consumer, &id, Some(terminate_schema))
                .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => {
                panic!("Expected Status Code 404, because the contract negotiation does not exist")
            }
        }
    }
}
