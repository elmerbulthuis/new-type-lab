use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonName(String);

impl PersonName {
    pub fn new(value: impl ToString) -> Self {
        Self(value.to_string())
    }
}

impl<T> From<T> for PersonName
where
    T: ToString,
{
    fn from(value: T) -> Self {
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
