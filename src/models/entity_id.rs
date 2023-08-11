use crate::ValidationError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(try_from = "usize")]
pub struct EntityId(usize);

impl EntityId {
    fn new(value: usize) -> Result<Self, ValidationError> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(ValidationError::new("EntityId"))
        }
    }

    pub fn validate(&self) -> bool {
        if self.0 == 0 {
            return false;
        }

        true
    }
}

impl TryFrom<usize> for EntityId {
    type Error = ValidationError;

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
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse()
            .map_err(|_error| ValidationError::new("EntityId"))?;
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
