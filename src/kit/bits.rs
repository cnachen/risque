pub fn mask(n: u32) -> u64 {
    if n >= 64 {
        u64::MAX
    } else {
        (1u64 << n) - 1
    }
}

pub fn sext(reg: u64, n: u32) -> u64 {
    let m = !mask(n);
    let sign = (reg >> (n - 1)) & 1;
    if sign != 0 {
        reg | m
    } else {
        reg
    }
}

pub fn zext(reg: u64, n: u32) -> u64 {
    reg & mask(n)
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
}
