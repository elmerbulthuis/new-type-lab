use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::ValidationError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct OrganizationType(String);

impl OrganizationType {
    fn new(value: String) -> Result<Self, ValidationError> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(ValidationError::new("OrganizationType"))
        }
    }

    fn validate(&self) -> bool {
        if self.0 != "organization" {
            return false;
        }

        true
    }
}

impl TryFrom<String> for OrganizationType {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OrganizationType> for String {
    fn from(value: OrganizationType) -> Self {
        value.0
    }
}

impl FromStr for OrganizationType {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl ToString for OrganizationType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for OrganizationType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<str> for OrganizationType {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::OrganizationType;

    #[test]
    fn test_1() {
        let serialized = json!("organization");
        let deserialized: Result<OrganizationType, _> = serde_json::from_value(serialized);
        let deserialized = deserialized.unwrap();

        assert_eq!(deserialized.as_ref(), "organization");

        let serialized = json!("something else");
        let deserialized: Result<OrganizationType, _> = serde_json::from_value(serialized);

        assert!(deserialized.is_err());
    }
}
