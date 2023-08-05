use serde::{Deserialize, Serialize};
use std::ops::Deref;

use super::Person;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonList(Vec<Person>);

impl PersonList {
    pub fn new(value: Vec<Person>) -> Self {
        Self(value)
    }
}

impl From<Vec<Person>> for PersonList {
    fn from(value: Vec<Person>) -> Self {
        PersonList::new(value)
    }
}

impl From<PersonList> for Vec<Person> {
    fn from(value: PersonList) -> Self {
        value.0
    }
}

impl Deref for PersonList {
    type Target = Vec<Person>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
