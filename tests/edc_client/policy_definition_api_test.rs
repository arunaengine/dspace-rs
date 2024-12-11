mod common;

#[cfg(test)]
mod policy_definition_api_create_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::setup_provider_configuration;
    use edc_api::PolicyDefinitionInput;
    use edc_client::{policy_definition_api, Error};
    use odrl::name_spaces::EDC_NS;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_policy_definition() {
        let provider_configuration = setup_provider_configuration();

        let policy_definition_id = Uuid::new_v4().to_string();

        let test_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Set",
            "uid": "api_test_policy",
            "permission": []
        }
        "#;

        let policy_definition = PolicyDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(policy_definition_id.clone()),
            at_type: Some("PolicyDefinition".to_string()),
            policy: serde_json::from_str(test_policy).unwrap(),
        };

        let response = policy_definition_api::create_policy_definition(
            &provider_configuration,
            Some(policy_definition),
        )
        .await
        .unwrap();

        assert_eq!(response.at_id.unwrap(), policy_definition_id);
    }

    #[tokio::test]
    async fn test_create_duplicate_id_policy_definition() {
        let provider_configuration = setup_provider_configuration();

        let policy_definition_id = Uuid::new_v4().to_string();

        let test_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Set",
            "uid": "api_test_policy",
            "permission": []
        }
        "#;

        let policy_definition = PolicyDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(policy_definition_id.clone()),
            at_type: Some("PolicyDefinition".to_string()),
            policy: serde_json::from_str(test_policy).unwrap(),
        };

        let response = policy_definition_api::create_policy_definition(
            &provider_configuration,
            Some(policy_definition.clone()),
        )
        .await
        .unwrap();

        assert_eq!(response.at_id.unwrap(), policy_definition_id);

        let response = policy_definition_api::create_policy_definition(
            &provider_configuration,
            Some(policy_definition),
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::CONFLICT);
            },
            _ => panic!("Expected Status Code 409, because a policy definition with the same id already exists"),
        }
    }
}

#[cfg(test)]
mod policy_definition_api_request_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{setup_provider_configuration, setup_random_contract_definition};
    use edc_api::{Criterion, QuerySpec};
    use edc_client::policy_definition_api;
    use odrl::name_spaces::EDC_NS;

    #[tokio::test]
    async fn test_request_policy_definition() {
        let provider_configuration = setup_provider_configuration();

        let (_, first_policy_definition_id, _) =
            setup_random_contract_definition(&provider_configuration).await;
        let (_, second_policy_definition_id, _) =
            setup_random_contract_definition(&provider_configuration).await;

        let criterion = Criterion {
            at_type: None,
            operand_left: serde_json::Value::from("id"),
            operator: "=".to_string(),
            operand_right: serde_json::Value::from(first_policy_definition_id.clone()),
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
            policy_definition_api::query_policy_definitions(&provider_configuration, Some(query))
                .await
                .unwrap();

        assert_eq!(response.len(), 1);
        let pulled_id = response.first().unwrap().at_id.as_ref().unwrap();
        assert_eq!(pulled_id, &first_policy_definition_id);
    }
}

#[cfg(test)]
mod policy_definition_api_get_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{setup_provider_configuration, setup_random_contract_definition};
    use edc_client::{policy_definition_api, Error};

    #[tokio::test]
    async fn test_get_policy_definition() {
        let provider_configuration = setup_provider_configuration();

        let (_, policy_definition_id, _) =
            setup_random_contract_definition(&provider_configuration).await;

        let response = policy_definition_api::get_policy_definition(
            &provider_configuration,
            &policy_definition_id,
        )
        .await
        .unwrap();

        assert_eq!(response.at_id.unwrap(), policy_definition_id);
    }

    #[tokio::test]
    async fn test_get_non_existing_policy_definition() {
        let provider_configuration = setup_provider_configuration();

        let response = policy_definition_api::get_policy_definition(
            &provider_configuration,
            "non-existing-id",
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because the policy definition does not exist"),
        }
    }
}

#[cfg(test)]
mod policy_definition_api_update_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{setup_provider_configuration, setup_random_contract_definition};
    use edc_api::PolicyDefinitionInput;
    use edc_client::{policy_definition_api, Error};
    use odrl::name_spaces::EDC_NS;

    #[tokio::test]
    async fn test_update_policy_definition() {
        let provider_configuration = setup_provider_configuration();

        let (_, policy_definition_id, _) =
            setup_random_contract_definition(&provider_configuration).await;

        let test_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Set",
            "uid": "update_policy_id",
            "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "action": "use"
            }]
        }
        "#;

        let policy_definition = PolicyDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(policy_definition_id.clone()),
            at_type: Some("PolicyDefinition".to_string()),
            policy: serde_json::from_str(test_policy).unwrap(),
        };

        let response = policy_definition_api::update_policy_definition(
            &provider_configuration,
            &policy_definition_id,
            Some(policy_definition),
        )
        .await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_update_non_existing_policy_definition() {
        let provider_configuration = setup_provider_configuration();

        let test_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Set",
            "uid": "update_non_existing_policy",
            "permission": []
        }
        "#;

        let policy_definition = PolicyDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some("non-existing-id".to_string()),
            at_type: Some("PolicyDefinition".to_string()),
            policy: serde_json::from_str(test_policy).unwrap(),
        };

        let response = policy_definition_api::update_policy_definition(
            &provider_configuration,
            "non-existing-id",
            Some(policy_definition),
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because the policy definition does not exist"),
        }
    }
}

#[cfg(test)]
mod policy_definition_api_delete_test {
    extern crate edc_api;
    extern crate edc_client;

    use crate::common::{setup_provider_configuration, setup_random_contract_definition};
    use edc_api::PolicyDefinitionInput;
    use edc_client::{policy_definition_api, Error};
    use odrl::name_spaces::EDC_NS;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_delete_policy_definition() {
        let provider_configuration = setup_provider_configuration();

        let policy_definition_id = Uuid::new_v4().to_string();

        let test_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Set",
            "uid": "delete_test_policy",
            "permission": []
        }
        "#;

        let policy_definition = PolicyDefinitionInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String(EDC_NS.to_string()),
            )]),
            at_id: Some(policy_definition_id.clone()),
            at_type: Some("PolicyDefinition".to_string()),
            policy: serde_json::from_str(test_policy).unwrap(),
        };

        let push_policy = policy_definition_api::create_policy_definition(
            &provider_configuration,
            Some(policy_definition),
        )
        .await;
        assert!(push_policy.is_ok());

        let response = policy_definition_api::delete_policy_definition(
            &provider_configuration,
            &policy_definition_id,
        )
        .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_delete_non_existing_policy_definition() {
        let provider_configuration = setup_provider_configuration();

        let response = policy_definition_api::delete_policy_definition(
            &provider_configuration,
            "non-existing-id",
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            }
            _ => panic!("Expected Status Code 404, because the policy definition does not exist"),
        }
    }

    #[tokio::test]
    async fn test_delete_policy_definition_of_contract() {
        let provider_configuration = setup_provider_configuration();

        let (_, policy_definition_id, _) =
            setup_random_contract_definition(&provider_configuration).await;

        let response = policy_definition_api::delete_policy_definition(
            &provider_configuration,
            &policy_definition_id,
        )
        .await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::CONFLICT);
            }
            _ => panic!("Expected Status Code 409, because the policy definition is still in use"),
        }
    }
}
