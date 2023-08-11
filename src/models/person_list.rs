use serde::{Deserialize, Serialize};

use super::Person;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonList(Vec<Person>);

impl PersonList {
    fn new(value: Vec<Person>) -> Result<Self, ()> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(())
        }
    }

    fn validate(&self) -> bool {
        if self.0.is_empty() {
            return false;
        }

        true
    }
}

impl TryFrom<Vec<Person>> for PersonList {
    type Error = ();

    fn try_from(value: Vec<Person>) -> Result<Self, Self::Error> {
        Self::new(value.into_iter().collect())
    }
}

impl From<PersonList> for Vec<Person> {
    fn from(value: PersonList) -> Self {
        value.0
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for PersonList {
    type Target = Vec<Person>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<Vec<Person>> for PersonList {
    fn as_ref(&self) -> &Vec<Person> {
        &self.0
    }
}
