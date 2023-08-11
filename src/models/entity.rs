use super::{OrganizationContainer, PersonContainer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Entity {
    Organization(OrganizationContainer),
    Person(PersonContainer),
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::models::*;

    #[test]
    fn test_1() {
        let serialized = json!(
            {"type":"person","id":1,"name":"abc"}
        );
        let deserialized: Result<Entity, _> = serde_json::from_value(serialized);
        let deserialized = deserialized.unwrap();

        if let Entity::Person(_) = deserialized {
            //
        } else {
            panic!("not a person")
        }

        let serialized = json!(
            {"type":"organization","id":1,"name":"abc"}
        );
        let deserialized: Result<Entity, _> = serde_json::from_value(serialized);
        let deserialized = deserialized.unwrap();

        if let Entity::Organization(_) = deserialized {
            //
        } else {
            panic!("not an organization")
        }

        let serialized = json!(
            {"type":"something else","id":1,"name":"abc"}
        );
        let deserialized: Result<Entity, _> = serde_json::from_value(serialized);

        assert!(deserialized.is_err());
    }
}
