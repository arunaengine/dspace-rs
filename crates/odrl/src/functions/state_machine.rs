use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ConsumerState {
    // Negotiation
    Initial(Initial),       // initial state
    Requesting(Requesting), // if the consumer requests a contract he will send a ContractRequestMessage
    // the ContractRequestMessage might also be a counteroffer
    Requested(Requested),
    Offered(Offered), // can only be reached if a ContractOfferMessage is received
    Accepting(Accepting), // if the consumer accepts the offer he will send a ContractNegotiationEventMessage
    Accepted(Accepted),

    // Agreement
    Agreed(Agreed), // can only be reached if a ContractAgreementMessage is received
    Verifying(Verifying), // if the consumer verifies the agreement he will send a ContractAgreementVerificationMessage
    Verified(Verified),

    // Finalization
    Finalized(Finalized), // can only be reached if a ContractNegotiationEventMessage is received

    // Termination
    Terminating(Terminating), // if the consumer terminates the transfer he will send a TransferTerminationMessage
    Terminated(Terminated),   // final state
}

impl fmt::Display for ConsumerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsumerState::Initial(_) => write!(f, "Initial"),
            ConsumerState::Requesting(_) => write!(f, "Requesting"),
            ConsumerState::Requested(_) => write!(f, "Requested"),
            ConsumerState::Offered(_) => write!(f, "Offered"),
            ConsumerState::Accepting(_) => write!(f, "Accepting"),
            ConsumerState::Accepted(_) => write!(f, "Accepted"),
            ConsumerState::Agreed(_) => write!(f, "Agreed"),
            ConsumerState::Verifying(_) => write!(f, "Verifying"),
            ConsumerState::Verified(_) => write!(f, "Verified"),
            ConsumerState::Finalized(_) => write!(f, "Finalized"),
            ConsumerState::Terminating(_) => write!(f, "Terminating"),
            ConsumerState::Terminated(_) => write!(f, "Terminated"),
        }
    }
}

