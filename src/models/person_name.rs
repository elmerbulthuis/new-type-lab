use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonName(String);

impl PersonName {
    fn new(value: String) -> Result<Self, ()> {
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

impl TryFrom<String> for PersonName {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PersonName> for String {
    fn from(value: PersonName) -> Self {
        value.0
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for PersonName {
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
