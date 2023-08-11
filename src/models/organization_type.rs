use std::str::FromStr;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OrganizationType(String);

impl OrganizationType {
    fn new(value: String) -> Result<Self, ()> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(())
        }
    }

    fn validate(&self) -> bool {
        if self.0 != "organization" {
            return false;
        }

        true
    }
}

impl<'de> Deserialize<'de> for OrganizationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(OrganizationTypeVisitor)
    }
}

impl TryFrom<String> for OrganizationType {
    type Error = ();

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
    type Err = ();

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

struct OrganizationTypeVisitor;

impl<'de> Visitor<'de> for OrganizationTypeVisitor {
    type Value = OrganizationType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expecting entity name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v = v.to_string();
        v.try_into()
            .map_err(|_| E::custom("entity name deserialization failed"))
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
