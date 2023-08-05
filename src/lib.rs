mod models;

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::models::*;

    #[test]
    fn test_1() {
        let list = PersonList::new(vec![Person {
            id: 1.into(),
            name: "abc".into(),
        }]);

        let s = serde_json::to_string(&list).unwrap();
        println!("{}", s);
    }

    #[test]
    fn test_2() {
        let value = json!([
            {"id":1,"name":"abc"}
        ]);
        let list: PersonList = serde_json::from_value(value).unwrap();

        for item in list.iter() {
            let id = *item.id;
            let name: &str = &item.name;

            assert_eq!(id, 1);
            assert_eq!(name, "abc");
        }
    }
}
