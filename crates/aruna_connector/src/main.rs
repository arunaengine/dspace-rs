use axum::extract::Path;
use axum::http::HeaderMap;
use axum::routing::{delete, put};
use axum::{
    routing::{get, post},
    Json, Router, ServiceExt,
};
use dsp_api::catalog::{
    AbstractDataset, Catalog, DataService, Dataset, Distribution, MultiLanguage, Resource,
};
use dsp_api::contract_negotiation::{
    AbstractPolicyRule, Action, Constraint, Duty, LeftOperand, MessageOffer, Offer, Operator,
    Permission, PolicyClass, RightOperand, Target,
};
use dsp_api::transfer_process::TransferProcess;
use edc_api::ContractAgreement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::Level;
use tracing::{error, info};
use tracing_subscriber;

pub mod api {
    pub mod management_asset;
    pub mod management_catalog;
    pub mod management_contract_agreement;
    pub mod management_contract_definition;
    pub mod management_contract_negotiation;
    pub mod management_policy_definition;
    pub mod management_transfer_process;

    pub mod catalog;
    pub mod contract_negotiation_provider;
    pub mod transfer_process_provider;
}

use crate::api::management_asset;
use crate::api::management_catalog;
use crate::api::management_contract_agreement;
use crate::api::management_contract_definition;
use crate::api::management_contract_negotiation;
use crate::api::management_policy_definition;
use crate::api::management_transfer_process;

use crate::api::catalog;
use crate::api::contract_negotiation_provider as negotiation_provider;
use crate::api::transfer_process_provider as tp_provider;

async fn debug_route(
    headers: HeaderMap,
    Path(path): Path<String>,
    value: Option<Json<serde_json::Value>>,
) {
    info!(
        "Debug route called path {:#?} with value: {:#?}\nHeader: {:#?}",
        path, value, headers
    );
}

