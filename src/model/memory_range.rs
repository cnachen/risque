use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryRange {
    pub begin: String,
    pub end: String,
}