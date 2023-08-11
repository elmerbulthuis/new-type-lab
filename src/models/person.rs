use super::{PersonId, PersonName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonContainer(Person);

impl PersonContainer {
    fn new(value: Person) -> Result<Self, ()> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(())
        }
    }

    fn validate(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: PersonId,
    pub name: PersonName,
}

impl TryFrom<Person> for PersonContainer {
    type Error = ();

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
