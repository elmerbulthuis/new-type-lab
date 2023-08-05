use super::{PersonId, PersonName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: PersonId,
    pub name: PersonName,
}