async fn initiate_dsp_catalog() -> Catalog {
    let context: HashMap<String, serde_json::Value> = HashMap::from([(
        "dspace".to_string(),
        serde_json::Value::String("https://w3id.org/dspace/2024/1/context.json".to_string()),
    )]);

    let first_dataset = Dataset::new(AbstractDataset {
        resource: Resource {
            id: Some("Test".to_string()),
            keywords: None,
            themes: None,
            conforms_to: None,
            creator: None,
            descriptions: None,
            identifier: None,
            issued: None,
            modified: None,
            title: None,
        },
        policies: Some(vec![Offer {
            message_offer: MessageOffer {
                policy_class: PolicyClass {
                    abstract_policy_rule: AbstractPolicyRule {
                        assigner: Some("aruna-connector".to_string()),
                        assignee: None,
                    },
                    id: "test-policy".to_string(),
                    profile: vec![],
                    permission: vec![Permission {
                        abstract_policy_rule: AbstractPolicyRule {
                            assigner: Some("aruna-connector".to_string()),
                            assignee: None,
                        },
                        action: Action::Use,
                        constraint: vec![],
                        duty: None,
                    }],
                    obligation: vec![],
                    target: Target {
                        id: "Test".to_string(),
                    },
                },
                odrl_type: "odrl:Offer".to_string(),
            },
        }]),
        distributions: None,
    });
    let second_dataset = Dataset::new(AbstractDataset {
        resource: Resource {
            id: Some("3dd1add8-4d2d-569e-d634-8394a8836a88".to_string()),
            keywords: Some(vec!["traffic".to_string()]),
            themes: None,
            conforms_to: None,
            creator: None,
            descriptions: Some(vec![MultiLanguage {
                value: "Traffic data sample extract".to_string(),
                language: "en".to_string(),
            }]),
            identifier: Some("3dd1add8-4d2d-569e-d634-8394a8836a88".to_string()),
            issued: None,
            modified: None,
            title: Some("Traffic Data".to_string()),
        },
        policies: Some(vec![Offer {
            message_offer: MessageOffer {
                policy_class: PolicyClass {
                    abstract_policy_rule: AbstractPolicyRule {
                        assigner: Some("http://example.com/Provider".to_string()),
                        assignee: None,
                    },
                    id: "3dd1add8-4d2d-569e-d634-8394a8836a88".to_string(),
                    profile: vec![],
                    permission: vec![Permission {
                        abstract_policy_rule: AbstractPolicyRule {
                            assigner: Some("http://example.com/Provider".to_string()),
                            assignee: None,
                        },
                        action: Action::Use,
                        constraint: vec![Constraint {
                            right_operand: Some(RightOperand::String(
                                "http://example.org/EU".to_string(),
                            )),
                            right_operand_reference: None,
                            left_operand: LeftOperand::AbsolutePosition,
                            operator: Operator::Eq,
                        }],
                        duty: Some(Duty {
                            abstract_policy_rule: AbstractPolicyRule {
                                assigner: None,
                                assignee: None,
                            },
                            id: None,
                            action: Action::Attribution,
                            constraint: vec![],
                        }),
                    }],
                    obligation: vec![],
                    target: Target {
                        id: "3dd1add8-4d2d-569e-d634-8394a8836a88".to_string(),
                    },
                },
                odrl_type: "odrl:Offer".to_string(),
            },
        }]),
        distributions: Some(vec![Distribution {
            title: None,
            descriptions: vec![],
            issued: None,
            modified: None,
            policy: vec![],
            access_services: vec![DataService {
                resource: Resource {
                    id: None,
                    keywords: None,
                    themes: None,
                    conforms_to: None,
                    creator: None,
                    descriptions: None,
                    identifier: None,
                    issued: None,
                    modified: None,
                    title: None,
                },
                endpoint_description: None,
                endpoint_url: Some("https://provider-a.com/connector".to_string()),
                serves_datasets: None,
            }],
        }]),
    });
    let free_resource = Resource {
        id: Some("FreeUseResource".to_string()),
        keywords: Some(vec!["free".to_string()]),
        themes: None,
        conforms_to: None,
        creator: Some("Aruna".to_string()),
        descriptions: Some(vec![MultiLanguage {
            value: "Free use dataset for testing purposes".to_string(),
            language: "en".to_string(),
        }]),
        identifier: Some("FreeUseResource".to_string()),
        issued: None,
        modified: None,
        title: None,
    };
    let offer_policy = Offer {
        message_offer: MessageOffer {
            policy_class: PolicyClass {
                abstract_policy_rule: AbstractPolicyRule {
                    assigner: Some("aruna-connector".to_string()),
                    assignee: None,
                },
                id: "free-use-policy".to_string(),
                profile: vec![],
                permission: vec![Permission {
                    abstract_policy_rule: AbstractPolicyRule {
                        assigner: Some("aruna-connector".to_string()),
                        assignee: None,
                    },
                    action: Action::Use,
                    constraint: vec![],
                    duty: None,
                }],
                obligation: vec![],
                target: Target {
                    id: free_resource.clone().id.unwrap(),
                },
            },
            odrl_type: "odrl:Offer".to_string(),
        },
    };
    let free_use_dataset = Dataset::new(AbstractDataset {
        resource: free_resource.clone(),
        policies: Some(vec![offer_policy.clone()]),
        distributions: Some(vec![Distribution {
            title: None,
            descriptions: vec![MultiLanguage {
                value: "Free use dataset for testing purposes".to_string(),
                language: "en".to_string(),
            }],
            issued: None,
            modified: None,
            policy: vec![offer_policy.clone()],
            access_services: vec![DataService {
                resource: free_resource.clone(),
                endpoint_description: None,
                endpoint_url: Some("https://jsonplaceholder.typicode.com/users".to_string()),
                serves_datasets: None,
            }],
        }]),
    });
    let datasets = Vec::from([first_dataset, second_dataset, free_use_dataset]);

    let data_service = vec![DataService {
        resource: Resource {
            id: None,
            keywords: None,
            themes: None,
            conforms_to: None,
            creator: None,
            descriptions: None,
            identifier: None,
            issued: None,
            modified: None,
            title: None,
        },
        endpoint_description: None,
        endpoint_url: Some("https://aruna-connector/public".to_string()),
        serves_datasets: None,
    }];

    let catalog = Catalog::new(
        AbstractDataset {
            resource: Resource {
                id: None,
                keywords: None,
                themes: None,
                conforms_to: None,
                creator: None,
                descriptions: None,
                identifier: None,
                issued: None,
                modified: None,
                title: None,
            },
            policies: None,
            distributions: None,
        },
        context,
        "dcat:Catalog".to_string(),
        Some(datasets),
        data_service,
        None,
        None,
    );

    catalog
}

