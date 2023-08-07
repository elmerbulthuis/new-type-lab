use serde::{Deserialize, Serialize};
#[cfg(feature = "deref")]
use std::ops::Deref;

use super::Person;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonList(Vec<Person>);

impl PersonList {
    fn new(value: impl IntoIterator<Item = Person>) -> Self {
        Self(value.into_iter().collect())
    }
}

impl<T> From<T> for PersonList
where
    T: IntoIterator<Item = Person>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl From<PersonList> for Vec<Person> {
    fn from(value: PersonList) -> Self {
        value.0
    }
}

#[cfg(feature = "deref")]
impl Deref for PersonList {
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
