use super::{
    bus::Bus,
    exception::Exception,
    param::{DRAM_BASE, DRAM_END},
};

const ABINAME: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub bus: Bus,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = DRAM_END;
        let bus = Bus::new(code);
        Self {
            regs,
            pc: DRAM_BASE,
            bus: bus,
        }
    }

    /// Load a value from a dram.
    pub fn _load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        self.bus.load(addr, size)
    }

    /// Store a value to a dram.
    pub fn _store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
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
        let _funct3 = (inst >> 12) & 0x7;
        let _funct7 = (inst >> 25) & 0x7f;

        // x0 is hardwired zero
        self.regs[0] = 0;

        // execute stage
        match opcode {
            0x13 => {
                // addi
                let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }

            _ => {
                dbg!(format!("Invalid opcode: {:#x}", opcode));
                return Err(Exception::IllegalInstruction(inst));
            }
        }

        Ok(self.pc + 4)
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
