use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterValueResponse {
    pub key: String,
    pub value: String,
}

impl RegisterValueResponse {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
