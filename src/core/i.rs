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

fn beq() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let b = vdepart!(insn, InsnType::B);
            if cpu.regs[b.rs1 as usize] == cpu.regs[b.rs2 as usize] {
                cpu.pc = cpu.pc.wrapping_add(sext(b.imm as u64, 13));
                cpu.pcimm = 0;
            }
            Ok(0)
        })),
        "beq",
        0x63,
        InsnType::B,
    )
}

fn bne() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let b = vdepart!(insn, InsnType::B);
            if cpu.regs[b.rs1 as usize] != cpu.regs[b.rs2 as usize] {
                cpu.pc = cpu.pc.wrapping_add(sext(b.imm as u64, 13));
                cpu.pcimm = 0;
            }
            Ok(0)
        })),
        "bne",
        0x1063,
        InsnType::B,
    )
}

fn blt() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let b = vdepart!(insn, InsnType::B);
            if (cpu.regs[b.rs1 as usize] as i64) < (cpu.regs[b.rs2 as usize] as i64) {
                cpu.pc = cpu.pc.wrapping_add(sext(b.imm as u64, 13));
                cpu.pcimm = 0;
            }
            Ok(0)
        })),
        "blt",
        0x4063,
        InsnType::B,
    )
}

fn bge() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let b = vdepart!(insn, InsnType::B);
            if cpu.regs[b.rs1 as usize] as i64 >= cpu.regs[b.rs2 as usize] as i64 {
                cpu.pc = cpu.pc.wrapping_add(sext(b.imm as u64, 13));
                cpu.pcimm = 0;
            }
            Ok(0)
        })),
        "bge",
        0x5063,
        InsnType::B,
    )
}

fn bltu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let b = vdepart!(insn, InsnType::B);
            if cpu.regs[b.rs1 as usize] < cpu.regs[b.rs2 as usize] {
                cpu.pc = cpu.pc.wrapping_add(sext(b.imm as u64, 13));
                cpu.pcimm = 0;
            }
            Ok(0)
        })),
        "bltu",
        0x6063,
        InsnType::B,
    )
}

fn bgeu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let b = vdepart!(insn, InsnType::B);
            if cpu.regs[b.rs1 as usize] >= cpu.regs[b.rs2 as usize] {
                cpu.pc = cpu.pc.wrapping_add(sext(b.imm as u64, 13));
                cpu.pcimm = 0;
            }
            Ok(0)
        })),
        "bgeu",
        0x7063,
        InsnType::B,
    )
}

fn addi() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            cpu.regs[i.rd as usize] = cpu.regs[i.rs1 as usize].wrapping_add(sext(i.imm as u64, 12));
            Ok(0)
        })),
        "addi",
        0x13,
        InsnType::I,
    )
}

fn slti() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            cpu.regs[i.rd as usize] = ((cpu.regs[i.rs1 as usize] as i64) < sext(i.imm as u64, 12) as i64) as u64;
            Ok(0)
        })),
        "slti",
        0x2013,
        InsnType::I,
    )
}

fn sltiu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            cpu.regs[i.rd as usize] = (cpu.regs[i.rs1 as usize] < sext(i.imm as u64, 12)) as u64;
            Ok(0)
        })),
        "sltiu",
        0x3013,
        InsnType::I,
    )
}

fn xori() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            cpu.regs[i.rd as usize] = cpu.regs[i.rs1 as usize] ^ sext(i.imm as u64, 12);
            Ok(0)
        })),
        "xori",
        0x4013,
        InsnType::I,
    )
}

fn ori() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            cpu.regs[i.rd as usize] = cpu.regs[i.rs1 as usize] | sext(i.imm as u64, 12);
            Ok(0)
        })),
        "ori",
        0x6013,
        InsnType::I,
    )
}

fn andi() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            cpu.regs[i.rd as usize] = (cpu.regs[i.rs1 as usize] & sext(i.imm as u64, 12)) as u64;
            Ok(0)
        })),
        "andi",
        0x7013,
        InsnType::I,
    )
}

fn add() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            cpu.regs[r.rd as usize] = cpu.regs[r.rs1 as usize].wrapping_add(cpu.regs[r.rs2 as usize]);
            Ok(0)
        })),
        "add",
        0x33,
        InsnType::R,
    )
}

fn sub() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            cpu.regs[r.rd as usize] = cpu.regs[r.rs1 as usize].wrapping_sub(cpu.regs[r.rs2 as usize]);
            Ok(0)
        })),
        "sub",
        0x40000033,
        InsnType::R,
    )
}

fn or() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            cpu.regs[r.rd as usize] = cpu.regs[r.rs1 as usize] | cpu.regs[r.rs2 as usize];
            Ok(0)
        })),
        "or",
        0x6033,
        InsnType::R,
    )
}

fn and() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            cpu.regs[r.rd as usize] = cpu.regs[r.rs1 as usize] & cpu.regs[r.rs2 as usize];
            Ok(0)
        })),
        "and",
        0x7033,
        InsnType::R,
    )
}

pub fn register_ext(map: &mut HashMap<u32, Vec<IsaDefine>>) {
    // TBD: add more instructions
    install(map, lui());
    install(map, auipc());
    install(map, jal());
    install(map, jalr());
    install(map, beq());
    install(map, bne());
    install(map, blt());
    install(map, bge());
    install(map, bltu());
    install(map, bgeu());
    install(map, addi());
    install(map, slti());
    install(map, sltiu());
    install(map, xori());
    install(map, ori());
    install(map, andi());
    install(map, add());
    install(map, sub());
    install(map, or());
    install(map, and());
}
