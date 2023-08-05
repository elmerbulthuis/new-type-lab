use serde::{Deserialize, Serialize};
use std::ops::Deref;

use super::Person;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonList<'l>(#[serde(borrow)] Vec<Person<'l>>);

impl<'l> PersonList<'l> {
    pub fn new(value: Vec<Person<'l>>) -> Self {
        Self(value)
    }
}

impl<'l> From<Vec<Person<'l>>> for PersonList<'l> {
    fn from(value: Vec<Person<'l>>) -> Self {
        PersonList::new(value)
    }
}

impl<'l> From<PersonList<'l>> for Vec<Person<'l>> {
    fn from(value: PersonList<'l>) -> Self {
        value.0
    }
}

impl<'l> Deref for PersonList<'l> {
    type Target = Vec<Person<'l>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
