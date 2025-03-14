use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterValue {
    pub key: String,
    pub value: u64,
}

impl RegisterValue {
    pub fn new(key: String, value: u64) -> Self {
        Self {
            key, value
        }
    }
}