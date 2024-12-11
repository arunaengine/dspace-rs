mod common;

#[cfg(test)]
mod aruna_connector_catalog_api_test {
    use edc_client::catalog_api;

    use crate::common::{setup_management_consumer, DATASPACE_PROTOCOL};

    #[tokio::test]
    async fn test_get_dataset() {
        let request = edc_api::DatasetRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("DatasetRequest".to_string()),
            at_id: Some("Test".to_string()),
            counter_party_address: Some("http://localhost:3000".to_string()),
            counter_party_id: Some("foobar".to_string()),
            protocol: Some(DATASPACE_PROTOCOL.to_string()),
            query_spec: None,
        };

        let config = setup_management_consumer().await;

        let result = catalog_api::get_dataset(&config, Some(request)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_unknown_dataset() {
        let request = edc_api::DatasetRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("DatasetRequest".to_string()),
            at_id: Some("unknown".to_string()),
            counter_party_address: Some("http://localhost:3000".to_string()),
            counter_party_id: Some("aruna-connector".to_string()),
            protocol: Some(DATASPACE_PROTOCOL.to_string()),
            query_spec: None,
        };

        let config = setup_management_consumer().await;

        let result = catalog_api::get_dataset(&config, Some(request)).await;
        println!("{:?}", result);
        assert!(result.is_err());
        //TODO: Check why err code is 502 (Bad gateway) instead of 404
        //TODO: Curl is resulting in 404
    }

    #[tokio::test]
    async fn test_get_catalog() {
        let config = setup_management_consumer().await;

        let request = edc_api::CatalogRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("CatalogRequest".to_string()),
            counter_party_address: "http://localhost:3000".to_string(),
            counter_party_id: Some("aruna-connector".to_string()),
            protocol: DATASPACE_PROTOCOL.to_string(),
            query_spec: None,
        };

        let result = catalog_api::request_catalog(&config, Some(request)).await;
        assert!(result.is_ok());
        match result {
            Ok(catalog) => {
                println!("{:#?}", catalog);
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        }
    }
}
