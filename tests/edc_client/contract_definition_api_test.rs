mod common;

#[cfg(test)]
mod contract_definition_api_create_test {
    extern crate edc_api;
    extern crate edc_client;
    extern crate odrl;

    use crate::common::setup_provider_configuration;
    use edc_client::{contract_definition_api, Error};
    use odrl::name_spaces::EDC_NS;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_contract_definition() {
        let provider = setup_provider_configuration();

        let definition_id = Uuid::new_v4().to_string();

        let contract_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(definition_id.clone()),
            at_type: None,
            access_policy_id: "1".to_string(),
            assets_selector: vec![],
            contract_policy_id: "1".to_string(),
        };

        let response = contract_definition_api::create_contract_definition(
            &provider,
            Some(contract_definition),
        )
        .await
        .unwrap();

        assert_eq!(definition_id.clone(), response.at_id.unwrap());
    }

    #[tokio::test]
    async fn test_create_malformed_contract_definition() {
        let provider = setup_provider_configuration();

        let definition_id = Uuid::new_v4().to_string();

        let contract_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(definition_id.clone()),
            at_type: None,
            access_policy_id: String::new(),
            assets_selector: vec![],
            contract_policy_id: String::new(),
        };

        let response = contract_definition_api::create_contract_definition(
            &provider,
            Some(contract_definition),
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::BAD_REQUEST);
            }
            _ => panic!("Expected Status Code 400, because the contract definition is malformed"),
        }
    }

    #[tokio::test]
    async fn test_create_duplicate_contract_definition() {
        let provider = setup_provider_configuration();

        let definition_id = Uuid::new_v4().to_string();

        let contract_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(definition_id.clone()),
            at_type: None,
            access_policy_id: "duplicate".to_string(),
            assets_selector: vec![],
            contract_policy_id: "duplicate".to_string(),
        };

        let response = contract_definition_api::create_contract_definition(
            &provider,
            Some(contract_definition.clone()),
        )
        .await
        .unwrap();

        assert_eq!(definition_id.clone(), response.at_id.unwrap());

        let response = contract_definition_api::create_contract_definition(
            &provider,
            Some(contract_definition.clone()),
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::CONFLICT);
            }
            _ => panic!("Expected Status Code 409, because the contract definition already exists"),
        }
    }
}

#[cfg(test)]
mod contract_definition_api_delete_test {
    extern crate edc_api;
    extern crate edc_client;
    extern crate odrl;

    use crate::common::setup_provider_configuration;
    use edc_client::{contract_definition_api, Error};
    use odrl::name_spaces::EDC_NS;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_delete_contract_definition() {
        let provider = setup_provider_configuration();

        let definition_id = Uuid::new_v4().to_string();

        let contract_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(definition_id.clone()),
            at_type: None,
            access_policy_id: "delete-me".to_string(),
            assets_selector: vec![],
            contract_policy_id: "delete-me".to_string(),
        };

        let response = contract_definition_api::create_contract_definition(
            &provider,
            Some(contract_definition),
        )
        .await
        .unwrap();

        assert_eq!(definition_id.clone(), response.at_id.unwrap());

        let response =
            contract_definition_api::delete_contract_definition(&provider, &definition_id).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_delete_unknown_contract_definition() {
        let provider = setup_provider_configuration();

        let definition_id = "delete-me".to_string();

        let response =
            contract_definition_api::delete_contract_definition(&provider, &definition_id).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because the contract definition does not exist"),
        }
    }
}

#[cfg(test)]
mod contract_definition_api_get_test {
    extern crate edc_api;
    extern crate edc_client;
    extern crate odrl;

    use crate::common::setup_provider_configuration;
    use edc_client::{contract_definition_api, Error};
    use odrl::name_spaces::EDC_NS;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_contract_definition() {
        let provider = setup_provider_configuration();

        let definition_id = Uuid::new_v4().to_string();

        let contract_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(definition_id.clone()),
            at_type: None,
            access_policy_id: "get-me".to_string(),
            assets_selector: vec![],
            contract_policy_id: "get-me".to_string(),
        };

        let created = contract_definition_api::create_contract_definition(
            &provider,
            Some(contract_definition.clone()),
        )
        .await
        .unwrap();

        let pulled =
            contract_definition_api::get_contract_definition(&provider, &definition_id.clone())
                .await
                .unwrap();

