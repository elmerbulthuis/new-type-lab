use crate::models::Entity;
use crate::ValidationError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(try_from = "Vec<Entity>")]
pub struct EntityList(Vec<Entity>);

impl EntityList {
    fn new(value: Vec<Entity>) -> Result<Self, ValidationError> {
        let instance = Self(value);
        if instance.validate() {
            Ok(instance)
        } else {
            Err(ValidationError::new("EntityList"))
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
    type Error = ValidationError;

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
                organization: None,
                parent: None,
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
