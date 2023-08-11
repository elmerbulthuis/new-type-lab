use std::str::FromStr;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityName(String);

impl EntityName {
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

impl<'de> Deserialize<'de> for EntityName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(EntityNameVisitor)
    }
}

impl TryFrom<String> for EntityName {
    type Error = ();

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
    type Err = ();

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

struct EntityNameVisitor;

impl<'de> Visitor<'de> for EntityNameVisitor {
    type Value = EntityName;

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
