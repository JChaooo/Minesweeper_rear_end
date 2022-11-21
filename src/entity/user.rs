use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    name: String,
    password: String,
}

impl User {
    pub fn new(name: String, password: String) -> Self {
        User { name, password }
    }
}
