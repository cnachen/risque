use std::collections::HashMap;

use rand::prelude::*;

use crate::model::{MemoryValueResponse, RegisterValueResponse};

use super::{
    bus::Bus,
    except::Exception,
    isa::{register_default_isa_define_map, IsaDefine},
    param::{ABINAME, DRAM_BASE, DRAM_END},
};

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub pcimm: u64,
    pub bus: Bus,
    pub running: bool,
    pub isa_define_map: HashMap<u32, Vec<IsaDefine>>,
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

        let mut bus = Bus::new(code);

        for i in (DRAM_BASE..=DRAM_BASE + 0x1c).step_by(8) {
            bus.store(i, 64, rng.next_u64()).unwrap();
        }

        Self {
            regs,
            pc: DRAM_BASE,
            pcimm: 4,
            bus: bus,
            running: false,
            isa_define_map: register_default_isa_define_map(),
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
                    if insn & isa.ident == isa.ident {
                        return format!("{} {}", isa.mnemonic, insn);
                    }
                }
            }
            None => {
                return format!("Invalid opcode: {:#x}", insn);
            }
        }

        format!("Invalid opcode: {:#x}", insn)
    }

    pub fn execute(&mut self, insn: u32) -> Result<u64, Exception> {
        // x0 is hardwired zero
        self.regs[0] = 0;

        let isa_defines = self.isa_define_map.get(&(insn & 0x7f)).cloned();

        match isa_defines {
            Some(isa_defines) => {
                for isa in isa_defines {
                    if insn & isa.ident == isa.ident {
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

        Ok(self.pc + self.pcimm)
    }

    pub fn read_registers(&self) -> Vec<RegisterValueResponse> {
        let mut vec = Vec::new();
        for (i, &name) in ABINAME.iter().enumerate() {
            vec.push(RegisterValueResponse::new(name.into(), self.regs[i]));
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
