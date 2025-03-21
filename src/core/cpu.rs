use std::collections::HashMap;

use rand::prelude::*;

use crate::{
    kit::insn::InsnType,
    model::{MemoryValueResponse, RegisterValueResponse},
};

use super::{
    bus::Bus,
    except::Exception,
    isa::IsaDefine,
    param::{ABINAME, DRAM_BASE, DRAM_END},
};

use crate::kit::insn::*;
use crate::vdepart;

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub pcimm: u64,
    pub bus: Bus,
    pub running: bool,
    pub isa_define_map: HashMap<u32, Vec<IsaDefine>>,
    pub breakpoints: Vec<u64>,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        let mut rng = rand::rng();

        let mut regs = [0; 32];
        for i in 1..regs.len() {
            regs[i] = rng.next_u64(); // Set random values for the rest of the elements
        }
        regs[0] = 0;
        regs[2] = DRAM_END;

        let bus = Bus::new(code);

        // for i in (DRAM_BASE..=DRAM_BASE + 0x1c).step_by(8) {
        //     bus.store(i, 64, rng.next_u64()).unwrap();
        // }

        let mut map = HashMap::new();
        super::i::register_ext(&mut map);

        Self {
            regs,
            pc: DRAM_BASE,
            pcimm: 4,
            bus: bus,
            running: false,
            isa_define_map: map,
            breakpoints: Vec::new(),
        }
    }

    /// Load a value from a dram.
    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        self.bus.load(addr, size)
    }

    /// Store a value to a dram.
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        self.bus.store(addr, size, value)
    }

    /// Get an instruction from the dram.
    pub fn fetch(&mut self) -> Result<u64, Exception> {
        self.bus.load(self.pc, 32)
    }

    pub fn explain(&self, insn: u32) -> String {
        let isa_defines = self.isa_define_map.get(&(insn & 0x7f)).cloned();

        match isa_defines {
            Some(isa_defines) => {
                for isa in isa_defines {
                    if isa.mtype == InsnType::R && (insn & 0xfe00707f) == isa.ident
                        || (isa.mtype == InsnType::I
                            || isa.mtype == InsnType::S
                            || isa.mtype == InsnType::B)
                            && (insn & 0x707f) == isa.ident
                        || (isa.mtype == InsnType::U || isa.mtype == InsnType::J)
                            && (insn & 0x7f) == isa.ident
                    {
                        match isa.mtype {
                            InsnType::U => {
                                let u = vdepart!(insn, InsnType::U);
                                return format!(
                                    "{:016x}: {}\t{}, 0x{:05x}",
                                    self.pc, isa.mnemonic, ABINAME[u.rd as usize], u.imm
                                );
                            }
                            InsnType::I => {
                                let i = vdepart!(insn, InsnType::I);
                                return format!(
                                    "{:016x}: {}\t{}, {}, 0x{:03x}",
                                    self.pc,
                                    isa.mnemonic,
                                    ABINAME[i.rd as usize],
                                    ABINAME[i.rs1 as usize],
                                    i.imm
                                );
                            }
                            InsnType::S => {
                                let s = vdepart!(insn, InsnType::S);
                                return format!(
                                    "{:016x}: {}\t{}, {}, 0x{:x}",
                                    self.pc,
                                    isa.mnemonic,
                                    ABINAME[s.rs1 as usize],
                                    ABINAME[s.rs2 as usize],
                                    s.imm
                                );
                            }
                            InsnType::B => {
                                let b = vdepart!(insn, InsnType::B);
                                return format!(
                                    "{:016x}: {}\t{}, {}, 0x{:x}",
                                    self.pc,
                                    isa.mnemonic,
                                    ABINAME[b.rs1 as usize],
                                    ABINAME[b.rs2 as usize],
                                    b.imm
                                );
                            }
                            InsnType::R => {
                                let r = vdepart!(insn, InsnType::R);
                                return format!(
                                    "{:016x}: {}\t{}, {}, 0x{}",
                                    self.pc,
                                    isa.mnemonic,
                                    ABINAME[r.rd as usize],
                                    ABINAME[r.rs1 as usize],
                                    ABINAME[r.rs2 as usize]
                                );
                            }
                            InsnType::J => {
                                let j = vdepart!(insn, InsnType::J);
                                return format!(
                                    "{:016x}: {}\t{}, 0x{:x}",
                                    self.pc, isa.mnemonic, ABINAME[j.rd as usize], j.imm
                                );
                            }
                        }
                    }
                }
            }
            None => {}
        }
        format!("{:016x}: ? 0x{:08x}", self.pc, insn)
    }

    pub fn execute(&mut self, insn: u32) -> Result<u64, Exception> {
        // x0 is hardwired zero
        self.regs[0] = 0;
        self.pcimm = 4;

        println!("{}", self.explain(insn));

        let isa_defines = self.isa_define_map.get(&(insn & 0x7f)).cloned();

        match isa_defines {
            Some(isa_defines) => {
                for isa in isa_defines {
                    if isa.mtype == InsnType::R && (insn & 0xfe00707f) == isa.ident
                        || (isa.mtype == InsnType::I
                            || isa.mtype == InsnType::S
                            || isa.mtype == InsnType::B)
                            && (insn & 0x707f) == isa.ident
                        || (isa.mtype == InsnType::U || isa.mtype == InsnType::J)
                            && (insn & 0x7f) == isa.ident
                    {
                        // Now call the processor with a mutable borrow of `self`
                        if let Err(e) = (isa.processor)(self, insn) {
                            return Err(e);
                        }
                    }
                }
            }
            None => {
                dbg!(format!("Invalid opcode: {:#x}", insn));
                return Err(Exception::IllegalInstruction(insn));
            }
        }

        // x0 is hardwired zero
        self.regs[0] = 0;

        Ok(self.pc.wrapping_add(self.pcimm))
    }

    pub fn wgpr(&mut self, id: u32) -> &mut u64 {
        &mut self.regs[id as usize]
    }

    pub fn rgpr(&self, id: u32) -> u64 {
        self.regs[id as usize]
    }

    pub fn read_registers(&self) -> Vec<RegisterValueResponse> {
        let mut vec = Vec::new();
        vec.push(RegisterValueResponse::new("pc".into(), format!("0x{:016x}", self.pc)));
        for (i, &name) in ABINAME.iter().enumerate() {
            vec.push(RegisterValueResponse::new(name.into(), format!("0x{:016x}", self.regs[i])));
        }
        vec
    }

    pub fn read_memory_range(&self, begin: u64, end: u64) -> Vec<MemoryValueResponse> {
        let mut vec = Vec::new();

        for address in (begin..=end).step_by(4) {
            vec.push(MemoryValueResponse::new(
                address,
                self.bus.load(address, 32).unwrap() as u32,
            ));
        }

        vec.reverse();
        vec
    }
}
