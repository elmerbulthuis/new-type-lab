use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonName(String);

impl PersonName {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PersonName {
    fn from(value: &str) -> Self {
        PersonName::new(value.to_string())
    }
}

impl From<String> for PersonName {
    fn from(value: String) -> Self {
        PersonName::new(value)
    }
}

impl From<PersonName> for String {
    fn from(value: PersonName) -> Self {
        value.0
    }
}

impl Deref for PersonName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
