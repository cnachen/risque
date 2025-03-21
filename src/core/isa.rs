use std::collections::HashMap;
use std::sync::Arc;

use super::{except::Exception, Cpu};
use crate::kit::bits::*;
use crate::kit::insn::*;
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
    pub fn new(
        ident: u32,
        mtype: InsnType,
        mnemonic: &'static str,
        processor: IsaProcessor,
    ) -> Self {
        Self {
            ident,
            mtype,
            mnemonic,
            processor,
        }
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
            Ok(0)
        })),
    );

    let auipc = IsaDefine::new(
        0x17,
        InsnType::U,
        "auipc",
        Arc::new(Box::new(|cpu, inst| {
            let u = vdepart!(inst, InsnType::U);

            cpu.regs[u.rd as usize] = cpu.pc.wrapping_add(sext(u.imm as u64, 20) << 12);
            Ok(0)
        })),
    );

    let jal = IsaDefine::new(
        0x6f,
        InsnType::J,
        "jal",
        Arc::new(Box::new(|cpu, inst| {
            let j = vdepart!(inst, InsnType::J);

            cpu.regs[j.rd as usize] = cpu.pc + 4;
            cpu.pc = cpu.pc.wrapping_add(sext(j.imm as u64, 21));
            cpu.pcimm = 0;
            Ok(0)
        })),
    );

    let jalr = IsaDefine::new(
        0x67,
        InsnType::I,
        "jalr",
        Arc::new(Box::new(|cpu, inst| {
            let i = vdepart!(inst, InsnType::I);

            let t = cpu.pc + 4;
            cpu.pc = (cpu.regs[i.rs1 as usize].wrapping_add(sext(i.imm as u64, 12))) & mask(1);
            cpu.pcimm = 0;
            cpu.regs[i.rd as usize] = t;
            Ok(0)
        })),
    );

    // TBD: add more instructions
    

    let mut map = HashMap::new();
    map.insert(0x37, vec![lui]);
    map.insert(0x17, vec![auipc]);
    map.insert(0x6f, vec![jal]);
    map.insert(0x67, vec![jalr]);

    map
}
