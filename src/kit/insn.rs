#[derive(Clone)]
pub enum InsnType {
    R,
    I,
    S,
    B,
    U,
    J,
}

#[derive(Debug)]
pub struct RType {
    pub opcode: u32,
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub funct7: u32,
}

#[derive(Debug)]
pub struct IType {
    pub opcode: u32,
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug)]
pub struct SType {
    pub opcode: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub imm: u32,
}

#[derive(Debug)]
pub struct BType {
    pub opcode: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub imm: u32,
}

#[derive(Debug)]
pub struct UType {
    pub opcode: u32,
    pub rd: u32,
    pub imm: u32,
}

#[derive(Debug)]
pub struct JType {
    pub opcode: u32,
    pub rd: u32,
    pub imm: u32,
}

#[macro_export]
macro_rules! vdepart {
    ($inst:expr, InsnType::R) => {{
        RType {
            opcode:  $inst        & 0x7f,
            rd:     ($inst >> 7)  & 0x1f,
            funct3: ($inst >> 12) & 0x7,
            rs1:    ($inst >> 15) & 0x1f,
            rs2:    ($inst >> 20) & 0x1f,
            funct7: ($inst >> 25) & 0x7f,
        }
    }};
    ($inst:expr, InsnType::I) => {{
        IType {
            opcode:  $inst        & 0x7f,
            rd:     ($inst >> 7)  & 0x1f,
            funct3: ($inst >> 12) & 0x7,
            rs1:    ($inst >> 15) & 0x1f,
            imm:    ($inst >> 20) & 0xfff,
        }
    }};
    ($inst:expr, InsnType::S) => {{
        SType {
            opcode:  $inst        & 0x7f,
            funct3: ($inst >> 12) & 0x7,
            rs1:    ($inst >> 15) & 0x1f,
            rs2:    ($inst >> 20) & 0x1f,
            imm: (((($inst >> 25) & 0x7f) << 5)
                 | (($inst >> 7)  & 0x1f)),
        }
    }};
    ($inst:expr, InsnType::B) => {{
        BType {
            opcode:  $inst & 0x7f,
            funct3: ($inst >> 12) & 0x7,
            rs1:    ($inst >> 15) & 0x1f,
            rs2:    ($inst >> 20) & 0x1f,
            imm: (((($inst >> 31) & 0x1)  << 12)   // imm[12]
                | ((($inst >> 25) & 0x3f) << 5)    // imm[10:5]
                | ((($inst >> 8)  & 0xf)  << 1)    // imm[4:1]
                | ((($inst >> 7)  & 0x1)  << 11)), // imm[11],
        }
    }};
    ($inst:expr, InsnType::U) => {
        UType {
            opcode: $inst        & 0x7f,
            rd:    ($inst >> 7)  & 0x1f,
            imm:   ($inst >> 12) & 0xfffff,
        }
    };
    ($inst:expr, InsnType::J) => {{
        JType {
            opcode:  $inst        & 0x7f,
            rd:     ($inst >> 7)  & 0x1f,
            imm: (((($inst >> 31) & 0x1)   << 20)   // imm[20]
                | ((($inst >> 21) & 0x3ff) << 1)    // imm[10:1]
                | ((($inst >> 20) & 0x1)   << 11)   // imm[11]
                | ((($inst >> 12) & 0xff)  << 12)), // imm[19:12],
        }
    }};
}
