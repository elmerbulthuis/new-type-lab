use std::str::FromStr;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(usize);

impl EntityId {
    fn new(value: usize) -> Result<Self, ()> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(())
        }
    }

    pub fn validate(&self) -> bool {
        if self.0 == 0 {
            return false;
        }

        true
    }
}

impl<'de> Deserialize<'de> for EntityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u64(EntityIdVisitor)
    }
}

impl TryFrom<usize> for EntityId {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<EntityId> for usize {
    fn from(value: EntityId) -> Self {
        value.0
    }
}

impl FromStr for EntityId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse().map_err(|_error| ())?;
        Self::new(value)
    }
}

impl ToString for EntityId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for EntityId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<usize> for EntityId {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

struct EntityIdVisitor;

impl<'de> Visitor<'de> for EntityIdVisitor {
    type Value = EntityId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expecting entity id")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v: usize = v
            .try_into()
            .map_err(|_| E::custom("entity name deserialization failed"))?;

        v.try_into()
            .map_err(|_| E::custom("entity name deserialization failed"))
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::EntityId;

    #[test]
    fn test_1() {
        let serialized = json!(1);
        let deserialized: Result<EntityId, _> = serde_json::from_value(serialized);
        let deserialized = deserialized.unwrap();

        assert_eq!(*deserialized.as_ref(), 1);

        let serialized = json!(0);
        let deserialized: Result<EntityId, _> = serde_json::from_value(serialized);

        assert!(deserialized.is_err());
    }
}
