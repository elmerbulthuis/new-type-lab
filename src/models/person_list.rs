use serde::{Deserialize, Serialize};

use super::PersonContainer;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct PersonList(Vec<PersonContainer>);

impl PersonList {
    fn new(value: Vec<PersonContainer>) -> Result<Self, ()> {
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

impl TryFrom<Vec<PersonContainer>> for PersonList {
    type Error = ();

    fn try_from(value: Vec<PersonContainer>) -> Result<Self, Self::Error> {
        Self::new(value.into_iter().collect())
    }
}

impl From<PersonList> for Vec<PersonContainer> {
    fn from(value: PersonList) -> Self {
        value.0
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for PersonList {
    type Target = Vec<PersonContainer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<Vec<PersonContainer>> for PersonList {
    fn as_ref(&self) -> &Vec<PersonContainer> {
        &self.0
    }
}
