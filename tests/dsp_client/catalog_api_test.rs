mod common;

#[cfg(test)]
mod dsp_catalog_api_test {

    use crate::common::setup_consumer_configuration;

    #[tokio::test]
    async fn test_request_catalog() {
        let configuration = setup_consumer_configuration();
        let request_message = dsp_api::catalog::CatalogRequestMessage {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(
                    "https://w3id.org/dspace/2024/1/context.json".to_string(),
                ),
            )]),
            dsp_type: "https://w3id.org/dspace/v0.8/CatalogRequestMessage".to_string(),
            filter: Vec::new(),
        };
        let response =
            dsp_client::catalog::catalog_api::request_catalog(&configuration, request_message)
                .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_request_catalog_malformed_request() {
        let configuration = setup_consumer_configuration();
        let request_message = dsp_api::catalog::CatalogRequestMessage {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(
                    "https://w3id.org/dspace/2024/1/context.json".to_string(),
                ),
            )]),
            dsp_type: "wrong type".to_string(),
            filter: Vec::new(),
        };
        let response =
            dsp_client::catalog::catalog_api::request_catalog(&configuration, request_message)
                .await;

        assert!(response.is_err());
        match response {
            Err(dsp_client::Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::BAD_REQUEST);
            }
            _ => panic!("Expected Status Code 400, because the request message is malformed."),
        }
    }
}

#[cfg(test)]
mod dsp_dataset_api_test {

    use crate::common::setup_consumer_configuration;
    use dsp_client::Error;

    #[tokio::test]
    async fn test_get_dataset_unknown_id() {
        let configuration = setup_consumer_configuration();
        let id = "1234";
        let response = dsp_client::catalog::catalog_api::get_dataset(&configuration, id).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => {
                panic!("Expected Status Code 404, because the dataset with id 1234 does not exist.")
            }
        }
    }
}
