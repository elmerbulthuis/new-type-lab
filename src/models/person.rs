use super::{PersonId, PersonName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Person<'l> {
    pub id: PersonId,
    #[serde(borrow)]
    pub name: PersonName<'l>,
}
