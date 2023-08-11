use crate::ValidationError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct EntityName(String);

impl EntityName {
    fn new(value: String) -> Result<Self, ValidationError> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(ValidationError::new("EntityName"))
        }
    }

    fn validate(&self) -> bool {
        if self.0.is_empty() {
            return false;
        }

        true
    }
}

impl TryFrom<String> for EntityName {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<EntityName> for String {
    fn from(value: EntityName) -> Self {
        value.0
    }
}

impl FromStr for EntityName {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl ToString for EntityName {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for EntityName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<str> for EntityName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::EntityName;

    #[test]
    fn test_1() {
        let serialized = json!("a");
        let deserialized: Result<EntityName, _> = serde_json::from_value(serialized);
        let deserialized = deserialized.unwrap();

        assert_eq!(deserialized.as_ref(), "a");

        let serialized = json!("");
        let deserialized: Result<EntityName, _> = serde_json::from_value(serialized);

        assert!(deserialized.is_err());
    }
}
