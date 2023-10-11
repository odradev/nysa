mod int;
mod ops;
mod utils;
pub use int::Unsigned;

pub type U8 = Unsigned<8, 1>;
pub type U16 = Unsigned<16, 1>;
pub type U24 = Unsigned<24, 1>;
pub type U32 = Unsigned<32, 1>;
pub type U40 = Unsigned<40, 1>;
pub type U48 = Unsigned<48, 1>;
pub type U56 = Unsigned<56, 1>;
pub type U64 = Unsigned<64, 1>;
pub type U72 = Unsigned<72, 2>;
pub type U80 = Unsigned<80, 2>;
pub type U88 = Unsigned<88, 2>;
pub type U96 = Unsigned<96, 2>;
pub type U104 = Unsigned<104, 2>;
pub type U112 = Unsigned<112, 2>;
pub type U120 = Unsigned<120, 2>;
pub type U128 = Unsigned<128, 2>;
pub type U136 = Unsigned<136, 3>;
pub type U144 = Unsigned<144, 3>;
pub type U152 = Unsigned<152, 3>;
pub type U160 = Unsigned<160, 3>;
pub type U168 = Unsigned<168, 3>;
pub type U176 = Unsigned<176, 3>;
pub type U184 = Unsigned<184, 3>;
pub type U192 = Unsigned<192, 3>;
pub type U200 = Unsigned<200, 4>;
pub type U208 = Unsigned<208, 4>;
pub type U216 = Unsigned<216, 4>;
pub type U224 = Unsigned<224, 4>;
pub type U232 = Unsigned<232, 4>;
pub type U240 = Unsigned<240, 4>;
pub type U248 = Unsigned<248, 4>;
pub type U256 = Unsigned<256, 4>;

#[cfg(test)]
mod t {
    use super::*;

    const ZERO: U32 = U32::MIN;
    const ONE: U32 = U32::from_limbs([1]);
    const TWO: U32 = U32::from_limbs([2]);
    const THREE: U32 = U32::from_limbs([3]);
    const EIGHT: U32 = U32::from_limbs([8]);
    const NINE: U32 = U32::from_limbs([9]);
    const TEN: U32 = U32::from_limbs([10]);
    const ELEVEN: U32 = U32::from_limbs([11]);
    const TWENTY: U32 = U32::from_limbs([20]);
    const THIRTY_TWO: U32 = U32::from_limbs([32]);

    #[test]
    fn bit_operators() {
        let val1 = U32::from_limbs([0b10000001]); // 129
        let val2 = U48::from_limbs([0b111]); // 7

        // xor
        let expected = U64::from_limbs([0b10000110]);
        assert_eq!(expected, val1.cast() ^ val2.cast());

        // or
        let expected = U64::from_limbs([0b10000111]);
        assert_eq!(expected, val1.cast() | val2.cast());

        // and
        let expected = U64::from_limbs([0b1]);
        assert_eq!(expected, val1.cast() & val2.cast());

        // not
        let expected =
            U64::from_limbs([0b0000000000000000111111111111111111111111111111111111111111111000]);
        assert_eq!(expected, (!val2).cast());
    }

    #[test]
    fn comparisons() {
        let val1 = U32::from_limbs([0b10000001]); // 129
        let val2 = U48::from_limbs([0b111]); // 7

        assert!(val1.cast() > val2);
        assert!(val1.cast() >= val2);
        assert!(val2 < val1.cast());
        assert!(val2 <= val1.cast());
        assert!(val2 != val1.cast());
        assert!(val1 == val1);
    }

    #[test]
    fn shifts() {
        let v = U32::from_limbs_slice(&[0b10000001]); // 129

        assert_eq!(
            U32::from_limbs([0b00000000000000000000000100000010]),
            v << 1
        );
        assert_eq!(
            U32::from_limbs([0b00000000000000100000010000000000]),
            v << 10
        );
        assert_eq!(
            U32::from_limbs([0b00001000000100000000000000000000]),
            v << 20
        );
        assert_eq!(
            U32::from_limbs([0b10000001000000000000000000000000]),
            v << 24
        );
        assert_eq!(
            U32::from_limbs([0b00000010000000000000000000000000]),
            v << 25
        );
        assert_eq!(
            U32::from_limbs([0b00000000000000000000000000000000]),
            v << 32
        );
        assert_eq!(
            U32::from_limbs([0b00000000000000000000000100000010]),
            v << 1
        );

        assert_eq!(
            U32::from_limbs([0b00000000000000000000000001000000]),
            v >> 1
        );
        assert_eq!(
            U32::from_limbs([0b00000000000000000000000000000001]),
            v >> 7
        );
        assert_eq!(
            U32::from_limbs([0b00000000000000000000000000000000]),
            v >> 8
        );
    }

    #[test]
    fn arithmetics() {
        let v = U32::from_limbs([0b10]); // 2

        assert_eq!(v.pow(THREE), EIGHT);
        assert_eq!(v.pow(TEN), U32::from_limbs([2u64.pow(10)]));
        assert_eq!(v.pow(THIRTY_TWO), ZERO);

        assert_eq!(TEN + TEN, TWENTY);
        assert_eq!(U32::from_limbs([u32::MAX as u64]) + ONE, ZERO);
        assert_eq!(U32::from_limbs([u32::MAX as u64]) + ELEVEN, TEN);

        assert_eq!(TWENTY - TEN, TEN);
        assert_eq!(ZERO - ONE, U32::MAX);
        assert_eq!(ZERO - TEN, U32::MAX - NINE);

        assert_eq!(TWENTY * TEN, U32::from_limbs([200]));
        assert_eq!(ONE * U32::MAX, U32::MAX);
        assert_eq!(U32::MAX * TWO, U32::MAX - ONE);

        assert_eq!(TWENTY / TEN, TWO);
        assert_eq!(TEN / TEN, ONE);

        assert_eq!(TWENTY % TEN, ZERO);
        assert_eq!(TEN % TWENTY, TEN);
        assert_eq!(TEN % THREE, ONE);
    }
}
