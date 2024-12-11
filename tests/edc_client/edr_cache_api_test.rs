mod common;

#[cfg(test)]
mod edr_cache_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration, setup_random_transfer_process,
        wait_for_transfer_state,
    };
    use edc_api::transfer_state::TransferProcessState;
    use edc_api::{Criterion, QuerySpec, TransferState};
    use edc_client::Error;
    use odrl::name_spaces::EDC_NS;

    #[tokio::test]
    async fn test_get_edr_cache_data_address() {
        let consumer = setup_consumer_configuration();
        let provider = setup_provider_configuration();

        let (transfer_process_id, _, _) = setup_random_transfer_process(&consumer, &provider).await;

        wait_for_transfer_state(
            &consumer,
            &transfer_process_id.clone(),
            TransferState {
                state: TransferProcessState::Started,
            },
        )
        .await;

        let data_address =
            edc_client::edr_cache_api::get_edr_data_address(&consumer, &transfer_process_id).await;

        assert!(data_address.is_ok());
    }

    #[tokio::test]
    async fn test_get_data_address_of_unknown_transfer() {
        let consumer = setup_consumer_configuration();
        let provider = setup_provider_configuration();

        let transfer_process_id = "unknown_transfer";

        let response =
            edc_client::edr_cache_api::get_edr_data_address(&consumer, &transfer_process_id).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            },
            _ => panic!("Expected Status Code 404, because a transfer process with the given id does not exist and therefore no edr entry exists."),
        }
    }

    #[tokio::test]
    async fn test_query_edr_cache() {
        let consumer = setup_consumer_configuration();
        let provider = setup_provider_configuration();

        let (transfer_process_id, _, asset_id) =
            setup_random_transfer_process(&consumer, &provider).await;

        wait_for_transfer_state(
            &consumer,
            &transfer_process_id.clone(),
            TransferState {
                state: TransferProcessState::Started,
            },
        )
        .await;

        let criterion = Criterion {
            at_type: None,
            operand_left: serde_json::Value::from("assetId"),
            operator: "=".to_string(),
            operand_right: serde_json::Value::from(asset_id.clone()),
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

        let response = edc_client::edr_cache_api::query_edrs(&consumer, Some(query)).await;

        assert!(response.is_ok());

        let edr = response.unwrap();

        assert_eq!(edr.clone().len(), 1);

        let edr_id = edr.clone().get(0).unwrap().at_id.clone().unwrap();

        assert_eq!(transfer_process_id, edr_id);
    }
}
