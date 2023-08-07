use serde::{Deserialize, Serialize};
#[cfg(feature = "deref")]
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

#[cfg(feature = "deref")]
impl Deref for PersonId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<usize> for PersonId {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}