        assert_eq!(definition_id.clone(), pulled.at_id.unwrap());
        assert_eq!(
            contract_definition.access_policy_id,
            pulled.access_policy_id.unwrap()
        );
        assert_eq!(
            contract_definition.contract_policy_id,
            pulled.contract_policy_id.unwrap()
        );
    }

    #[tokio::test]
    async fn test_get_contract_definition_with_unknown_id() {
        let provider = setup_provider_configuration();

        let definition_id = "get-me-not".to_string();

        let response =
            contract_definition_api::get_contract_definition(&provider, &definition_id).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because the contract definition does not exist"),
        }
    }
}

#[cfg(test)]
mod contract_definition_api_query_test {
    extern crate edc_api;
    extern crate edc_client;
    extern crate odrl;

    use crate::common::setup_provider_configuration;
    use edc_client::contract_definition_api;
    use odrl::name_spaces::EDC_NS;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_query_contract_definitions() {
        let provider = setup_provider_configuration();

        let definition_id = Uuid::new_v4().to_string();

        let contract_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(definition_id.clone()),
            at_type: None,
            access_policy_id: "query-me".to_string(),
            assets_selector: vec![],
            contract_policy_id: "query-me".to_string(),
        };

        let second_id = Uuid::new_v4().to_string();

        let second_contract_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(second_id.clone()),
            at_type: None,
            access_policy_id: "query-me-not".to_string(),
            assets_selector: vec![],
            contract_policy_id: "query-me-not".to_string(),
        };

        let first_created = contract_definition_api::create_contract_definition(
            &provider,
            Some(contract_definition.clone()),
        )
        .await
        .unwrap();
        let second_created = contract_definition_api::create_contract_definition(
            &provider,
            Some(second_contract_definition.clone()),
        )
        .await
        .unwrap();

        let criterion = edc_api::Criterion {
            at_type: None,
            operand_left: serde_json::Value::from("id"),
            operator: "=".to_string(),
            operand_right: serde_json::Value::from(definition_id.clone()),
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

        let definitions =
            contract_definition_api::query_all_contract_definitions(&provider, Some(query))
                .await
                .unwrap();
        assert_eq!(1, definitions.len());
        let queried_id = definitions.first().unwrap().at_id.clone().unwrap();
        assert_eq!(definition_id.clone(), queried_id);
    }
}

#[cfg(test)]
mod contract_definition_api_update_test {
    extern crate edc_api;
    extern crate edc_client;
    extern crate odrl;

    use crate::common::setup_provider_configuration;
    use edc_client::{contract_definition_api, Error};
    use odrl::name_spaces::EDC_NS;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_update_contract_definition() {
        let provider = setup_provider_configuration();

        let definition_id = Uuid::new_v4().to_string();

        let contract_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(definition_id.clone()),
            at_type: None,
            access_policy_id: "update-me".to_string(),
            assets_selector: vec![],
            contract_policy_id: "update-me".to_string(),
        };

        let created = contract_definition_api::create_contract_definition(
            &provider,
            Some(contract_definition.clone()),
        )
        .await
        .unwrap();

        let updated_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(definition_id.clone()),
            at_type: None,
            access_policy_id: "updated".to_string(),
            assets_selector: vec![],
            contract_policy_id: "updated".to_string(),
        };

        let updated = contract_definition_api::update_contract_definition(
            &provider,
            Some(updated_definition.clone()),
        )
        .await;

        assert!(updated.is_ok());

        let pulled_update =
            contract_definition_api::get_contract_definition(&provider, &definition_id.clone())
                .await
                .unwrap();

        assert_eq!(definition_id.clone(), pulled_update.at_id.unwrap());
        assert_eq!(
            updated_definition.access_policy_id,
            pulled_update.access_policy_id.unwrap()
        );
        assert_eq!(
            updated_definition.contract_policy_id,
            pulled_update.contract_policy_id.unwrap()
        );
    }

    #[tokio::test]
    async fn test_update_contract_definition_with_unknown_id() {
        let provider = setup_provider_configuration();

        let definition_id = "update-me-not".to_string();

        let updated_definition = edc_api::ContractDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(definition_id.clone()),
            at_type: None,
            access_policy_id: "updated".to_string(),
            assets_selector: vec![],
            contract_policy_id: "updated".to_string(),
        };

        let updated = contract_definition_api::update_contract_definition(
            &provider,
            Some(updated_definition.clone()),
        )
        .await;

        assert!(updated.is_err());
        match updated {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because the contract definition does not exist"),
        }
    }
}
