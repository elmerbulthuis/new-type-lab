use crate::ValidationError;

use super::{EntityId, EntityName, OrganizationType, PersonContainer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Organization {
    pub r#type: OrganizationType,
    pub id: EntityId,
    pub name: EntityName,
    pub owner: PersonContainer,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(try_from = "Organization")]
pub struct OrganizationContainer(Box<Organization>);

impl OrganizationContainer {
    fn new(value: Organization) -> Result<Self, ValidationError> {
        let instance = Self(Box::new(value));
        if instance.validate() {
            Ok(instance)
        } else {
            Err(ValidationError::new("Organization"))
        }
    }

    fn validate(&self) -> bool {
        true
    }
}

impl TryFrom<Organization> for OrganizationContainer {
    type Error = ValidationError;

    fn try_from(value: Organization) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OrganizationContainer> for Organization {
    fn from(value: OrganizationContainer) -> Self {
        *value.0
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for OrganizationContainer {
    type Target = Organization;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<Organization> for OrganizationContainer {
    fn as_ref(&self) -> &Organization {
        &self.0
    }
}
