use super::{OrganizationContainer, PersonContainer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Entity {
    Organization(OrganizationContainer),
    Person(PersonContainer),
}
