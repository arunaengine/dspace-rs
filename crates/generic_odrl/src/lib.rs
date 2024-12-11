#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod policy;
mod generics;

use self::policy::Policy;