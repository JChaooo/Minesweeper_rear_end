use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    name: String,
    password: String,
}

impl User {
    pub fn new(id: i32, name: String, password: String) -> Self {
        User { id, name, password }
    }
}
