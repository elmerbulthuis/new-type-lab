use crate::ValidationError;

use super::{EntityId, EntityName, PersonType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Person {
    pub id: EntityId,
    pub name: EntityName,
    pub r#type: PersonType,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(try_from = "Person")]
pub struct PersonContainer(Person);

impl PersonContainer {
    fn new(value: Person) -> Result<Self, ValidationError> {
        let instance = Self(value);
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
        value.0
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
