use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonId(usize);

impl PersonId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

impl From<usize> for PersonId {
    fn from(value: usize) -> Self {
        PersonId::new(value)
    }
}

impl From<PersonId> for usize {
    fn from(value: PersonId) -> Self {
        value.0
    }
}

impl Deref for PersonId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
