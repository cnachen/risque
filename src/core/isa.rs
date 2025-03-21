use std::collections::HashMap;
use std::sync::Arc;

use super::{except::Exception, Cpu};
use crate::kit::insn::InsnType;

type IsaProcessor = Arc<Box<dyn Fn(&mut Cpu, u32) -> Result<u64, Exception> + Send + Sync>>;

#[derive(Clone)]
pub struct IsaDefine {
    pub ident: u32,
    pub mtype: InsnType,
    pub mnemonic: &'static str,
    pub processor: IsaProcessor,
}

impl IsaDefine {
    pub fn new(
        processor: IsaProcessor,
        mnemonic: &'static str,
        ident: u32,
        mtype: InsnType,
    ) -> Self {
        Self {
            ident,
            mtype,
            mnemonic,
            processor,
        }
    }
}

pub fn install(map: &mut HashMap<u32, Vec<IsaDefine>>, insn: IsaDefine) {
    let op = insn.ident & 0x7f;

    if let Some(v) = map.get_mut(&op) {
        v.push(insn);
        return;
    } else {
        map.insert(op, vec![insn]);
    }
}
