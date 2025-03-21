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
            *cpu.wgpr(u.rd) = sext(u.imm as u64, 20) << 12;
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
            *cpu.wgpr(u.rd) = cpu.pc.wrapping_add(sext(u.imm as u64, 20) << 12);
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
            *cpu.wgpr(j.rd) = cpu.pc.wrapping_add(4);
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
            let t = cpu.pc.wrapping_add(4);
            cpu.pc = (cpu.rgpr(i.rs1).wrapping_add(sext(i.imm as u64, 12))) & mask(1);
            cpu.pcimm = 0;
            *cpu.wgpr(i.rd) = t;
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
            if cpu.rgpr(b.rs1) == cpu.rgpr(b.rs2) {
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
            if cpu.rgpr(b.rs1) != cpu.rgpr(b.rs2) {
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
            if (cpu.rgpr(b.rs1) as i64) < (cpu.rgpr(b.rs2) as i64) {
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
            if cpu.rgpr(b.rs1) as i64 >= cpu.rgpr(b.rs2) as i64 {
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
            if cpu.rgpr(b.rs1) < cpu.rgpr(b.rs2) {
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
            if cpu.rgpr(b.rs1) >= cpu.rgpr(b.rs2) {
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

fn lb() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            *cpu.wgpr(i.rd) = sext(cpu.load(cpu.rgpr(i.rs1).wrapping_add(sext(i.imm as u64, 12)), 8).unwrap(), 8);
            Ok(0)
        })),
        "lb",
        0x3,
        InsnType::I,
    )
}

fn lh() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            *cpu.wgpr(i.rd) = sext(cpu.load(cpu.rgpr(i.rs1).wrapping_add(sext(i.imm as u64, 12)), 16).unwrap(), 16);
            Ok(0)
        })),
        "lh",
        0x1003,
        InsnType::I,
    )
}

fn lw() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            *cpu.wgpr(i.rd) = sext(cpu.load(cpu.rgpr(i.rs1).wrapping_add(sext(i.imm as u64, 12)), 32).unwrap(), 32);
            Ok(0)
        })),
        "lw",
        0x2003,
        InsnType::I,
    )
}

fn ld() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            *cpu.wgpr(i.rd) = cpu.load(cpu.rgpr(i.rs1).wrapping_add(sext(i.imm as u64, 12)), 64).unwrap();
            Ok(0)
        })),
        "ld",
        0x3003,
        InsnType::I,
    )
}

fn lbu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            *cpu.wgpr(i.rd) = zext(cpu.load(cpu.rgpr(i.rs1).wrapping_add(sext(i.imm as u64, 12)), 8).unwrap(), 8);
            Ok(0)
        })),
        "lbu",
        0x4003,
        InsnType::I,
    )
}

fn lhu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            *cpu.wgpr(i.rd) = zext(cpu.load(cpu.rgpr(i.rs1).wrapping_add(sext(i.imm as u64, 12)), 16).unwrap(), 16);
            Ok(0)
        })),
        "lhu",
        0x5003,
        InsnType::I,
    )
}

fn lwu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            *cpu.wgpr(i.rd) = zext(cpu.load(cpu.rgpr(i.rs1).wrapping_add(sext(i.imm as u64, 12)), 32).unwrap(), 32);
            Ok(0)
        })),
        "lwu",
        0x6003,
        InsnType::I,
    )
}

fn addi() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let i = vdepart!(insn, InsnType::I);
            *cpu.wgpr(i.rd) = cpu.rgpr(i.rs1).wrapping_add(sext(i.imm as u64, 12));
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
            *cpu.wgpr(i.rd) = ((cpu.rgpr(i.rs1) as i64) < sext(i.imm as u64, 12) as i64) as u64;
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
            *cpu.wgpr(i.rd) = (cpu.rgpr(i.rs1) < sext(i.imm as u64, 12)) as u64;
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
            *cpu.wgpr(i.rd) = cpu.rgpr(i.rs1) ^ sext(i.imm as u64, 12);
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
            *cpu.wgpr(i.rd) = cpu.rgpr(i.rs1) | sext(i.imm as u64, 12);
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
            *cpu.wgpr(i.rd) = (cpu.rgpr(i.rs1) & sext(i.imm as u64, 12)) as u64;
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
            *cpu.wgpr(r.rd) = cpu.rgpr(r.rs1).wrapping_add(cpu.rgpr(r.rs2));
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
            *cpu.wgpr(r.rd) = cpu.rgpr(r.rs1).wrapping_sub(cpu.rgpr(r.rs2));
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
            *cpu.wgpr(r.rd) = cpu.rgpr(r.rs1) | cpu.rgpr(r.rs2);
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
            *cpu.wgpr(r.rd) = cpu.rgpr(r.rs1) & cpu.rgpr(r.rs2);
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
    install(map, lb());
    install(map, lh());
    install(map, lw());
    install(map, ld());
    install(map, lbu());
    install(map, lhu());
    install(map, lwu());
    /*
    install(map, sb());
    install(map, sh());
    install(map, sw());
    install(map, sd());
    */
    install(map, addi());
    install(map, slti());
    install(map, sltiu());
    install(map, xori());
    install(map, ori());
    install(map, andi());
    /*
    install(map, slli());
    install(map, srli());
    install(map, srai());
    */
    install(map, add());
    install(map, sub());
    /*
    install(map, sll());
    install(map, slt());
    install(map, sltu());
    install(map, xor());
    install(map, srl());
    install(map, sra());
    */
    install(map, or());
    install(map, and());
}
