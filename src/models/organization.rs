use super::{OrganizationId, OrganizationName, OrganizationType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: OrganizationName,
    pub r#type: OrganizationType,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct OrganizationContainer(Organization);

impl OrganizationContainer {
    fn new(value: Organization) -> Result<Self, ()> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(())
        }
    }

    fn validate(&self) -> bool {
        true
    }
}

impl TryFrom<Organization> for OrganizationContainer {
    type Error = ();

    fn try_from(value: Organization) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OrganizationContainer> for Organization {
    fn from(value: OrganizationContainer) -> Self {
        value.0
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
