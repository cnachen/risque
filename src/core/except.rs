#[derive(Debug)]
pub enum Exception {
    StoreAMOAccessFault(u64),
    LoadAccessFault(u64),
    IllegalInstruction(u32),
}
