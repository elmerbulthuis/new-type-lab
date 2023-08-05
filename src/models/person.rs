use super::{PersonId, PersonName};

pub struct Person<'l> {
    pub id: PersonId,
    pub name: PersonName<'l>,
}
