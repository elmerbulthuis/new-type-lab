use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonId(usize);

impl PersonId {
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

impl TryFrom<usize> for PersonId {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PersonId> for usize {
    fn from(value: PersonId) -> Self {
        value.0
    }
}

#[cfg(feature = "deref")]
impl std::ops::Deref for PersonId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "as_ref")]
impl AsRef<usize> for PersonId {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}
