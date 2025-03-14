use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryRange {
    pub begin: u64,
    pub end: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryValue {
    pub address: u64,
    pub word: u32,
}

impl MemoryValue {
    pub fn new(address: u64, word: u32) -> Self {
        Self { address, word }
    }
}
