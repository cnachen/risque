use std::collections::HashMap;
use std::sync::Arc;

use crate::kit::bits::*;
use crate::kit::insn::*;
use crate::vdepart;

use super::isa::{install, IsaDefine};

fn mul() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = cpu.rgpr(r.rs1).wrapping_mul(cpu.rgpr(r.rs2));
            Ok(0)
        })),
        "mul",
        0x2000033,
        InsnType::R,
    )
}

/*
fn mulh() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            // *cpu.wgpr(r.rd) = ((cpu.rgpr(r.rs1) as i64 as i128).wrapping_mul(cpu.rgpr(r.rs2) as u128) >> 64) as u64;
            Ok(0)
        })),
        "mulh",
        0x2001033,
        InsnType::R,
    )
}

fn mulhsu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            Ok(0)
        })),
        "mulhsu",
        0x2002033,
        InsnType::R,
    )
}
 */

fn mulhu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) =
                ((cpu.rgpr(r.rs1) as u128).wrapping_mul(cpu.rgpr(r.rs2) as u128) >> 64) as u64;
            Ok(0)
        })),
        "mulhu",
        0x2003033,
        InsnType::R,
    )
}

fn div() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = (cpu.rgpr(r.rs1) as i64).wrapping_div(cpu.rgpr(r.rs2) as i64) as u64;
            Ok(0)
        })),
        "div",
        0x2004033,
        InsnType::R,
    )
}

fn divu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = cpu.rgpr(r.rs1).wrapping_div(cpu.rgpr(r.rs2));
            Ok(0)
        })),
        "divu",
        0x2005033,
        InsnType::R,
    )
}

fn rem() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = (cpu.rgpr(r.rs1) as i64).wrapping_rem(cpu.rgpr(r.rs2) as i64) as u64;
            Ok(0)
        })),
        "rem",
        0x2006033,
        InsnType::R,
    )
}

fn remu() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = cpu.rgpr(r.rs1).wrapping_rem(cpu.rgpr(r.rs2));
            Ok(0)
        })),
        "remu",
        0x2007033,
        InsnType::R,
    )
}

fn mulw() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = sext(
                cpu.rgpr(r.rs1).wrapping_mul(cpu.rgpr(r.rs2)) as u32 as u64,
                32,
            );
            Ok(0)
        })),
        "mulw",
        0x200003b,
        InsnType::R,
    )
}

fn divw() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = sext(
                (cpu.rgpr(r.rs1) as i32).wrapping_div(cpu.rgpr(r.rs2) as i32) as u32 as u64,
                32,
            );
            Ok(0)
        })),
        "divw",
        0x200403b,
        InsnType::R,
    )
}

fn divuw() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = sext(
                (cpu.rgpr(r.rs1) as u32).wrapping_div(cpu.rgpr(r.rs2) as u32) as u64,
                32,
            );
            Ok(0)
        })),
        "divuw",
        0x200503b,
        InsnType::R,
    )
}

fn remw() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = sext(
                (cpu.rgpr(r.rs1) as i32).wrapping_rem(cpu.rgpr(r.rs2) as i32) as u32 as u64,
                32,
            );
            Ok(0)
        })),
        "remw",
        0x200603b,
        InsnType::R,
    )
}

fn remuw() -> IsaDefine {
    IsaDefine::new(
        Arc::new(Box::new(|cpu, insn| {
            let r = vdepart!(insn, InsnType::R);
            *cpu.wgpr(r.rd) = sext(
                (cpu.rgpr(r.rs1) as u32).wrapping_rem(cpu.rgpr(r.rs2) as u32) as u64,
                32,
            );
            Ok(0)
        })),
        "remuw",
        0x200703b,
        InsnType::R,
    )
}

pub fn register_ext(map: &mut HashMap<u32, Vec<IsaDefine>>) {
    // TBD: add more instructions
    install(map, mul());
    // install(map, mulh());
    // install(map, mulhsu());
    install(map, mulhu());
    install(map, div());
    install(map, divu());
    install(map, rem());
    install(map, remu());
    install(map, mulw());
    install(map, divw());
    install(map, divuw());
    install(map, remw());
    install(map, remuw());
}