async fn initialize_shared_dsp_catalog() -> Arc<Mutex<Catalog>> {
    let catalog = initiate_dsp_catalog().await;
    Arc::new(Mutex::new(catalog))
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // Shared dsp states to store
    let shared_dsp_catalog_state = initialize_shared_dsp_catalog().await; // Catalog
    let shared_dsp_negotiation_state = Arc::new(Mutex::new(HashMap::new())); // Contract Negotiation
    let shared_dsp_transfer_state = Arc::new(Mutex::new(HashMap::new())); // Transfer Process

    // Dsp routes for catalogs
    let dsp_catalog_route = Router::new()
        .route("/request", post(catalog::get_catalog))
        .route("/datasets/:id", get(catalog::get_dataset))
        .with_state(shared_dsp_catalog_state);

    // Dsp provider routes for negotiations
    let dsp_provider_negotiation_route = Router::new()
        .route("/:pid", get(negotiation_provider::get_negotiation))
        .route("/request", post(negotiation_provider::request_negotiation))
        .route("/:pid/request", post(negotiation_provider::make_offer))
        .route("/:pid/events", post(negotiation_provider::accept_offer))
        .route(
            "/:pid/agreement/verification",
            post(negotiation_provider::verify_agreement),
        )
        .route(
            "/:pid/termination",
            post(negotiation_provider::terminate_negotiation),
        )
        .with_state(shared_dsp_negotiation_state);

    // DSP provider routes for transfer processes
    let dsp_provider_transfer_route = Router::new()
        .route("/:pid", get(tp_provider::get_transfer_process))
        .route("/request", post(tp_provider::request_transfer_processes))
        .route("/:pid/start", post(tp_provider::start_transfer_process))
        .route(
            "/:pid/completion",
            post(tp_provider::complete_transfer_process),
        )
        .route(
            "/:pid/termination",
            post(tp_provider::terminate_transfer_process),
        )
        .route(
            "/:pid/suspension",
            post(tp_provider::suspend_transfer_process),
        )
        .with_state(shared_dsp_transfer_state);

    // Shared management states to store
    let shared_management_catalog_state = initialize_shared_dsp_catalog().await; // Catalog (Management API is using the same dcat Catalog as the DSP Api)
    let shared_management_contract_agreement_state = Arc::new(Mutex::new(HashMap::new())); // Contract Agreement
    let shared_management_contract_definition_state = Arc::new(Mutex::new(HashMap::new())); // Contract Definition
    let shared_management_contract_negotiation_state = Arc::new(Mutex::new(HashMap::new())); // Contract Negotiation
    let shared_management_policy_definition_state = Arc::new(Mutex::new(HashMap::new())); // Policy Definition
    let shared_management_transfer_process_state = Arc::new(Mutex::new(HashMap::new())); // Transfer Process
    let shared_management_asset_state = Arc::new(Mutex::new(HashMap::new())); // Asset

    // Management routes for catalogs
    let management_catalog_route = Router::new()
        // Request a single Dataset
        // `POST /v2/catalog/datasets/request` goes to `request_catalog_dataset`
        .route(
            "/datasets/request",
            post(management_catalog::request_catalog_dataset),
        )
        // Request Catalog Entries
        // `POST /v2/catalog/request` goes to `request_catalog`
        .route("/request", post(management_catalog::request_catalog))
        .with_state(shared_management_catalog_state);

    // Management routes for contract agreements
    let management_contract_agreement_route = Router::new()
        // Request Contract Agreements
        // `POST /v2/contractagreements/request` goes to `request_agreements`
        .route(
            "/request",
            post(management_contract_agreement::request_agreements),
        )
        // Get a Contract Agreement
        // `GET /v2/contractagreements/{id}` goes to `get_agreement`
        .route("/:id", get(management_contract_agreement::get_agreement))
        // Get a Contract Negotiation for a Contract Agreement
        // `GET /v2/contractagreements/{id}/negotiation` goes to `get_agreement_negotiation`
        .route(
            "/:id/negotiation",
            get(management_contract_agreement::get_agreement_negotiation),
        )
        .with_state(shared_management_contract_agreement_state);

    // Management routes for contract definitions
    let management_contract_definitions_route = Router::new()
        // Contract Definitions
        // `PUT /v2/contractdefinitions` goes to `update_contract_definition`
        .route(
            "/",
            put(management_contract_definition::update_contract_definition),
        )
        // `POST /v2/contractdefinitions` goes to `create_contract_definition`
        .route(
            "/",
            post(management_contract_definition::create_contract_definition),
        )
        // `POST /v2/contractdefinitions/request` goes to `request_contract_definition`
        .route(
            "/request",
            post(management_contract_definition::request_contract_definition),
        )
        // `GET /v2/contractdefinitions/{id}` goes to `get_contract_definition`
        .route(
            "/:id",
            get(management_contract_definition::get_contract_definition),
        )
        // `DELETE /v2/contractdefinitions/{id}` goes to `delete_contract_definition`
        .route(
            "/:id",
            delete(management_contract_definition::delete_contract_definition),
        )
        // add shared state to the app
        .with_state(shared_management_contract_definition_state);

    // Management routes for contract negotiations
    let management_contract_negotiations_route = Router::new()
        // Contract Negotiations
        // `POST /v2/contractnegotiations` goes to `initiate_contract_negotiation`
        .route(
            "/",
            post(management_contract_negotiation::initiate_contract_negotiation),
        )
        // `POST /v2/contractnegotiations/request` goes to `request_contract_negotiation`
        .route(
            "/request",
            post(management_contract_negotiation::request_contract_negotiation),
        )
        // `GET /v2/contractnegotiations/{id}` goes to `get_contract_negotiation`
        .route(
            "/:id",
            get(management_contract_negotiation::get_contract_negotiation),
        )
        // `GET /v2/contractnegotiations/{id}/agreement` goes to `get_agreement_by_negotiation_id`
        .route(
            "/:id/agreement",
            get(management_contract_negotiation::get_agreement_by_negotiation_id),
        )
        // `GET /v2/contractnegotiations/{id}/state` goes to `get_negotiation_state`
        .route(
            "/:id/state",
            get(management_contract_negotiation::get_negotiation_state),
        )
        // `POST /v2/contractnegotiations/{id}/terminate` goes to `terminate_contract_negotiation`
        .route(
            "/:id/terminate",
            post(management_contract_negotiation::terminate_contract_negotiation),
        )
        // add shared state to the app
        .with_state(shared_management_contract_negotiation_state);

    // Management routes for policy definitions
    let management_policy_definition_route = Router::new()
        // Create a new Policy Definition
        // `POST /v2/policydefinitions` goes to `create_policy_definition`
        .route(
            "/",
            post(management_policy_definition::create_policy_definition),
        )
        // Request Policy Definitions
        // `POST /v2/policydefinitions/request` goes to `request_policy_definitions`
        .route(
            "/request",
            post(management_policy_definition::request_policy_definitions),
        )
        // Get a Policy Definition
        // `GET /v2/policydefinitions/{id}` goes to `get_policy_definition`
        .route(
            "/:id",
            get(management_policy_definition::get_policy_definition),
        )
        // Update a Policy Definition
        // `PUT /v2/policydefinitions/{id}` goes to `update_policy_definition`
        .route(
            "/:id",
            put(management_policy_definition::update_policy_definition),
        )
        // Delete a Policy Definition
        // `DELETE /v2/policydefinitions/{id}` goes to `delete_policy_definition`
        .route(
            "/:id",
            delete(management_policy_definition::delete_policy_definition),
        )
        .with_state(shared_management_policy_definition_state);

    // Management routes for transfer processes
    let management_transfer_process_route = Router::new()
        // Create a new Transfer Process
        // `POST /v2/transferprocesses` goes to `initiate_transfer_process`
        .route(
            "/",
            post(management_transfer_process::initiate_transfer_process),
        )
        // Request Transfer Processes
        // `POST /v2/transferprocesses/request` goes to `request_transfer_processes`
        .route(
            "/request",
            post(management_transfer_process::request_transfer_processes),
        )
        // Get a Transfer Process
        // `GET /v2/transferprocesses/{id}` goes to `get_transfer_process`
        .route(
            "/:id",
            get(management_transfer_process::get_transfer_process),
        )
        // Deprovision the resources of a Transfer Process
        // `POST /v2/transferprocesses/{id}/deprovision` goes to `deprovision_transfer_process`
        .route(
            "/:id/deprovision",
            post(management_transfer_process::deprovision_transfer_process_resource),
        )
        // Resume a Transfer Process
        // `POST /v2/transferprocesses/{id}/resume` goes to `resume_transfer_process`
        .route(
            "/:id/resume",
            post(management_transfer_process::resume_transfer_process),
        )
        // Get the state of a Transfer Process
        // `GET /v2/transferprocesses/{id}/state` goes to `get_transfer_process_state`
        .route(
            "/:id/state",
            get(management_transfer_process::get_transfer_process_state),
        )
        // Suspend a Transfer Process
        // `POST /v2/transferprocesses/{id}/suspend` goes to `suspend_transfer_process`
        .route(
            "/:id/suspend",
            post(management_transfer_process::suspend_transfer_process),
        )
        // Terminate a Transfer Process
        // `POST /v2/transferprocesses/{id}/terminate` goes to `terminate_transfer_process`
        .route(
            "/:id/terminate",
            post(management_transfer_process::terminate_transfer_process),
        )
        .with_state(shared_management_transfer_process_state);

    // Management routes for assets
    let management_asset_route = Router::new()
        // Update an Asset
        // `PUT /v3/assets` goes to `update_asset`
        .route("/", put(management_asset::update_asset))
        // Create a new Asset
        // `POST /v3/assets` goes to `create_asset`
        .route("/", post(management_asset::create_asset))
        // Request Assets
        // `POST /v3/assets/request` goes to `request_assets`
        .route("/request", post(management_asset::request_assets))
        // Get an Asset
        // `GET /v3/assets/{id}` goes to `get_asset`
        .route("/:id", get(management_asset::get_asset))
        // Delete an Asset
        // `DELETE /v3/assets/{id}` goes to `delete_asset`
        .route("/:id", delete(management_asset::delete_asset))
        .with_state(shared_management_asset_state);

    // create our app by nesting the routes
    let api_routes = Router::new()
        // Management Routes
        .nest("/v2/catalog", management_catalog_route)
        .nest(
            "/v2/contractagreements",
            management_contract_agreement_route,
        )
        .nest(
            "/v2/contractdefinitions",
            management_contract_definitions_route,
        )
        .nest(
            "/v2/contractnegotiations",
            management_contract_negotiations_route,
        )
        .nest("/v2/policydefinitions", management_policy_definition_route)
        .nest("/v2/transferprocesses", management_transfer_process_route)
        .nest("/v3/assets", management_asset_route)
        // DSP Routes
        .nest("/catalog", dsp_catalog_route)
        .nest("/negotiations", dsp_provider_negotiation_route)
        .nest("/transfers", dsp_provider_transfer_route)
        // Unknown Routes
        .route(
            "/*path",
            get(debug_route)
                .post(debug_route)
                .put(debug_route)
                .delete(debug_route),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, api_routes.into_make_service())
        .await
        .unwrap();
}
