use serde::{Deserialize, Serialize};
#[cfg(feature = "deref")]
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonName(String);

impl PersonName {
    fn new(value: String) -> Result<Self, ()> {
        Ok(Self(value))
    }
}

impl<T> From<T> for PersonName
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self::new(value.to_string()).unwrap()
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
