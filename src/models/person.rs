use crate::ValidationError;

use super::{EntityId, EntityName, OrganizationContainer, PersonType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Person {
    pub r#type: PersonType,
    pub id: EntityId,
    pub name: EntityName,
    pub organization: Option<OrganizationContainer>,
    pub parent: Option<PersonContainer>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(try_from = "Person")]
pub struct PersonContainer(Box<Person>);

impl PersonContainer {
    fn new(value: Person) -> Result<Self, ValidationError> {
        let instance = Self(Box::new(value));
        if instance.validate() {
            Ok(instance)
        } else {
            Err(ValidationError::new("Person"))
        }
    }

    fn validate(&self) -> bool {
        true
    }
}

impl TryFrom<Person> for PersonContainer {
    type Error = ValidationError;

    fn try_from(value: Person) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PersonContainer> for Person {
    fn from(value: PersonContainer) -> Self {
        *value.0
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for PersonContainer {
    type Target = Person;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<Person> for PersonContainer {
    fn as_ref(&self) -> &Person {
        &self.0
    }
}
