use odrl::model::policy::OfferPolicy;
use uuid::Uuid;
use odrl::model::party::Party;
use crate::policy::GenericPolicy;

#[derive(Debug)]
pub enum ParsingError {
    InvalidPolicy(String),
}

pub fn optional2original(optional_policy: GenericPolicy) -> Result<OfferPolicy, ParsingError> {

    let uid = optional_policy.uid.unwrap_or(Uuid::new_v4().to_string());

    let assigner = if optional_policy.assigner.is_some() {
        optional_policy.assigner.unwrap()
    } else {
        return Err(ParsingError::InvalidPolicy("Assigner is required".to_string()));
    };

    let orig_assigner = Party {
        uid: None,
        part_of: vec![],
        function: Default::default(),
        party_type: None,
    };

    Ok(OfferPolicy {
        uid,
        assigner: orig_assigner,
        rules: vec![],
        profiles: vec![],
        inherit_from: vec![],
        conflict: None,
        obligation: vec![],
        target: None,
        action: None,
    })

}

pub fn aruna2odrl() {
    todo!()
}