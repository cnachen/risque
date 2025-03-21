use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterValueResponse {
    pub key: String,
    pub value: u64,
}

impl RegisterValueResponse {
    pub fn new(key: String, value: u64) -> Self {
        Self { key, value }
    }
}
