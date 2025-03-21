use std::collections::HashMap;
use std::sync::Arc;

use super::{except::Exception, Cpu};
use crate::kit::bits::*;
use crate::kit::insn::UType;
use crate::kit::insn::InsnType;
use crate::vdepart;

type IsaProcessor = Arc<Box<dyn Fn(&mut Cpu, u32) -> Result<u64, Exception> + Send + Sync>>;

#[derive(Clone)]
pub struct IsaDefine {
    pub ident: u32,
    pub mtype: InsnType,
    pub mnemonic: &'static str,
    pub processor: IsaProcessor,
}

impl IsaDefine {
    pub fn new(ident: u32, mtype: InsnType, mnemonic: &'static str, processor: IsaProcessor) -> Self {
        Self { ident, mtype, mnemonic, processor }
    }
}

pub fn register_default_isa_define_map() -> HashMap<u32, Vec<IsaDefine>> {
    let lui = IsaDefine::new(
        0x37,
        InsnType::U,
        "lui",
        Arc::new(Box::new(|cpu, inst| {
            let u = vdepart!(inst, InsnType::U);
            cpu.regs[u.rd as usize] = sext(u.imm as u64, 20) << 12;
            Ok(4)
        })),
    );

    let mut map = HashMap::new();
    map.insert(0x37, vec![lui]);

    map
}
