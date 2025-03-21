use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StepResponse {
    pc: u64,
    insn: u32,
    message: String,
}

impl StepResponse {
    pub fn new(pc: u64, insn: u32, message: String) -> Self {
        Self { pc, insn, message }
    }
    }