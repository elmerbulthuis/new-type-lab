use serde::{Deserialize, Serialize};

use super::entity::Entity;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct EntityList(Vec<Entity>);

impl EntityList {
    fn new(value: Vec<Entity>) -> Result<Self, ()> {
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

impl TryFrom<Vec<Entity>> for EntityList {
    type Error = ();

    fn try_from(value: Vec<Entity>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<EntityList> for Vec<Entity> {
    fn from(value: EntityList) -> Self {
        value.0
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for EntityList {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<Vec<Entity>> for EntityList {
    fn as_ref(&self) -> &Vec<Entity> {
        &self.0
    }
}
