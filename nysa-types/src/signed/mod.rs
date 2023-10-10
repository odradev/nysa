mod int;
mod ops;
mod sign;
pub(crate) mod utils;

pub use int::Signed;
pub use sign::Sign;

pub type I8 = Signed<8, 1>;
pub type I16 = Signed<16, 1>;
pub type I24 = Signed<24, 1>;
pub type I32 = Signed<32, 1>;
pub type I40 = Signed<40, 1>;
pub type I48 = Signed<48, 1>;
pub type I56 = Signed<56, 1>;
pub type I64 = Signed<64, 1>;
pub type I72 = Signed<72, 2>;
pub type I80 = Signed<80, 2>;
pub type I88 = Signed<88, 2>;
pub type I96 = Signed<96, 2>;
pub type I104 = Signed<104, 2>;
pub type I112 = Signed<112, 2>;
pub type I120 = Signed<120, 2>;
pub type I128 = Signed<128, 2>;
pub type I136 = Signed<136, 3>;
pub type I144 = Signed<144, 3>;
pub type I152 = Signed<152, 3>;
pub type I160 = Signed<160, 3>;
pub type I168 = Signed<168, 3>;
pub type I176 = Signed<176, 3>;
pub type I184 = Signed<184, 3>;
pub type I192 = Signed<192, 3>;
pub type I200 = Signed<200, 4>;
pub type I208 = Signed<208, 4>;
pub type I216 = Signed<216, 4>;
pub type I224 = Signed<224, 4>;
pub type I232 = Signed<232, 4>;
pub type I240 = Signed<240, 4>;
pub type I248 = Signed<248, 4>;
pub type I256 = Signed<256, 4>;

#[cfg(test)]
mod t {
    use crate::U64;

    use super::*;

    const MINUS_TWENTY: I64 = I64::from_limbs([u64::MAX - 19]);
    const MINUS_TEN: I64 = I64::from_limbs([u64::MAX - 9]);
    const MINUS_NINE: I64 = I64::from_limbs([u64::MAX - 8]);
    const MINUS_EIGHT: I64 = I64::from_limbs([u64::MAX - 7]);
    const MINUS_SEVEN: I64 = I64::from_limbs([u64::MAX - 6]);
    const MINUS_SIX: I64 = I64::from_limbs([u64::MAX - 5]);
    const MINUS_FIVE: I64 = I64::from_limbs([u64::MAX - 4]);
    const MINUS_FOUR: I64 = I64::from_limbs([u64::MAX - 3]);
    const MINUS_THREE: I64 = I64::from_limbs([u64::MAX - 2]);
    const MINUS_TWO: I64 = I64::from_limbs([u64::MAX - 1]);
    const MINUS_ONE: I64 = I64::from_limbs([u64::MAX]);

    const ONE: I64 = I64::from_limbs([1]);
    const TWO: I64 = I64::from_limbs([2]);
    const THREE: I64 = I64::from_limbs([3]);
    const FOUR: I64 = I64::from_limbs([4]);
    const FIVE: I64 = I64::from_limbs([5]);
    const SIX: I64 = I64::from_limbs([6]);
    const SEVEN: I64 = I64::from_limbs([7]);
    const EIGHT: I64 = I64::from_limbs([8]);
    const NINE: I64 = I64::from_limbs([9]);
    const TEN: I64 = I64::from_limbs([10]);
    const TWENTY: I64 = I64::from_limbs([20]);

    #[test]
    fn signed_bit_operators() {
        let val1 = I32::from_limbs([0b10000001]); //0b10000001
        let val2 = I48::from_limbs([0b111]); //0b111

        // xor
        let expected = I64::from_limbs([0b10000110]);
        assert_eq!(expected, val1.cast() ^ val2.cast());

        // or
        let expected = I64::from_limbs([0b10000111]);
        assert_eq!(expected, val1.cast() | val2.cast());

        // and
        let expected = I64::from_limbs([0b1]);
        assert_eq!(expected, val1.cast() & val2.cast());

        // not
        assert_eq!(MINUS_EIGHT, (!val2).cast());
    }

    #[test]
    fn comparisons() {
        let val1 = I8::from_limbs([0b10000001]); // -127
        let val2 = I48::from_limbs([0b111]); // 7

        assert!(val1.cast() < val2);
        assert!(val1.cast() <= val2);
        assert!(val1 <= val1);
        assert!(val2 > val1.cast());
        assert!(val2 >= val1.cast());
        assert!(val2 >= val2);
        assert!(val2 != val1.cast());
        assert!(val1 == val1);
    }

    #[test]
    fn shifts() {
        let v = I16::from_limbs_slice(&[0b100]); // 1

        assert_eq!(I16::from_limbs([0b0000000000001000]), v << 1);
        assert_eq!(I16::from_limbs([0b0001000000000000]), v << 10);
        assert_eq!(I16::from_limbs([0b1000000000000000]), v << 13);
        assert_eq!(I16::from_limbs([0b0000000000000000]), v << 14);

        assert_eq!(I16::from_limbs([0b0000000000000010]), v >> 1);
        assert_eq!(I16::from_limbs([0b0000000000000000]), v >> 3);
        assert_eq!(I16::from_limbs([0b0000000000000000]), v >> 10);
    }

    #[test]
    fn arithmetics() {
        assert_eq!(TWO.pow(U64::from_limbs([3])), EIGHT); //  2**3= 8
        assert_eq!(MINUS_TWO.pow(U64::from_limbs([3])), MINUS_EIGHT); // -2**3=-8
        assert_eq!(MINUS_TWO.pow(U64::from_limbs([2])), FOUR); // -2**2= 4

        assert_eq!(TEN + TEN, TWENTY); //10 +  10 = 20
        assert_eq!(ONE + MINUS_TEN, MINUS_NINE); //1  + -10 = -9
        assert_eq!(MINUS_FOUR + SEVEN, THREE); //-4  + 7 = 3

        assert_eq!(TWENTY - TEN, TEN); //20 - 10 = 10
        assert_eq!(TEN - TWENTY, MINUS_TEN); //10 - 20 = -10
        assert_eq!(MINUS_SEVEN - ONE, MINUS_EIGHT); //-7 -  1 = -8
        assert_eq!(MINUS_ONE - MINUS_FIVE, FOUR); //-1 - -5 =  4

        assert_eq!(TWENTY * TEN, I64::from_limbs([200]));
        assert_eq!(ONE * MINUS_EIGHT, MINUS_EIGHT);
        assert_eq!(MINUS_THREE * MINUS_THREE, NINE);

        assert_eq!(TEN / TWO, FIVE); // 10 / 2  = 5
        assert_eq!(TEN / TEN, ONE); // 10 / 10 = 1
        assert_eq!(FIVE / TEN, I64::ZERO); // 5 / 10 = 0
        assert_eq!(TEN / MINUS_TWO, MINUS_FIVE); // 10 / -2 = -5
        assert_eq!(MINUS_TEN / MINUS_TWO, FIVE); // -10 / -2 = 5

        assert_eq!(MINUS_TEN % MINUS_THREE, MINUS_ONE); // -10 % -3 = -1
        assert_eq!(TEN % THREE, ONE); //  10 %  3 =  1
        assert_eq!(TEN % TWENTY, TEN); //  10 %  20 = 10
        assert_eq!(MINUS_TEN % MINUS_TWENTY, MINUS_TEN); // -10 % -20 = -10

        assert_eq!(-MINUS_EIGHT, EIGHT); // --8==8
        assert_eq!(-SIX, MINUS_SIX); // -6==6
    }
}
