mod common;

#[cfg(test)]
mod contract_agreement_api_test {

    extern crate edc_api;
    extern crate edc_client;
    extern crate odrl;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration,
        setup_random_contract_negotiation, wait_for_negotiation_state,
    };
    use edc_api::NegotiationState;
    use edc_client::{contract_agreement_api, contract_negotiation_api, Error};
    use odrl::name_spaces::EDC_NS;

    #[tokio::test]
    async fn test_get_agreement_by_id() {
        let provider = setup_provider_configuration();
        let consumer = setup_consumer_configuration();
        let (negotiation_id, asset_id) =
            setup_random_contract_negotiation(&consumer, &provider).await;

        wait_for_negotiation_state(
            &consumer,
            &negotiation_id,
            NegotiationState {
                state: edc_api::ContractNegotiationState::Finalized,
            },
        )
        .await;

        let agreement_id = contract_negotiation_api::get_negotiation(&consumer, &negotiation_id)
            .await
            .unwrap()
            .contract_agreement_id
            .unwrap();

        let agreement = contract_agreement_api::get_agreement_by_id(&consumer, &agreement_id)
            .await
            .unwrap();

        assert_eq!(agreement_id, agreement.clone().at_id.unwrap());
    }

    #[tokio::test]
    async fn test_get_agreement_with_unknown_id() {
        let consumer = setup_consumer_configuration();
        let agreement_id = "wrong_id".to_string();

        let agreement = contract_agreement_api::get_agreement_by_id(&consumer, &agreement_id).await;

        assert!(agreement.is_err());
        match agreement {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => {
                panic!("Expected Status Code 404, because there is no agreement with the given id")
            }
        }
    }

    #[tokio::test]
    async fn test_get_negotiation_by_id() {
        let provider = setup_provider_configuration();
        let consumer = setup_consumer_configuration();
        let (negotiation_id, asset_id) =
            setup_random_contract_negotiation(&consumer, &provider).await;

        wait_for_negotiation_state(
            &consumer,
            &negotiation_id,
            NegotiationState {
                state: edc_api::ContractNegotiationState::Finalized,
            },
        )
        .await;

        let agreement_id = contract_negotiation_api::get_negotiation(&consumer, &negotiation_id)
            .await
            .unwrap()
            .contract_agreement_id
            .unwrap();

        let negotiation =
            contract_agreement_api::get_negotiation_by_agreement_id(&consumer, &agreement_id)
                .await
                .unwrap();

        assert_eq!(negotiation_id, negotiation.clone().at_id.unwrap());
    }

    #[tokio::test]
    async fn test_get_negotiation_with_unknown_id() {
        let consumer = setup_consumer_configuration();
        let negotiation_id = "wrong_id".to_string();

        let negotiation =
            contract_agreement_api::get_negotiation_by_agreement_id(&consumer, &negotiation_id)
                .await;

        assert!(negotiation.is_err());
        match negotiation {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!(
                "Expected Status Code 404, because there is no negotiation with the given id"
            ),
        }
    }

    #[tokio::test]
    async fn test_get_agreements_by_query() {
        let provider = setup_provider_configuration();
        let consumer = setup_consumer_configuration();

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

        let (second_negotiation_id, second_asset_id) =
            setup_random_contract_negotiation(&consumer, &provider).await;

        wait_for_negotiation_state(
            &consumer,
            &second_negotiation_id,
            NegotiationState {
                state: edc_api::ContractNegotiationState::Finalized,
            },
        )
        .await;

        // Filter for specific asset id
        let criterion = edc_api::Criterion {
            at_type: None,
            operand_left: serde_json::Value::from("assetId"),
            operator: "=".to_string(),
            operand_right: serde_json::Value::from(asset_id.clone()),
        };

        let query = edc_api::QuerySpec {
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

        let agreements = contract_agreement_api::query_all_agreements(&consumer, Some(query))
            .await
            .unwrap();
        assert_eq!(1, agreements.len());
        let agreement_asset_id = agreements.get(0).unwrap().asset_id.clone().unwrap();
        assert_eq!(asset_id, agreement_asset_id);
    }
}
