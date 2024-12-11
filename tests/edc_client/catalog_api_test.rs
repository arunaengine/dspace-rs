mod common;

#[cfg(test)]
mod catalog_api_test {

    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{
        setup_consumer_configuration, setup_provider_configuration,
        setup_random_contract_definition, PROVIDER_PROTOCOL,
    };
    use edc_api::{CatalogRequest, DatasetRequest};
    use edc_client::catalog_api;

    #[tokio::test]
    async fn test_get_dataset() {
        let consumer = setup_consumer_configuration();
        let provider = setup_provider_configuration();

        let (asset_id, _, _) = setup_random_contract_definition(&provider).await;

        let request = DatasetRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("DatasetRequest".to_string()),
            at_id: Some(asset_id.to_string()),
            counter_party_address: Some(PROVIDER_PROTOCOL.to_string()),
            counter_party_id: None,
            protocol: Some("dataspace-protocol-http".to_string()),
            query_spec: None,
        };

        let dataset = catalog_api::get_dataset(&consumer, Some(request))
            .await
            .unwrap();

        assert!(dataset.get("id").is_some());
        assert_eq!(
            asset_id,
            dataset.get("id").unwrap().to_string().replace("\"", "")
        );
    }

    #[tokio::test]
    async fn test_get_catalog() {
        let consumer = setup_consumer_configuration();
        let provider = setup_provider_configuration();

        let (asset_id, _, _) = setup_random_contract_definition(&provider).await;

        let request = CatalogRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("CatalogRequest".to_string()),
            counter_party_address: PROVIDER_PROTOCOL.to_string(),
            counter_party_id: None,
            protocol: "dataspace-protocol-http".to_string(),
            query_spec: None,
        };

        let catalog = catalog_api::request_catalog(&consumer, Some(request))
            .await
            .unwrap();

        assert!(catalog.get("dcat:dataset").is_some());

        let dataset = catalog
            .get("dcat:dataset")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .find(|ds| ds.get("id").unwrap().to_string().replace("\"", "") == asset_id);

        assert!(dataset.is_some());
        assert_eq!(
            asset_id,
            dataset
                .unwrap()
                .get("id")
                .unwrap()
                .to_string()
                .replace("\"", "")
        );
    }
}
