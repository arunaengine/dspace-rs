mod common;

mod aruna_connector_negotiation_provider_api_test {
    use crate::common::{
        setup_dsp_provider_configuration, setup_management_consumer, setup_provider_configuration,
        DATASPACE_PROTOCOL, PROVIDER_ID, PROVIDER_PROTOCOL,
    };
    use dsp_client::contract_negotiation::negotiation_provider_api;
    use edc_api::{AssetInput, ContractOfferDescription, Offer};
    use edc_client::{asset_api, contract_negotiation_api};
    use odrl::name_spaces::LD_NS;
    use tokio::time::sleep;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_unknown_negotiation() {
        let config = setup_dsp_provider_configuration().await;
        let pid = "Test";

        let result = negotiation_provider_api::get_negotiation(&config, pid).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_negotiation() {
        let consumer = setup_management_consumer().await;
        let provider = setup_provider_configuration();

        let asset_id = Uuid::new_v4().to_string();

        let asset_input = AssetInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_id: Some(asset_id.clone()),
            at_type: Some("Asset".to_string()),
            data_address: Box::new(Default::default()),
            private_properties: None,
            properties: std::collections::HashMap::from([
                (
                    "name".to_string(),
                    serde_json::Value::String("test".to_string()),
                ),
                (
                    "foo".to_string(),
                    serde_json::Value::String("bar".to_string()),
                ),
            ]),
        };

        let asset_response = asset_api::create_asset(&provider, Some(asset_input)).await;

        assert!(asset_response.is_ok());

        sleep(std::time::Duration::from_secs(5)).await;

        let offer_id = Uuid::new_v4().to_string();

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

        let contract_request = edc_api::ContractRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("ContractRequest".to_string()),
            callback_addresses: None,
            connector_address: None,
            counter_party_address: "http://localhost:3000".to_string(),
            offer: Some(offer_description),
            policy: Some(offer.clone()),
            protocol: DATASPACE_PROTOCOL.to_string(),
            provider_id: None,
        };

        let contract_negotiation_response =
            contract_negotiation_api::initiate_contract_negotiation(
                &consumer,
                Some(contract_request),
            )
                .await;

        match contract_negotiation_response {
            Ok(response) => {
                assert!(response.at_id.is_some());
                assert!(response.created_at.is_some());
                let id = response.at_id.clone().unwrap();

                let neg = contract_negotiation_api::get_negotiation(&consumer, id.as_str()).await;
                assert!(neg.is_ok());
            }
            Err(e) => {
                panic!("Error: {:#?}", e);
            }
        }
    }

    #[tokio::test]
    async fn request_negotiation() {
        let consumer = setup_management_consumer().await;
        let provider = setup_provider_configuration();

        let asset_id = Uuid::new_v4().to_string();

        let asset_input = AssetInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_id: Some(asset_id.clone()),
            at_type: Some("Asset".to_string()),
            data_address: Box::new(Default::default()),
            private_properties: None,
            properties: std::collections::HashMap::from([
                (
                    "name".to_string(),
                    serde_json::Value::String("test".to_string()),
                ),
                (
                    "foo".to_string(),
                    serde_json::Value::String("bar".to_string()),
                ),
            ]),
        };

        let asset_response = asset_api::create_asset(&provider, Some(asset_input)).await;

        assert!(asset_response.is_ok());

        let offer_id = Uuid::new_v4().to_string();

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

        let contract_request = edc_api::ContractRequest {
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

        let contract_negotiation_response =
            contract_negotiation_api::initiate_contract_negotiation(
                &consumer,
                Some(contract_request),
            )
            .await;

        match contract_negotiation_response {
            Ok(response) => {
                assert!(response.at_id.is_some());
                assert!(response.created_at.is_some());
            }
            Err(e) => {
                panic!("Error: {:#?}", e);
            }
        }
    }

    #[tokio::test]
    async fn accept_negotiation() {
        unimplemented!()
    }

    #[tokio::test]
    async fn terminate_negotiation() {
        let consumer = setup_management_consumer().await;
        let provider = setup_provider_configuration();

        let asset_id = Uuid::new_v4().to_string();

        let asset_input = AssetInput {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_id: Some(asset_id.clone()),
            at_type: Some("Asset".to_string()),
            data_address: Box::new(Default::default()),
            private_properties: None,
            properties: std::collections::HashMap::from([
                (
                    "name".to_string(),
                    serde_json::Value::String("test".to_string()),
                ),
                (
                    "foo".to_string(),
                    serde_json::Value::String("bar".to_string()),
                ),
            ]),
        };

        let asset_response = asset_api::create_asset(&provider, Some(asset_input)).await;

        assert!(asset_response.is_ok());

        sleep(std::time::Duration::from_secs(5)).await;

        let offer_id = Uuid::new_v4().to_string();

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

        let contract_request = edc_api::ContractRequest {
            context: std::collections::HashMap::from([(
                "@vocab".to_string(),
                serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            )]),
            at_type: Some("ContractRequest".to_string()),
            callback_addresses: None,
            connector_address: None,
            counter_party_address: "http://localhost:3000".to_string(),
            offer: Some(offer_description),
            policy: Some(offer.clone()),
            protocol: DATASPACE_PROTOCOL.to_string(),
            provider_id: None,
        };

        let contract_negotiation_response =
            contract_negotiation_api::initiate_contract_negotiation(
                &consumer,
                Some(contract_request),
            )
            .await;

        match contract_negotiation_response {
            Ok(response) => {
                assert!(response.at_id.is_some());
                assert!(response.created_at.is_some());

                println!("Negotiation: {:#?}", response);

                sleep(std::time::Duration::from_secs(5)).await;

                let cn_pid = response.at_id.clone().unwrap();

                let termination_request = edc_api::TerminateNegotiationSchema {
                    context: std::collections::HashMap::from([(
                        "@vocab".to_string(),
                        serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
                    )]),
                    at_id: cn_pid.clone(),
                    at_type: None,
                    reason: Some("Terminating for testing purposes".to_string()),
                };

                let terminate_response = contract_negotiation_api::terminate_negotiation(
                    &consumer,
                    cn_pid.clone().as_str(),
                    Some(termination_request),
                )
                .await;
                assert!(terminate_response.is_ok());

                let id = response.at_id.clone().unwrap();

                let neg = contract_negotiation_api::get_negotiation(&consumer, id.as_str()).await;
            }
            Err(e) => {
                panic!("Error: {:#?}", e);
            }
        }
    }
}
