use std::collections::HashMap;

use super::{except::Exception, Cpu};
use crate::kit::insn::UType;
use crate::vdepart;

type IsaProcessor = Box<dyn Fn(&mut Cpu, u32) -> Result<u64, Exception> + Send + Sync>;

pub struct IsaDefine {
    pub ident: u32,
    pub processor: IsaProcessor,
}

impl IsaDefine {
    pub fn new(ident: u32, processor: IsaProcessor) -> Self {
        Self { ident, processor }
    }
}

pub fn register_default_isa_define_map() -> HashMap<u32, Vec<IsaDefine>> {
    let lui = IsaDefine::new(
        0x37,
        Box::new(|cpu, inst| {
            let c = vdepart!(inst, InsnType::U);
            Ok(4)
        }),
    );

    let mut map = HashMap::new();
    map.insert(0x37, vec![lui]);

    map
}
