use std::collections::HashMap;
use std::sync::Arc;

use crate::kit::bits::*;
use crate::kit::insn::*;
use crate::vdepart;

use super::isa::{install, IsaDefine};

fn lui() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let u = vdepart!(insn, InsnType::U);
            cpu.regs[u.rd as usize] = sext(u.imm as u64, 20) << 12;
            Ok(0)
        })),
        "lui",
        0x37,
        InsnType::U,
    )
}

fn auipc() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let u = vdepart!(insn, InsnType::U);
            cpu.regs[u.rd as usize] = cpu.pc.wrapping_add(sext(u.imm as u64, 20) << 12);
            Ok(0)
        })),
        "auipc",
        0x17,
        InsnType::U,
    )
}

fn jal() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let j = vdepart!(insn, InsnType::J);
            cpu.regs[j.rd as usize] = cpu.pc + 4;
            cpu.pc = cpu.pc.wrapping_add(sext(j.imm as u64, 21));
            cpu.pcimm = 0;
            Ok(0)
        })),
        "jal",
        0x6f,
        InsnType::J,
    )
}

fn jalr() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            let t = cpu.pc + 4;
            cpu.pc = (cpu.regs[i.rs1 as usize].wrapping_add(sext(i.imm as u64, 12))) & mask(1);
            cpu.pcimm = 0;
            cpu.regs[i.rd as usize] = t;
            Ok(0)
        })),
        "jalr",
        0x67,
        InsnType::I,
    )
}

pub fn register_ext(map: &mut HashMap<u32, Vec<IsaDefine>>) {
    // TBD: add more instructions
    install(map, lui());
    install(map, auipc());
    install(map, jal());
    install(map, jalr());
}
