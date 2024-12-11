mod common;

#[cfg(test)]
mod asset_api_create_test {

    extern crate edc_api;
    extern crate edc_client;

    use crate::common::setup_provider_configuration;
    use edc_api::{AssetInput, DataAddress};
    use edc_client::{asset_api, Error};

    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "name".to_string(),
            serde_json::Value::String("test".to_string()),
        );
        properties.insert(
            "foo".to_string(),
            serde_json::Value::String("bar".to_string()),
        );
        let mut data_address = DataAddress::default();
        data_address.at_type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address);
        asset.properties = properties;

        let response = asset_api::create_asset(&configuration, Some(asset))
            .await
            .unwrap();

        assert!(response.at_id.is_some());
        assert_eq!(response.at_id.unwrap(), id.clone());
        assert!(response.created_at.is_some());
        assert!(response.created_at.unwrap() > 0);
    }

    #[tokio::test]
    async fn test_create_malformed_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "name".to_string(),
            serde_json::Value::String("test".to_string()),
        );
        properties.insert(
            "foo".to_string(),
            serde_json::Value::String("bar".to_string()),
        );
        let mut data_address = DataAddress::default();

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address);
        asset.properties = properties;
        asset.context = std::collections::HashMap::new();

        let response = asset_api::create_asset(&configuration, Some(asset)).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::BAD_REQUEST);
            }
            _ => panic!(
                "Expected Status Code 400, because of missing mandatory @type in DataAddress"
            ),
        }
    }

    #[tokio::test]
    async fn test_create_duplicate_id_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "name".to_string(),
            serde_json::Value::String("test".to_string()),
        );
        properties.insert(
            "foo".to_string(),
            serde_json::Value::String("bar".to_string()),
        );
        let mut data_address = DataAddress::default();
        data_address.at_type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address);
        asset.properties = properties;

        let _ = asset_api::create_asset(&configuration, Some(asset.clone())).await;

        let response = asset_api::create_asset(&configuration, Some(asset.clone())).await; // This should fail, because an asset with the same id already exists

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::CONFLICT);
            }
            _ => panic!("Expected Status Code 409, because an asset with that ID already exists"),
        }
    }
}

#[cfg(test)]
mod asset_api_get_test {

    extern crate edc_api;
    extern crate edc_client;

    use crate::common::setup_provider_configuration;
    use edc_api::{AssetInput, DataAddress};
    use edc_client::{asset_api, Error};

    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "name".to_string(),
            serde_json::Value::String("test".to_string()),
        );
        properties.insert(
            "foo".to_string(),
            serde_json::Value::String("bar".to_string()),
        );
        let mut data_address = DataAddress::default();
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address.clone());
        asset.properties = properties.clone();

        let _ = asset_api::create_asset(&configuration, Some(asset))
            .await
            .unwrap();

        let response = asset_api::get_asset(&configuration, &id.clone())
            .await
            .unwrap();

        // Check for correct properties
        assert!(response.properties.is_some());
        assert_eq!(
            response.properties.clone().unwrap().get("foo").unwrap(),
            &serde_json::Value::String("bar".to_string())
        );
        assert_eq!(
            response.properties.clone().unwrap().get("name").unwrap(),
            &serde_json::Value::String("test".to_string())
        );

        // Check for correct data address
        assert!(response.data_address.is_some());
        assert_eq!(
            response.data_address.unwrap(),
            Box::new(data_address.clone())
        );
    }

    #[tokio::test]
    async fn test_get_asset_with_unknown_id() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();

        let response = asset_api::get_asset(&configuration, &id.clone()).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because no asset with that ID exists"),
        }
    }
}

#[cfg(test)]
mod asset_api_delete_test {

    extern crate edc_api;
    extern crate edc_client;

    use crate::common::setup_provider_configuration;
    use edc_api::{AssetInput, DataAddress};
    use edc_client::{asset_api, Error};

    use uuid::Uuid;

    #[tokio::test]
    async fn test_delete_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "name".to_string(),
            serde_json::Value::String("test".to_string()),
        );
        properties.insert(
            "foo".to_string(),
            serde_json::Value::String("bar".to_string()),
        );
        let mut data_address = DataAddress::default();
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address.clone());
        asset.properties = properties.clone();

        let _ = asset_api::create_asset(&configuration, Some(asset))
            .await
            .unwrap();

        let response = asset_api::remove_asset(&configuration, &id.clone()).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_delete_asset_with_unknown_id() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();

        let response = asset_api::remove_asset(&configuration, &id.clone()).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because no asset with that ID exists"),
        }
    }

    /* TODO: Check for Error 409, after contract negotiation api is tested
    Assets that are part of a contract negotiation cannot be deleted */
}

#[cfg(test)]
mod asset_api_request_test {

    extern crate edc_api;
    extern crate edc_client;
    extern crate odrl;