impl fmt::Display for ProviderState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderState::Initial(_) => write!(f, "Initial"),
            ProviderState::Offering(_) => write!(f, "Offering"),
            ProviderState::Offered(_) => write!(f, "Offered"),
            ProviderState::Requested(_) => write!(f, "Requested"),
            ProviderState::Accepted(_) => write!(f, "Accepted"),
            ProviderState::Agreeing(_) => write!(f, "Agreeing"),
            ProviderState::Agreed(_) => write!(f, "Agreed"),
            ProviderState::Verified(_) => write!(f, "Verified"),
            ProviderState::Finalizing(_) => write!(f, "Finalizing"),
            ProviderState::Finalized(_) => write!(f, "Finalized"),
            ProviderState::Terminating(_) => write!(f, "Terminating"),
            ProviderState::Terminated(_) => write!(f, "Terminated"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProviderState {
    // Negotiation
    Initial(Initial),   // initial state
    Offering(Offering), // if the provider offers a contract he will send a ContractOfferMessage
    // the ContractOfferMessage might also be a counteroffer
    Offered(Offered),
    Requested(Requested), // can only be reached if a ContractRequestMessage is received
    Accepted(Accepted),   // can only be reached if a ContractNegotiationEventMessage is received
    Agreeing(Agreeing),   // if the provider agrees he will send a ContractAgreementMessage
    Agreed(Agreed),

    // Verification and Finalization
    Verified(Verified), // can only be reached if a ContractAgreementVerificationMessage is received
    Finalizing(Finalizing), // if the provider finalizes the agreement he will send a ContractNegotiationEventMessage
    Finalized(Finalized),

    // Termination
    Terminating(Terminating), // if the provider terminates the transfer he will send a TransferTerminationMessage
    Terminated(Terminated),   // final state
}

#[derive(Debug, Clone)]
pub struct NegotiationState {
    pub consumer: String,           // iri
    pub provider: String,           // iri
    pub state: String,              // Current state of the state machine
    pub message: String,            // Current state of the negotiation
    pub timestamp: DateTime<Local>, // Timestamp to ensure a history of the negotiation
}

impl NegotiationState {
    pub fn new(
        consumer: String,
        provider: String,
        state: String,
        message: String,
        timestamp: DateTime<Local>,
    ) -> Self {
        NegotiationState {
            consumer,
            provider,
            state,
            message,
            timestamp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsumerStateMachine<CS> {
    pub state: CS,
    pub iri: String,
    pub negotiation_partner: String,
}

impl ConsumerStateMachine<ConsumerState> {
    pub fn new(c_iri: &str, p_iri: &str) -> Self {
        ConsumerStateMachine {
            state: ConsumerState::Initial(Initial::new()),
            iri: c_iri.to_string(),
            negotiation_partner: p_iri.to_string(),
        }
    }

    // Method to transition to a certain state if the consumer receives messages
    fn transition_to_state(&mut self, state: ConsumerState) {
        self.state = state.clone();
        println!("Consumer state transitioned to {:?}", self.state);
    }

    pub fn transition_to_requesting(&mut self, msg: &str) {
        // Check if the current state is Initial or Offered
        match &self.state {
            ConsumerState::Initial(_) => {
                // Update the state machine's state to Requesting
                self.transition_to_state(ConsumerState::Requesting(Requesting::new(
                    msg,
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            ConsumerState::Offered(_) => {
                // Counteroffer
                // Update the state machine's state to Requesting
                self.transition_to_state(ConsumerState::Requesting(Requesting::new(
                    msg,
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Cannot transition to Requesting from current state"),
        }
    }

    pub fn transition_to_requested(&mut self, msg: &str) {
        // Check if the current state is Requesting
        match &self.state {
            ConsumerState::Requesting(_) => {
                // Update the state machine's state to Requested
                self.transition_to_state(ConsumerState::Requested(Requested::new(
                    msg,
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Cannot transition to Requested from current state"),
        }
    }

    pub fn transition_to_accepting(&mut self, msg: &str) {
        // Check if the current state is Offered
        match &self.state {
            ConsumerState::Offered(_) => {
                // Update the state machine's state to Accepting
                self.transition_to_state(ConsumerState::Accepting(Accepting::new(
                    msg,
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Cannot transition to Accepting from current state"),
        }
    }

    pub fn transition_to_accepted(&mut self, msg: &str) {
        // Check if the current state is Accepting
        match &self.state {
            ConsumerState::Accepting(_) => {
                // Update the state machine's state to Accepted
                self.transition_to_state(ConsumerState::Accepted(Accepted::new(
                    msg,
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Cannot transition to Accepted from current state"),
        }
    }
    pub fn transition_to_verifying(&mut self, msg: &str) {
        // Check if the current state is Agreed
        match &self.state {
            ConsumerState::Agreed(_) => {
                // Update the state machine's state to Verifying
                self.transition_to_state(ConsumerState::Verifying(Verifying::new(
                    msg,
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Cannot transition to Verifying from current state"),
        }
    }

    pub fn transition_to_verified(&mut self, msg: &str) {
        // Check if the current state is Verifying
        match &self.state {
            ConsumerState::Verifying(_) => {
                // Update the state machine's state to Verified
                self.transition_to_state(ConsumerState::Verified(Verified::new(
                    msg,
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Cannot transition to Verified from current state"),
        }
    }

    pub fn transition_to_terminating(&mut self, msg: &str) {
        // Check if the current state is Finalized
        // TODO: Differentiate between termination since the negotiation is finalized and a termination because of an error or since it was requested by the consumer
        match &self.state {
            ConsumerState::Finalized(_) => {
                // Update the state machine's state to Terminating
                self.transition_to_state(ConsumerState::Terminating(Terminating::new(
                    msg,
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => self.transition_to_state(ConsumerState::Terminating(Terminating::new(
                msg,
                self.iri.as_str(),
                self.negotiation_partner.as_str(),
            ))),
        }
    }
    pub fn transition_to_terminated(&mut self, msg: &str) {
        // Check if the current state is Terminating
        match &self.state {
            ConsumerState::Terminating(_) => {
                // Update the state machine's state to Terminated
                self.transition_to_state(ConsumerState::Terminated(Terminated::new(
                    msg,
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Cannot transition to Terminated from current state"),
        }
    }

    // Method to send a contract request message to the provider
    pub fn send_contract_request(
        &self,
        provider: &mut ProviderStateMachine<ProviderState>,
        contract_request_message: String,
    ) {
        provider.receive_contract_request(contract_request_message);
    }

    // Method to send a contract negotiation event message to the provider
    pub fn send_contract_negotiation_event(
        &self,
        provider: &mut ProviderStateMachine<ProviderState>,
        contract_negotiation_event_message: String,
    ) {
        provider.receive_contract_negotiation_event(contract_negotiation_event_message);
    }

    // Method to send a contract agreement verification message to the provider
    pub fn send_contract_agreement_verification(
        &self,
        provider: &mut ProviderStateMachine<ProviderState>,
        contract_agreement_verification_message: String,
    ) {
        provider.receive_contract_agreement_verification(contract_agreement_verification_message);
    }

    // Method to send a transfer termination message to the provider
    pub fn send_transfer_termination(
        &self,
        provider: &mut ProviderStateMachine<ProviderState>,
        transfer_termination_message: String,
    ) {
        provider.receive_transfer_termination(transfer_termination_message);
    }

    pub fn receive_contract_offer(&mut self, contract_offer_message: String) {
        match &mut self.state {
            // If the consumer is in the Initial or Requested state, he can receive a contract offer
            // and transition to the Offered state
            // if the consumer is in the Requesting state, he should not receive a contract offer
            // because he is waiting for the provider to accept or decline the request
            ConsumerState::Initial(_) => {
                println!("Consumer received contract offer");
                self.transition_to_state(ConsumerState::Offered(Offered::new(
                    contract_offer_message.as_str(),
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            ConsumerState::Requested(_) => {
                println!("Consumer received contract offer");
                self.transition_to_state(ConsumerState::Offered(Offered::new(
                    contract_offer_message.as_str(),
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Consumer cannot receive contract offer in current state"),
        }
    }

    pub fn receive_contract_agreement(&mut self, contract_agreement_message: String) {
        match &mut self.state {
            // If the consumer is in the Accepted or Requested state, he can receive a contract agreement
            // and transition to the Agreed state
            ConsumerState::Accepted(_) => {
                println!("Consumer received contract agreement");
                self.transition_to_state(ConsumerState::Agreed(Agreed::new(
                    contract_agreement_message.as_str(),
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            ConsumerState::Requested(_) => {
                println!("Consumer received contract agreement");
                self.transition_to_state(ConsumerState::Agreed(Agreed::new(
                    contract_agreement_message.as_str(),
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Consumer cannot receive contract agreement in current state"),
        }
    }

    pub fn receive_contract_negotiation_event(
        &mut self,
        contract_negotiation_event_message: String,
    ) {
        match &mut self.state {
            // If the consumer is in the Verified state, he can receive a contract negotiation event message
            // and transition to the Finalized state when the provider has finalized the contract
            ConsumerState::Verified(_) => {
                println!("Consumer received contract negotiation event message");
                self.transition_to_state(ConsumerState::Finalized(Finalized::new(
                    contract_negotiation_event_message.as_str(),
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!(
                "Consumer cannot receive contract negotiation event message in current state"
            ),
        }
    }

    pub fn receive_transfer_termination(&mut self, transfer_termination_message: String) {
        match &mut self.state {
            // If the consumer is in the Finalized state, he can receive a transfer termination message
            // and transition to the Terminated state
            ConsumerState::Finalized(_) => {
                println!("Consumer received transfer termination message");
                self.transition_to_state(ConsumerState::Terminated(Terminated::new(
                    transfer_termination_message.as_str(),
                    self.iri.as_str(),
                    self.negotiation_partner.as_str(),
                )));
            }
            _ => println!("Consumer cannot receive transfer termination message in current state"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProviderStateMachine<PS> {
    pub state: PS,
    pub iri: String,
    pub negotiation_partner: String,
}

impl ProviderStateMachine<ProviderState> {
    pub fn new(c_iri: &str, p_iri: &str) -> Self {
        ProviderStateMachine {
            state: ProviderState::Initial(Initial::new()),
            iri: c_iri.to_string(),
            negotiation_partner: p_iri.to_string(),
        }
    }

    // Method to transition to a certain state if the provider receives messages
    fn transition_to_state(&mut self, state: ProviderState) {
        self.state = state.clone();
        println!("Provider state transitioned to {:?}", self.state);
    }

    pub fn transition_to_offering(&mut self, msg: &str) {
        // Check if the current state is Initial or Requested
        match &self.state {
            ProviderState::Initial(_) => {
                // Update the state machine's state to Offering
                self.transition_to_state(ProviderState::Offering(Offering::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            ProviderState::Requested(_) => {
                // Counteroffer
                // Update the state machine's state to Offering
                self.transition_to_state(ProviderState::Offering(Offering::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!("Cannot transition to Offering from current state"),
        }
    }

    pub fn transition_to_offered(&mut self, msg: &str) {
        // Check if the current state is Offering
        match &self.state {
            ProviderState::Offering(_) => {
                // Update the state machine's state to Offered
                self.transition_to_state(ProviderState::Offered(Offered::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!("Cannot transition to Offered from current state"),
        }
    }

    pub fn transition_to_agreeing(&mut self, msg: &str) {
        // Check if the current state is Accepted or Requested
        match &self.state {
            ProviderState::Accepted(_) => {
                // Update the state machine's state to Agreeing
                self.transition_to_state(ProviderState::Agreeing(Agreeing::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            ProviderState::Requested(_) => {
                // Update the state machine's state to Agreeing
                self.transition_to_state(ProviderState::Agreeing(Agreeing::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!("Cannot transition to Agreeing from current state"),
        }
    }

    pub fn transition_to_agreed(&mut self, msg: &str) {
        // Check if the current state is Agreeing
        match &self.state {
            ProviderState::Agreeing(_) => {
                // Update the state machine's state to Agreed
                self.transition_to_state(ProviderState::Agreed(Agreed::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!("Cannot transition to Agreed from current state"),
        }
    }

    pub fn transition_to_finalizing(&mut self, msg: &str) {
        // Check if the current state is Verified
        match &self.state {
            ProviderState::Verified(_) => {
                // Update the state machine's state to Finalizing
                self.transition_to_state(ProviderState::Finalizing(Finalizing::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!("Cannot transition to Finalizing from current state"),
        }
    }

    pub fn transition_to_finalized(&mut self, msg: &str) {
        // Check if the current state is Finalizing
        match &self.state {
            ProviderState::Finalizing(_) => {
                // Update the state machine's state to Finalized
                self.transition_to_state(ProviderState::Finalized(Finalized::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!("Cannot transition to Finalized from current state"),
        }
    }

    pub fn transition_to_terminating(&mut self, msg: &str) {
        // Check if the current state is Finalized
        // TODO: Differentiate between termination since the negotiation is finalized and a termination because of an error or since it was requested by the provider
        match &self.state {
            ProviderState::Finalized(_) => {
                // Update the state machine's state to Terminating
                self.transition_to_state(ProviderState::Terminating(Terminating::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => self.transition_to_state(ProviderState::Terminating(Terminating::new(
                msg,
                self.negotiation_partner.as_str(),
                self.iri.as_str(),
            ))),
        }
    }

    pub fn transition_to_terminated(&mut self, msg: &str) {
        // Check if the current state is Terminating
        match &self.state {
            ProviderState::Terminating(_) => {
                // Update the state machine's state to Terminated
                self.transition_to_state(ProviderState::Terminated(Terminated::new(
                    msg,
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!("Cannot transition to Terminated from current state"),
        }
    }

    pub fn send_contract_offer(
        &self,
        consumer: &mut ConsumerStateMachine<ConsumerState>,
        contract_offer_message: String,
    ) {
        consumer.receive_contract_offer(contract_offer_message);
    }

    pub fn send_contract_agreement(
        &self,
        consumer: &mut ConsumerStateMachine<ConsumerState>,
        contract_agreement_message: String,
    ) {
        consumer.receive_contract_agreement(contract_agreement_message);
    }

    pub fn send_contract_negotiation_event(
        &self,
        consumer: &mut ConsumerStateMachine<ConsumerState>,
        contract_negotiation_event_message: String,
    ) {
        consumer.receive_contract_negotiation_event(contract_negotiation_event_message);
    }

    pub fn send_transfer_termination(
        &self,
        consumer: &mut ConsumerStateMachine<ConsumerState>,
        transfer_termination_message: String,
    ) {
        consumer.receive_transfer_termination(transfer_termination_message);
    }

    // Method to receive a contract request message from the consumer
    pub fn receive_contract_request(&mut self, contract_request_message: String) {
        match &mut self.state {
            // If the provider is in the Initial or Offered state, he can receive a contract request
            // and transition to the Requested state
            // if the provider is in the Offering state, he should not receive a contract request
            // because he is waiting for the consumer to accept or decline the offer
            ProviderState::Initial(_) => {
                println!("Provider received contract request");
                self.transition_to_state(ProviderState::Requested(Requested::new(
                    contract_request_message.as_str(),
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            ProviderState::Offered(_) => {
                println!("Provider received contract request");
                self.transition_to_state(ProviderState::Requested(Requested::new(
                    contract_request_message.as_str(),
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!("Provider cannot receive contract request in current state"),
        }
    }

    // Method to receive a contract negotiation event message from the consumer
    pub fn receive_contract_negotiation_event(
        &mut self,
        contract_negotiation_event_message: String,
    ) {
        match &mut self.state {
            // If the provider is in the Offered state, he can receive a contract negotiation event message
            // and transition to the Accepted state when the consumer accepts the offer
            ProviderState::Offered(_) => {
                println!("Provider received contract negotiation event message");
                self.transition_to_state(ProviderState::Accepted(Accepted::new(
                    contract_negotiation_event_message.as_str(),
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!(
                "Provider cannot receive contract negotiation event message in current state"
            ),
        }
    }

    // Method to receive a contract agreement verification message from the consumer
    pub fn receive_contract_agreement_verification(
        &mut self,
        contract_agreement_verification_message: String,
    ) {
        match &mut self.state {
            // If the provider is in the Agreed state, he can receive a contract agreement verification message
            // and transition to the Verified state
            ProviderState::Agreed(_) => {
                println!("Provider received contract agreement verification message");
                self.transition_to_state(ProviderState::Verified(Verified::new(
                    contract_agreement_verification_message.as_str(),
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!(
                "Provider cannot receive contract agreement verification message in current state"
            ),
        }
    }

    // Method to receive a transfer termination message from the consumer
    pub fn receive_transfer_termination(&mut self, transfer_termination_message: String) {
        match &mut self.state {
            // If the provider is in the Finalized state, he can receive a transfer termination message
            // and transition to the Terminated state
            ProviderState::Finalized(_) => {
                println!("Provider received transfer termination message");
                self.transition_to_state(ProviderState::Terminated(Terminated::new(
                    transfer_termination_message.as_str(),
                    self.negotiation_partner.as_str(),
                    self.iri.as_str(),
                )));
            }
            _ => println!("Provider cannot receive transfer termination message in current state"),
        }
    }
}

// initial state of both state machines
// CS & PS
#[derive(Debug, Clone)]
pub struct Initial {
    // Specify the values
}

impl Initial {
    pub fn new() -> Self {
        Initial {
            // Specify the values
        }
    }
}

// the state of the consumer state machine when the consumer requests a contract
// CS
#[derive(Debug, Clone)]
pub struct Requesting {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Requesting {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Requesting {
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Requesting".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of the provider state machine when the provider offers a contract
// PS
#[derive(Debug, Clone)]
pub struct Offering {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Offering {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Offering {
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Offering".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of both state machines when the consumer has requested a contract
// CS & PS
#[derive(Debug, Clone)]
pub struct Requested {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Requested {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Requested {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Requested".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of both state machines when the provider has offered a contract
// CS & PS
#[derive(Debug, Clone)]
pub struct Offered {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Offered {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Offered {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Offered".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of the consumer state machine when the consumer accepts the offer
// CS
#[derive(Debug, Clone)]
pub struct Accepting {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Accepting {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Accepting {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Accepting".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of both state machines when the consumer has accepted the offer
// CS & PS
#[derive(Debug, Clone)]
pub struct Accepted {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Accepted {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Accepted {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Accepted".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of the provider state machine when the provider agrees to the contract
// PS
#[derive(Debug, Clone)]
pub struct Agreeing {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Agreeing {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Agreeing {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Agreeing".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of both state machines when the provider has agreed to the contract
// CS & PS
#[derive(Debug, Clone)]
pub struct Agreed {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Agreed {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Agreed {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Agreed".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of the consumer state machine when the consumer verifies the agreement
// CS
#[derive(Debug, Clone)]
pub struct Verifying {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Verifying {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Verifying {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Verifying".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of both state machines when the consumer has verified the agreement
// CS & PS
#[derive(Debug, Clone)]
pub struct Verified {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Verified {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Verified {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Verified".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of the provider state machine when the provider finalizes the agreement
// PS
#[derive(Debug, Clone)]
pub struct Finalizing {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Finalizing {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Finalizing {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Finalizing".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of both state machines when the provider has finalized the agreement
// CS & PS
#[derive(Debug, Clone)]
pub struct Finalized {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Finalized {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Finalized {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Finalized".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the state of the respective state machine when one of the parties terminates the transfer
// CS & PS
#[derive(Debug, Clone)]
pub struct Terminating {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Terminating {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Terminating {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Terminating".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

// the final state of both state machines
// CS & PS
#[derive(Debug, Clone)]
pub struct Terminated {
    // Specify the values
    pub negotiation_state: NegotiationState,
}

impl Terminated {
    pub fn new(negotiation_message: &str, c_iri: &str, p_iri: &str) -> Self {
        Terminated {
            // Specify the values
            negotiation_state: NegotiationState::new(
                c_iri.to_string(),
                p_iri.to_string(),
                "Terminated".to_string(),
                negotiation_message.to_string(),
                Local::now(),
            ),
        }
    }
}

pub fn main() {
    let mut simple_consumer_fsm = ConsumerStateMachine {
        state: ConsumerState::Initial(Initial::new()),
        iri: "http://example.com/consumer".to_string(),
        negotiation_partner: "http://example.com/provider".to_string(),
    };
    let mut simple_provider_fsm = ProviderStateMachine {
        state: ProviderState::Initial(Initial::new()),
        iri: "http://example.com/provider".to_string(),
        negotiation_partner: "http://example.com/consumer".to_string(),
    };
    println!("Consumer State Machine: {:#?}", simple_consumer_fsm);
    println!("Provider State Machine: {:#?}", simple_provider_fsm);

    // Consumer starting a request
    simple_consumer_fsm.transition_to_requesting("Requesting contract");
    println!("Consumer State Machine: {:#?}", simple_consumer_fsm);
    println!("Provider State Machine: {:#?}", simple_provider_fsm);
    simple_consumer_fsm.send_contract_request(
        &mut simple_provider_fsm,
        "Contract request message".to_string(),
    );
    simple_consumer_fsm.transition_to_requested("Requested contract");
    println!("Consumer State Machine: {:#?}", simple_consumer_fsm);
    println!("Provider State Machine: {:#?}", simple_provider_fsm);

    // Provider counter offering
    simple_provider_fsm.transition_to_offering("Offering contract");
    simple_provider_fsm.send_contract_offer(
        &mut simple_consumer_fsm,
        "Contract offer message".to_string(),
    );
    println!("Consumer State Machine: {:#?}", simple_consumer_fsm);
    println!("Provider State Machine: {:#?}", simple_provider_fsm);
    simple_provider_fsm.transition_to_offered("Offered contract");

    // Consumer accepting the offer
    simple_consumer_fsm.transition_to_accepting("Accepting contract");
    simple_consumer_fsm.send_contract_negotiation_event(
        &mut simple_provider_fsm,
        "Contract negotiation event message".to_string(),
    );
    println!("Consumer State Machine: {:#?}", simple_consumer_fsm);
    println!("Provider State Machine: {:#?}", simple_provider_fsm);
    simple_consumer_fsm.transition_to_accepted("Accepted contract");

    // Provider agreeing to the contract
    simple_provider_fsm.transition_to_agreeing("Agreeing contract");
    simple_provider_fsm.send_contract_agreement(
        &mut simple_consumer_fsm,
        "Contract agreement message".to_string(),
    );
    println!("Consumer State Machine: {:#?}", simple_consumer_fsm);
    println!("Provider State Machine: {:#?}", simple_provider_fsm);
    simple_provider_fsm.transition_to_agreed("Agreed contract");

    // Consumer verifying the agreement
    simple_consumer_fsm.transition_to_verifying("Verifying agreement");
    simple_consumer_fsm.send_contract_agreement_verification(
        &mut simple_provider_fsm,
        "Contract agreement verification message".to_string(),
    );
    println!("Consumer State Machine: {:#?}", simple_consumer_fsm);
    println!("Provider State Machine: {:#?}", simple_provider_fsm);
    simple_consumer_fsm.transition_to_verified("Verified agreement");

    // Provider finalizing the agreement
    simple_provider_fsm.transition_to_finalizing("Finalizing agreement");
    simple_provider_fsm.send_contract_negotiation_event(
        &mut simple_consumer_fsm,
        "Contract negotiation event message".to_string(),
    );
    println!("Consumer State Machine: {:#?}", simple_consumer_fsm);
    println!("Provider State Machine: {:#?}", simple_provider_fsm);
    simple_provider_fsm.transition_to_finalized("Finalized agreement");

    // Consumer terminating the transfer
    simple_consumer_fsm.transition_to_terminating("Terminating transfer");
    simple_consumer_fsm.send_transfer_termination(
        &mut simple_provider_fsm,
        "Transfer termination message".to_string(),
    );
    println!("Consumer State Machine: {:#?}", simple_consumer_fsm);
    println!("Provider State Machine: {:#?}", simple_provider_fsm);
}
