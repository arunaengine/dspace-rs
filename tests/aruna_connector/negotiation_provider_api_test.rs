mod common;

mod aruna_connector_negotiation_provider_api_test {
    use uuid::Uuid;
    use dsp_api::contract_negotiation::{AbstractPolicyRule, Action, ContractNegotiationEventMessage, ContractRequestMessage, MessageOffer, Permission, PolicyClass, Target};
    use dsp_api::contract_negotiation::contract_negotiation_event_message::EventType;
    use dsp_client::contract_negotiation::negotiation_provider_api;

    use crate::common::{setup_dsp_provider_configuration, PROVIDER_ID};

    #[tokio::test]
    async fn test_get_unknown_negotiation() {

        let config = setup_dsp_provider_configuration().await;
        let pid = "Test";

        let result = negotiation_provider_api::get_negotiation(&config, pid).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_negotiation() {

    }

    #[tokio::test]
    async fn request_negotiation() {

        let request_message = ContractRequestMessage {
            context: std::collections::HashMap::from([("dspace".to_string(), serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()))]),
            dsp_type: "dspace:ContractRequestMessage".to_string(),
            provider_pid: None,
            consumer_pid: Uuid::new_v4().to_string(),
            offer: MessageOffer {
                policy_class: PolicyClass {
                    abstract_policy_rule: AbstractPolicyRule { assigner: Some(PROVIDER_ID.to_string()), assignee: None },
                    id: "free-use-policy".to_string(),
                    profile: vec![],
                    permission: vec![Permission {
                        abstract_policy_rule: AbstractPolicyRule { assigner: Some("aruna-connector".to_string()), assignee: None },
                        action: Action::Use,
                        constraint: vec![],
                        duty: None,
                    }],
                    obligation: vec![],
                    target: Target::new("FreeUseResource".to_string()),
                },
                odrl_type: "odrl:Offer".to_string(),
            },
            callback_address: "http://consumer-connector:9194/protocol".to_string(),
        };

        let config = setup_dsp_provider_configuration().await;

        let result = negotiation_provider_api::request_negotiation(&config, request_message).await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn accept_negotiation() {

        let request_message = ContractRequestMessage {
            context: std::collections::HashMap::from([("dspace".to_string(), serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()))]),
            dsp_type: "dspace:ContractRequestMessage".to_string(),
            provider_pid: None,
            consumer_pid: Uuid::new_v4().to_string(),
            offer: MessageOffer {
                policy_class: PolicyClass {
                    abstract_policy_rule: AbstractPolicyRule { assigner: Some(PROVIDER_ID.to_string()), assignee: None },
                    id: "free-use-policy".to_string(),
                    profile: vec![],
                    permission: vec![Permission {
                        abstract_policy_rule: AbstractPolicyRule { assigner: Some("aruna-connector".to_string()), assignee: None },
                        action: Action::Use,
                        constraint: vec![],
                        duty: None,
                    }],
                    obligation: vec![],
                    target: Target::new("FreeUseResource".to_string()),
                },
                odrl_type: "odrl:Offer".to_string(),
            },
            callback_address: "http://consumer-connector:9194/protocol".to_string(),
        };

        let config = setup_dsp_provider_configuration().await;

        println!("Config: {:?}", config);

        let result = negotiation_provider_api::request_negotiation(&config, request_message).await;

        match result {
            Ok(negotiation) => {

                println!("Negotiation Requested Successful: {:?}", negotiation.clone());

                let consumer_pid = negotiation.clone().consumer_pid;
                let provider_pid = negotiation.clone().provider_pid;

                let accept_message = ContractNegotiationEventMessage {
                    context: std::collections::HashMap::from([("dspace".to_string(), serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()))]),
                    dsp_type: "dspace:ContractNegotiationEventMessage".to_string(),
                    provider_pid: provider_pid.clone(),
                    consumer_pid: consumer_pid.clone(),
                    event_type: EventType::ACCEPTED,
                };

                let accepted = negotiation_provider_api::accept_offer(&config, accept_message, provider_pid.clone().as_str()).await;

                println!("Negotiation Accept?: {:?}", accepted);
            }
            Err(e) => {
                println!("Negotiation Requested Error: {:?}", e);
            }
        }
    }
}