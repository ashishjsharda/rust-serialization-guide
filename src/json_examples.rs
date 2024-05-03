use crate::models::User;
use serde_json;

pub fn serialize_user() -> String {
    let user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        active: true,
    };

    serde_json::to_string(&user).unwrap()
}

pub fn deserialize_user(serialized: &str) -> User {
    serde_json::from_str(serialized).unwrap()
}
