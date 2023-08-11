use std::str::FromStr;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PersonType(String);

impl PersonType {
    fn new(value: String) -> Result<Self, ()> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(())
        }
    }

    fn validate(&self) -> bool {
        if self.0 != "person" {
            return false;
        }

        true
    }
}

impl<'de> Deserialize<'de> for PersonType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(PersonTypeVisitor)
    }
}

impl TryFrom<String> for PersonType {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PersonType> for String {
    fn from(value: PersonType) -> Self {
        value.0
    }
}

impl FromStr for PersonType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl ToString for PersonType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for PersonType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<str> for PersonType {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

struct PersonTypeVisitor;

impl<'de> Visitor<'de> for PersonTypeVisitor {
    type Value = PersonType;

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

    use super::PersonType;

    #[test]
    fn test_1() {
        let serialized = json!("person");
        let deserialized: Result<PersonType, _> = serde_json::from_value(serialized);
        let deserialized = deserialized.unwrap();

        assert_eq!(deserialized.as_ref(), "person");

        let serialized = json!("something else");
        let deserialized: Result<PersonType, _> = serde_json::from_value(serialized);

        assert!(deserialized.is_err());
    }
}
