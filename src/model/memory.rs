use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryRangePayload {
    pub begin: u64,
    pub end: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryValueResponse {
    pub address: u64,
    pub word: u32,
}

impl MemoryValueResponse {
    pub fn new(address: u64, word: u32) -> Self {
        Self { address, word }
    }
}
