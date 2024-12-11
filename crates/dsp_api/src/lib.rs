#[macro_use]
extern crate serde_derive;

pub mod common {
    pub mod dspace_version;
    pub use self::dspace_version::DspaceVersion;
    pub use self::dspace_version::ProtocolVersion;
}

pub mod catalog {
    pub mod catalog_request_message;
    pub use self::catalog_request_message::CatalogRequestMessage;
    pub mod dataset_request_message;
    pub use self::dataset_request_message::DatasetRequestMessage;
    pub mod catalog;
    pub use self::catalog::Catalog;
    pub mod dataset;
    pub use self::dataset::AbstractDataset;
    pub use self::dataset::DataService;
    pub use self::dataset::Dataset;
    pub use self::dataset::Distribution;
    pub use self::dataset::MultiLanguage;
    pub use self::dataset::Reference;
    pub use self::dataset::Resource;
    pub mod catalog_error;
    pub use self::catalog_error::CatalogError;
}

pub mod contract_negotiation {
    pub mod contract_request_message;
    pub use self::contract_request_message::ContractRequestMessage;
    pub mod contract_offer_message;
    pub use self::contract_offer_message::ContractOfferMessage;
    pub mod contract_agreement_message;
    pub use self::contract_agreement_message::ContractAgreementMessage;
    pub mod contract_agreement_verification_message;
    pub use self::contract_agreement_verification_message::ContractAgreementVerificationMessage;
    pub mod contract_negotiation_event_message;
    pub use self::contract_negotiation_event_message::ContractNegotiationEventMessage;
    pub mod contract_negotiation_termination_message;
    pub use self::contract_negotiation_termination_message::ContractNegotiationTerminationMessage;
    pub mod contract_negotiation;
    pub use self::contract_negotiation::ContractNegotiation;
    pub mod contract_negotiation_error;
    pub use self::contract_negotiation_error::ContractNegotiationError;
    pub mod contract;
    pub use self::contract::AbstractPolicyRule;
    pub use self::contract::Action;
    pub use self::contract::Agreement;
    pub use self::contract::Constraint;
    pub use self::contract::Duty;
    pub use self::contract::LeftOperand;
    pub use self::contract::MessageOffer;
    pub use self::contract::Offer;
    pub use self::contract::Operator;
    pub use self::contract::Permission;
    pub use self::contract::Policy;
    pub use self::contract::PolicyClass;
    pub use self::contract::Reference;
    pub use self::contract::RightOperand;
    pub use self::contract::RuleClass;
    pub use self::contract::Target;
}

pub mod transfer_process {
    pub mod transfer_request_message;
    pub use self::transfer_request_message::DataAddress;
    pub use self::transfer_request_message::EndpointProperty;
    pub use self::transfer_request_message::TransferRequestMessage;
    pub mod transfer_start_message;
    pub use self::transfer_start_message::TransferStartMessage;
    pub mod transfer_suspension_message;
    pub use self::transfer_suspension_message::TransferSuspendMessage;
    pub mod transfer_completion_message;
    pub use self::transfer_completion_message::TransferCompletionMessage;
    pub mod transfer_termination_message;
    pub use self::transfer_termination_message::TransferTerminationMessage;
    pub mod transfer_process;
    pub use self::transfer_process::TransferProcess;
    pub use self::transfer_process::TransferState;
    pub mod transfer_error;
    pub use self::transfer_error::TransferError;
}
