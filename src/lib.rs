mod models;

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::models::*;

    #[test]
    fn test_1() {
        let list = EntityList::try_from(vec![Entity::Person(
            Person {
                r#type: "person".parse().unwrap(),
                id: 1.try_into().unwrap(),
                name: "abc".parse().unwrap(),
            }
            .try_into()
            .unwrap(),
        )])
        .unwrap();

        let s = serde_json::to_string(&list).unwrap();
        assert_eq!(s, r#"[{"type":"person","id":1,"name":"abc"}]"#);
    }

    #[test]
    fn test_2() {
        let value = json!([
            {"type":"person","id":1,"name":"abc"}
        ]);
        let list: EntityList = serde_json::from_value(value).unwrap();

        #[cfg(feature = "deref")]
        for item in list.iter() {
            match item {
                Entity::Person(item) => {
                    let id = *item.id;
                    let name: &str = &item.name;

                    assert_eq!(id, 1);
                    assert_eq!(name, "abc");
                }
                Entity::Organization(_) => panic!(),
            }
        }

        #[cfg(feature = "as_ref")]
        for item in list.as_ref().iter() {
            match item {
                Entity::Person(item) => {
                    let item = item.as_ref();
                    let id = *item.id.as_ref();
                    let name = item.name.as_ref();

                    assert_eq!(id, 1);
                    assert_eq!(name, "abc");
                }
                Entity::Organization(_) => panic!(),
            }
        }
    }
}
