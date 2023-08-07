use serde::{Deserialize, Serialize};
#[cfg(feature = "deref")]
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

#[cfg(feature = "deref")]
impl Deref for PersonName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<str> for PersonName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
