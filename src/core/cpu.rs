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

    pub fn execute(&mut self, inst: u32) -> Result<u64, Exception> {
        // decode as R-type
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        let funct3 = (inst >> 12) & 0x7;
        let funct7 = (inst >> 25) & 0x7f;

        // x0 is hardwired zero
        self.regs[0] = 0;

        // execute stage
        match opcode {
            0x37 => {
                // lui (Load Upper Immediate)
                let imm = (inst as u64 & 0xffff_0000_0000u64) >> 12;
                self.regs[rd] = imm as u64;
            }
            0x17 => {
                // auipc (Add Upper Immediate to PC)
                let imm = (inst as u64 & 0xffff_0000_0000u64) >> 12;
                self.regs[rd] = self.pc.wrapping_add(imm as u64);
            }
            0x6f => {
                // jal (Jump and Link)
                let imm = ((inst & 0x7fe00000) >> 20)
                    | ((inst & 0x000ff000) >> 7)
                    | ((inst & 0x00100000) >> 9);
                self.regs[rd] = self.pc + 4;
                self.pc = self.pc.wrapping_add(imm as u64);
            }
            0x67 => {
                // jalr (Jump and Link Register)
                let imm = (inst & 0xfff0_0000) >> 20;
                self.regs[rd] = self.pc + 4;
                self.pc = (self.regs[rs1] + imm as u64) & !1; // Ensure alignment
            }

            // Branch Instructions
            0x63 => {
                let imm = (((inst & 0x0000_7f00) >> 7) | ((inst & 0x7f000000) >> 20)) as u64;
                match funct3 {
                    0x0 => {
                        // beq (Branch if Equal)
                        if self.regs[rs1] == self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm);
                        }
                    }
                    0x1 => {
                        // bne (Branch if Not Equal)
                        if self.regs[rs1] != self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm);
                        }
                    }
                    0x4 => {
                        // blt (Branch if Less Than)
                        if (self.regs[rs1] as i64) < (self.regs[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm);
                        }
                    }
                    0x5 => {
                        // bge (Branch if Greater or Equal)
                        if (self.regs[rs1] as i64) >= (self.regs[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm);
                        }
                    }
                    0x6 => {
                        // bltu (Branch if Less Than Unsigned)
                        if self.regs[rs1] < self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm);
                        }
                    }
                    0x7 => {
                        // bgeu (Branch if Greater or Equal Unsigned)
                        if self.regs[rs1] >= self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm);
                        }
                    }
                    _ => {
                        dbg!(format!(
                            "Invalid funct3 for branch opcode 0x63: {:#x}",
                            funct3
                        ));
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }

            // Load Instructions
            0x03 => {
                let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                match funct3 {
                    0x0 => {
                        // lb (Load Byte)
                        let value = self.bus.load(self.regs[rs1].wrapping_add(imm), 1)?;
                        self.regs[rd] = value as u64;
                    }
                    0x1 => {
                        // lh (Load Halfword)
                        let value = self.bus.load(self.regs[rs1].wrapping_add(imm), 2)?;
                        self.regs[rd] = value as u64;
                    }
                    0x2 => {
                        // lw (Load Word)
                        let value = self.bus.load(self.regs[rs1].wrapping_add(imm), 4)?;
                        self.regs[rd] = value as u64;
                    }
                    0x4 => {
                        // lbu (Load Byte Unsigned)
                        let value = self.bus.load(self.regs[rs1].wrapping_add(imm), 1)?;
                        self.regs[rd] = value as u64;
                    }
                    0x5 => {
                        // lhu (Load Halfword Unsigned)
                        let value = self.bus.load(self.regs[rs1].wrapping_add(imm), 2)?;
                        self.regs[rd] = value as u64;
                    }
                    _ => {
                        dbg!(format!(
                            "Invalid funct3 for load opcode 0x03: {:#x}",
                            funct3
                        ));
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }

            // Store Instructions
            0x23 => {
                let imm = ((inst & 0xfe00_0000) >> 20) as u64;
                match funct3 {
                    0x0 => {
                        // sb (Store Byte)
                        self.bus
                            .store(self.regs[rs1].wrapping_add(imm), 1, self.regs[rs2])?;
                    }
                    0x1 => {
                        // sh (Store Halfword)
                        self.bus
                            .store(self.regs[rs1].wrapping_add(imm), 2, self.regs[rs2])?;
                    }
                    0x2 => {
                        // sw (Store Word)
                        self.bus
                            .store(self.regs[rs1].wrapping_add(imm), 4, self.regs[rs2])?;
                    }
                    _ => {
                        dbg!(format!(
                            "Invalid funct3 for store opcode 0x23: {:#x}",
                            funct3
                        ));
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }

            // Arithmetic Instructions
            0x13 => {
                match funct3 {
                    0x0 => {
                        // addi (Add Immediate)
                        let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                    }
                    0x2 => {
                        // slti (Set Less Than Immediate)
                        let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                        self.regs[rd] = if (self.regs[rs1] as i64) < (imm as i64) {
                            1
                        } else {
                            0
                        };
                    }
                    0x3 => {
                        // sltiu (Set Less Than Immediate Unsigned)
                        let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                        self.regs[rd] = if self.regs[rs1] < imm { 1 } else { 0 };
                    }
                    0x4 => {
                        // xori (XOR Immediate)
                        let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                        self.regs[rd] = self.regs[rs1] ^ imm;
                    }
                    0x6 => {
                        // ori (OR Immediate)
                        let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                        self.regs[rd] = self.regs[rs1] | imm;
                    }
                    0x7 => {
                        // andi (AND Immediate)
                        let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                        self.regs[rd] = self.regs[rs1] & imm;
                    }
                    _ => {
                        dbg!(format!("Invalid funct3 for opcode 0x13: {:#x}", funct3));
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }

            0x33 => {
                match funct3 {
                    0x0 => {
                        match funct7 {
                            0x00 => {
                                // add (Add)
                                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                            }
                            0x20 => {
                                // sub (Subtract)
                                self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                            }
                            _ => {
                                dbg!(format!("Invalid funct7 for add/sub: {:#x}", funct7));
                                return Err(Exception::IllegalInstruction(inst));
                            }
                        }
                    }
                    0x7 => {
                        // and (AND)
                        self.regs[rd] = self.regs[rs1] & self.regs[rs2];
                    }
                    0x4 => {
                        // xor (XOR)
                        self.regs[rd] = self.regs[rs1] ^ self.regs[rs2];
                    }
                    0x6 => {
                        // or (OR)
                        self.regs[rd] = self.regs[rs1] | self.regs[rs2];
                    }
                    _ => {
                        dbg!(format!("Invalid funct3 for opcode 0x33: {:#x}", funct3));
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }

            // More cases (e.g., addiw, addw, etc.) would be added here in a similar way.
            _ => {
                dbg!(format!("Invalid opcode: {:#x}", opcode));
                return Err(Exception::IllegalInstruction(inst));
            }
        }

        // x0 is hardwired zero
        self.regs[0] = 0;

        Ok(self.pc + 4)
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

    pub fn dump_registers(&mut self) {
        println!("{:-^80}", "registers");
        let mut output = String::new();
        self.regs[0] = 0;

        for i in (0..32).step_by(4) {
            let i0 = format!("x{}", i);
            let i1 = format!("x{}", i + 1);
            let i2 = format!("x{}", i + 2);
            let i3 = format!("x{}", i + 3);
            let line = format!(
                "{:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x}\n",
                i0, ABINAME[i], self.regs[i],
                i1, ABINAME[i + 1], self.regs[i + 1],
                i2, ABINAME[i + 2], self.regs[i + 2],
                i3, ABINAME[i + 3], self.regs[i + 3],
            );
            output = output + &line;
        }
        println!("{}", output);
    }
}
