mod common;

#[cfg(test)]
mod dataplane_selector_api_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::setup_provider_configuration;
    use edc_api::{DataAddress, DataPlaneInstanceSchema, SelectionRequestSchema};
    use edc_client::{dataplane_selector_api, Error};
    use odrl::name_spaces::EDC_NS;

    use uuid::Uuid;

    #[tokio::test]
    async fn dataplane_selector_api_add_entry_test() {
        let provider_configuration = setup_provider_configuration();

        let dataplane_id = Uuid::new_v4().to_string();

        let dataplane_instance = DataPlaneInstanceSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(dataplane_id.clone()),
            at_type: None,
            allowed_dest_types: vec![
                "dest-type1".to_string(),
                "https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string(),
            ],
            allowed_source_types: vec!["source-type1".to_string(), "source-type2".to_string()],
            allowed_transfer_types: Some(vec![
                "transfer-type-1".to_string(),
                "transfer-type-2".to_string(),
            ]),
            last_active: None,
            turn_count: None,
            url: "https://somewhere.com:1234/api/v1".to_string(),
            properties: None,
        };

        let response =
            dataplane_selector_api::add_entry(&provider_configuration, Some(dataplane_instance))
                .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn dataplane_selector_api_find_test() {
        let provider_configuration = setup_provider_configuration();

        let first_dataplane_id = Uuid::new_v4().to_string();
        let second_dataplane_id = Uuid::new_v4().to_string();

        let first_dataplane_instance = DataPlaneInstanceSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(first_dataplane_id.clone()),
            at_type: None,
            allowed_dest_types: vec![
                "test-dst1".to_string(),
                "https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string(),
            ],
            allowed_source_types: vec!["test-src1".to_string(), "test-src2".to_string()],
            allowed_transfer_types: Some(vec![
                "transfer-type-1".to_string(),
                "transfer-type-2".to_string(),
            ]),
            last_active: None,
            turn_count: None,
            url: "https://somewhere.com:1234/api/v1".to_string(),
            properties: None,
        };

        let second_dataplane_instance = DataPlaneInstanceSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(second_dataplane_id.clone()),
            at_type: None,
            allowed_dest_types: vec![
                "test-dst2".to_string(),
                "https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string(),
            ],
            allowed_source_types: vec!["test-src3".to_string(), "test-src4".to_string()],
            allowed_transfer_types: None,
            last_active: None,
            turn_count: None,
            url: "https://somewhere2.com:1234/api/v1".to_string(),
            properties: None,
        };

        let first = dataplane_selector_api::add_entry(
            &provider_configuration,
            Some(first_dataplane_instance),
        )
        .await;
        let second = dataplane_selector_api::add_entry(
            &provider_configuration,
            Some(second_dataplane_instance),
        )
        .await;

        let dataplane_selection = SelectionRequestSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_type: None,
            destination: Some(Box::new(DataAddress {
                at_type: Some("test-dst1".to_string()),
                r#type: Some("test-dst1".to_string()),
                base_url: None,
            })),
            source: Some(Box::new(DataAddress {
                at_type: Some("test-src1".to_string()),
                r#type: Some("test-src1".to_string()),
                base_url: None,
            })),
            strategy: None,
            transfer_type: Some("transfer-type-1".to_string()),
        };

        let response =
            dataplane_selector_api::find(&provider_configuration, Some(dataplane_selection)).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn dataplane_selector_api_find_none_test() {
        let provider_configuration = setup_provider_configuration();

        let dataplane_id = Uuid::new_v4().to_string();

        let dataplane = DataPlaneInstanceSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(dataplane_id.clone()),
            at_type: None,
            allowed_dest_types: vec![
                "test-dst1".to_string(),
                "https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string(),
            ],
            allowed_source_types: vec!["test-src1".to_string(), "test-src2".to_string()],
            allowed_transfer_types: Some(vec![
                "transfer-type-1".to_string(),
                "transfer-type-2".to_string(),
            ]),
            last_active: None,
            turn_count: None,
            url: "https://somewhere.com:1234/api/v1".to_string(),
            properties: None,
        };

        let response =
            dataplane_selector_api::add_entry(&provider_configuration, Some(dataplane)).await;

        assert!(response.is_ok());

        let dataplane_selection = SelectionRequestSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_type: None,
            destination: Some(Box::new(DataAddress {
                at_type: Some("test-dst1".to_string()),
                r#type: Some("test-dst1".to_string()),
                base_url: None,
            })),
            source: Some(Box::new(DataAddress {
                at_type: Some("test-src1".to_string()),
                r#type: Some("test-src1".to_string()),
                base_url: None,
            })),
            strategy: None,
            transfer_type: Some("wrong-type-1".to_string()),
        };

        let response =
            dataplane_selector_api::find(&provider_configuration, Some(dataplane_selection)).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NO_CONTENT);
            }
            _ => panic!("Expected Status Code 204, because no suitable dataplane was found"),
        }
    }

    #[tokio::test]
    async fn dataplane_selector_api_get_all_test() {
        let provider_configuration = setup_provider_configuration();

        let first_dataplane_id = Uuid::new_v4().to_string();
        let second_dataplane_id = Uuid::new_v4().to_string();

        let first_dataplane_instance = DataPlaneInstanceSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(first_dataplane_id.clone()),
            at_type: None,
            allowed_dest_types: vec![
                "test-dst1".to_string(),
                "https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string(),
            ],
            allowed_source_types: vec!["test-src1".to_string(), "test-src2".to_string()],
            allowed_transfer_types: Some(vec![
                "transfer-type-1".to_string(),
                "transfer-type-2".to_string(),
            ]),
            last_active: None,
            turn_count: None,
            url: "https://somewhere.com:1234/api/v1".to_string(),
            properties: None,
        };

        let second_dataplane_instance = DataPlaneInstanceSchema {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(second_dataplane_id.clone()),
            at_type: None,
            allowed_dest_types: vec![
                "test-dst2".to_string(),
                "https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string(),
            ],
            allowed_source_types: vec!["test-src3".to_string(), "test-src4".to_string()],
            allowed_transfer_types: Some(vec![
                "transfer-type-3".to_string(),
                "transfer-type-4".to_string(),
            ]),
            last_active: None,
            turn_count: None,
            url: "https://somewhere2.com:1234/api/v1".to_string(),
            properties: None,
        };

        let first = dataplane_selector_api::add_entry(
            &provider_configuration,
            Some(first_dataplane_instance),
        )
        .await;
        let second = dataplane_selector_api::add_entry(
            &provider_configuration,
            Some(second_dataplane_instance),
        )
        .await;

        let response = dataplane_selector_api::get_all(&provider_configuration).await;

        assert!(response.is_ok());
        assert!(response.unwrap().len() >= 2);
    }
}
