use models::{Person, PersonList};

mod models;

pub fn select_person_list() -> PersonList<'static> {
    PersonList::new(vec![Person {
        id: 1.into(),
        name: "abc".into(),
    }])
}

pub fn insert_person(item: Person<'static>) {
    //
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::*;

    #[test]
    fn test_1() {
        let id = 2.into();
        let name = "Y".into();

        let item = Person { id, name };
        insert_person(item);

        let list = select_person_list();
        for item in list.iter() {
            let id = *item.id;
            let name = &item.name;
        }
    }
}