    use crate::common::setup_provider_configuration;
    use edc_api::{AssetInput, Criterion, DataAddress, QuerySpec};
    use edc_client::{asset_api, Error};
    use odrl::name_spaces as NameSpaces;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_request_asset() {
        let configuration = setup_provider_configuration();

        let id_1 = Uuid::new_v4().to_string();
        let mut properties_1 = std::collections::HashMap::new();
        properties_1.insert(
            "name".to_string(),
            serde_json::Value::String("test".to_string()),
        );
        properties_1.insert(
            "foo".to_string(),
            serde_json::Value::String("bar".to_string()),
        );
        properties_1.insert(
            format!("{}{}", NameSpaces::EDC_NS, "id"),
            serde_json::Value::String(id_1.clone()),
        );
        let mut data_address = DataAddress::default();
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset_1 = AssetInput::default();
        asset_1.at_id = Some(id_1.clone());
        asset_1.data_address = Box::new(data_address.clone());
        asset_1.properties = properties_1.clone();

        let id_2 = Uuid::new_v4().to_string();
        let mut properties_2 = std::collections::HashMap::new();
        properties_2.insert(
            "name".to_string(),
            serde_json::Value::String("test2".to_string()),
        );
        properties_2.insert(
            "hello".to_string(),
            serde_json::Value::String("world".to_string()),
        );

        let mut asset_2 = AssetInput::default();
        asset_2.at_id = Some(id_2.clone());
        asset_2.data_address = Box::new(data_address.clone());
        asset_2.properties = properties_2.clone();

        let _ = asset_api::create_asset(&configuration, Some(asset_1))
            .await
            .unwrap();
        let _ = asset_api::create_asset(&configuration, Some(asset_2))
            .await
            .unwrap();

        let criterion = Criterion {
            at_type: None,
            operand_left: serde_json::Value::from(format!("{}{}", NameSpaces::EDC_NS, "id")),
            operator: "=".to_string(),
            operand_right: serde_json::Value::from(id_1.clone()),
        };

        // Should match only asset_1
        let query = QuerySpec {
            at_context: Some(std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(NameSpaces::EDC_NS.to_string()),
            )])),
            at_type: Some("QuerySpec".to_string()),
            filter_expression: vec![criterion],
            limit: None,
            offset: None,
            sort_field: None,
            sort_order: None,
        };

        let response = asset_api::request_assets(&configuration, Some(query.clone()))
            .await
            .unwrap();

        assert_eq!(response.len(), 1);
        assert_eq!(response[0].clone().at_id.clone().unwrap(), id_1.clone());
        assert_eq!(
            response[0].clone().properties.unwrap().get("name").unwrap(),
            &serde_json::Value::String("test".to_string())
        );
    }
}

#[cfg(test)]
mod asset_api_update_test {

    extern crate edc_api;
    extern crate edc_client;
    extern crate odrl;

    use crate::common::setup_provider_configuration;
    use edc_api::{AssetInput, DataAddress};
    use edc_client::{asset_api, Error};

    use uuid::Uuid;

    #[tokio::test]
    async fn test_update_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "name".to_string(),
            serde_json::Value::String("test".to_string()),
        );
        properties.insert(
            "foo".to_string(),
            serde_json::Value::String("bar".to_string()),
        );
        let mut data_address = DataAddress::default();
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address.clone());
        asset.properties = properties.clone();

        let _ = asset_api::create_asset(&configuration, Some(asset.clone()))
            .await
            .unwrap();

        let mut new_properties = std::collections::HashMap::new();
        new_properties.insert(
            "name".to_string(),
            serde_json::Value::String("test2".to_string()),
        );
        new_properties.insert(
            "foo".to_string(),
            serde_json::Value::String("bar2".to_string()),
        );

        let mut new_data_address = DataAddress::default();
        new_data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut new_asset = AssetInput::default();
        new_asset.at_id = Some(id.clone());
        new_asset.data_address = Box::new(new_data_address.clone());
        new_asset.properties = new_properties.clone();

        let response = asset_api::update_asset(&configuration, Some(new_asset.clone())).await;

        assert!(response.is_ok());

        let updated_asset = asset_api::get_asset(&configuration, &id.clone())
            .await
            .unwrap();

        assert_eq!(
            updated_asset
                .clone()
                .properties
                .unwrap()
                .get("name")
                .unwrap(),
            &serde_json::Value::String("test2".to_string())
        );
        assert_eq!(
            updated_asset
                .clone()
                .properties
                .unwrap()
                .get("foo")
                .unwrap(),
            &serde_json::Value::String("bar2".to_string())
        );
    }

    #[tokio::test]
    async fn test_update_non_existent_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut new_properties = std::collections::HashMap::new();
        new_properties.insert(
            "name".to_string(),
            serde_json::Value::String("test2".to_string()),
        );
        new_properties.insert(
            "foo".to_string(),
            serde_json::Value::String("bar2".to_string()),
        );

        let mut new_data_address = DataAddress::default();
        new_data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut new_asset = AssetInput::default();
        new_asset.at_id = Some(id.clone());
        new_asset.data_address = Box::new(new_data_address.clone());
        new_asset.properties = new_properties.clone();

        let response = asset_api::update_asset(&configuration, Some(new_asset.clone())).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because no asset with that ID exists"),
        }
    }
}
