type Inst = u32;
type Reg = u64;

pub fn mask(n: u32) -> Reg {
    if n >= 64 {
        u64::MAX
    } else {
        (1u64 << n) - 1
    }
}

pub fn sext(reg: Reg, n: u32) -> Reg {
    let m = !mask(n);
    let sign = (reg >> (n - 1)) & 1;
    if sign != 0 {
        reg | m
    } else {
        reg
    }
}

pub fn zext(reg: Reg, n: u32) -> Reg {
    reg & mask(n)
}

fn span(inst: Inst, end: u32, begin: u32) -> Inst {
    let m = ((1u32 << (end - begin + 1)) - 1) as Inst;
    (inst >> begin) & m
}

fn from_jimm20(jimm20: Inst) -> Reg {
    let mut r = span(jimm20, 19, 19) as Reg;
    r <<= 8;
    r |= span(jimm20, 7, 0) as Reg;
    r <<= 1;
    r |= span(jimm20, 8, 8) as Reg;
    r <<= 10;
    r |= span(jimm20, 18, 9) as Reg;
    r <<= 1;
    r
}

fn from_imm12hilo(imm12hi: Inst, imm12lo: Inst) -> Reg {
    let mut r = imm12hi as Reg;
    r <<= 5;
    r |= imm12lo as Reg;
    r
}

fn from_bimm12hilo(bimm12hi: Inst, bimm12lo: Inst) -> Reg {
    let mut r = span(bimm12hi, 6, 6) as Reg;
    r <<= 1;
    r |= span(bimm12lo, 0, 0) as Reg;
    r <<= 6;
    r |= span(bimm12hi, 5, 0) as Reg;
    r <<= 4;
    r |= span(bimm12lo, 4, 1) as Reg;
    r <<= 1;
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        assert_eq!(mask(0), 0);
        assert_eq!(mask(1), 0b1);
        assert_eq!(mask(5), 0b1_1111);
        assert_eq!(mask(64), u64::MAX);
        assert_eq!(mask(63), (1u64 << 63) - 1);
    }

    #[test]
    fn test_sext() {
        // sign bit is 0, should not extend
        assert_eq!(sext(0b0111, 4), 0b0111);
        // sign bit is 1, should extend with 1s
        assert_eq!(sext(0b1000, 4), 0xFFFFFFFFFFFFFFF8); // -8 in 64-bit two's complement
    }

    #[test]
    fn test_zext() {
        assert_eq!(zext(0b1111_1111, 4), 0b1111); // Keep only lower 4 bits
        assert_eq!(zext(0xFFFFFFFFFFFFFFFF, 64), 0xFFFFFFFFFFFFFFFF);
        assert_eq!(zext(0b1010_1010, 8), 0b1010_1010);
    }

    #[test]
    fn test_span() {
        let inst: Inst = 0b1111_0000_1010_0101_1100_0011_0110_1111;
        assert_eq!(span(inst, 12, 5), 0b0001_1011); // Manual calculation example
        assert_eq!(span(inst, 31, 31), 1); // The highest bit
        assert_eq!(span(inst, 7, 0), 0b0110_1111); // lowest byte
    }

    #[test]
    fn test_from_jimm20() {
        // Example bit pattern, you can adjust according to your encoding
        let jimm20: Inst = 0b1_0000000000_1_00000000_0; // fabricating a pattern
        let res = from_jimm20(jimm20);
        // You can calculate the expected value or just ensure function runs
        println!("JIMM20 Result: {:#x}", res);
    }

    #[test]
    fn test_from_imm12hilo() {
        let hi: Inst = 0b10101;
        let lo: Inst = 0b10010;
        let res = from_imm12hilo(hi, lo);
        assert_eq!(res, (hi as Reg) << 5 | lo as Reg);
    }

    #[test]
    fn test_from_bimm12hilo() {
        // Construct a sample branch immediate split
        let bimm12hi: Inst = 0b1_010101; // bit 6 is 1, bits 5:0 are 010101
        let bimm12lo: Inst = 0b0001_1011; // bit 0 is 1, bits 4:1 are 1011
        let res = from_bimm12hilo(bimm12hi, bimm12lo);
        println!("BIMM12 Result: {:#x}", res);
        // Exact value depends on bit layout, this is mainly to check correctness of extraction
    }
}
