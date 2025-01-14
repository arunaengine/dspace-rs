pub mod action;
pub use self::action::Action;
pub mod asset;
pub use self::asset::Asset;
pub use self::asset::AssetCollection;
pub mod conflict_term;
pub use self::conflict_term::ConflictTerm;
pub mod constraint;
pub use self::constraint::Constraint;
pub use self::constraint::LogicalConstraint;
pub mod party;
pub use self::party::Party;
pub use self::party::PartyCollection;
pub mod policy;
pub use self::policy::Policy;
pub use self::policy::SetPolicy;
pub use self::policy::OfferPolicy;
pub use self::policy::AgreementPolicy;
pub mod rule;
pub use self::rule::Rule;
pub use self::rule::Permission;
pub use self::rule::Prohibition;
pub use self::rule::Duty;
pub use self::rule::Obligation;
pub mod type_alias;
pub use self::type_alias::IRI;

pub enum StringOrX {
    String(String),
    Target(Asset),
    Profile(Vec<IRI>)
}