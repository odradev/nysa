use super::{utils::*, Sign};
use core::fmt;
use ruint::Uint;

/// Signed integer wrapping a `ruint::Uint`.
///
/// This signed integer implementation is fully abstract across the number of
/// bits. It wraps a [`ruint::Uint`], and co-opts the most significant bit to
/// represent the sign. The number is represented in two's complement, using the
/// underlying `Uint`'s `u64` limbs. The limbs can be accessed via the
/// [`Signed::as_limbs()`] method, and are least-significant first.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Signed<const BITS: usize, const LIMBS: usize>(pub(crate) Uint<BITS, LIMBS>);

// formatting
impl<const BITS: usize, const LIMBS: usize> fmt::Debug for Signed<BITS, LIMBS> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::Display for Signed<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (sign, abs) = self.into_sign_and_abs();
        // sign must be formatted directly, instead of with `write!` due to the
        // `sign_positive` flag
        sign.fmt(f)?;
        write!(f, "{abs}")
    }
}

impl<const BITS: usize, const LIMBS: usize> Signed<BITS, LIMBS> {
    /// Mask for the highest limb.
    pub(crate) const MASK: u64 = mask(BITS);

    /// Location of the sign bit within the highest limb.
    pub(crate) const SIGN_BIT: u64 = sign_bit(BITS);

    pub const MIN: Self = min();
    pub const MAX: Self = max();
    pub const ZERO: Self = zero();
    pub const ONE: Self = one();
    pub const MINUS_ONE: Self = Self(Uint::<BITS, LIMBS>::MAX);

    /// Returns the sign of self.
    #[inline(always)]
    pub const fn sign(self) -> Sign {
        // if the last limb contains the sign bit, then we're negative
        // because we can't set any higher bits to 1, we use >= as a proxy
        // check to avoid bit comparison
        if let Some(limb) = self.0.as_limbs().last() {
            if *limb >= Self::SIGN_BIT {
                return Sign::Negative;
            }
        }
        Sign::Positive
    }

    /// Compile-time equality. NOT constant-time equality.
    #[inline]
    pub const fn const_eq(&self, other: &Self) -> bool {
        const_eq(self, other)
    }

    /// Returns `true` if `self` is zero and `false` if the number is negative
    /// or positive.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.const_eq(&Self::ZERO)
    }

    /// Returns `true` if `self` is positive and `false` if the number is zero
    /// or negative.
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        !self.is_zero() && matches!(self.sign(), Sign::Positive)
    }

    /// Returns `true` if `self` is negative and `false` if the number is zero
    /// or positive.
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        matches!(self.sign(), Sign::Negative)
    }

    /// Returns the number of ones in the binary representation of `self`.
    #[inline(always)]
    pub fn count_ones(&self) -> usize {
        self.0.count_ones()
    }

    /// Returns the number of zeros in the binary representation of `self`.
    #[inline(always)]
    pub fn count_zeros(&self) -> usize {
        self.0.count_zeros()
    }

    /// Returns the number of leading zeros in the binary representation of
    /// `self`.
    #[inline(always)]
    pub fn leading_zeros(&self) -> usize {
        self.0.leading_zeros()
    }

    /// Returns the number of leading zeros in the binary representation of
    /// `self`.
    #[inline(always)]
    pub fn trailing_zeros(&self) -> usize {
        self.0.trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of
    /// `self`.
    #[inline(always)]
    pub fn trailing_ones(&self) -> usize {
        self.0.trailing_ones()
    }

    /// Return if specific bit is set.
    ///
    /// # Panics
    ///
    /// If index exceeds the bit width of the number.
    #[inline(always)]
    pub const fn bit(&self, index: usize) -> bool {
        self.0.bit(index)
    }

    /// Return specific byte.
    ///
    /// # Panics
    ///
    /// If index exceeds the byte width of the number.
    #[inline(always)]
    pub const fn byte(&self, index: usize) -> u8 {
        let limbs = self.0.as_limbs();
        match index {
            0..=7 => limbs[3].to_be_bytes()[index],
            8..=15 => limbs[2].to_be_bytes()[index - 8],
            16..=23 => limbs[1].to_be_bytes()[index - 16],
            24..=31 => limbs[0].to_be_bytes()[index - 24],
            _ => panic!(),
        }
    }

    /// Return the least number of bits needed to represent the number.
    #[inline(always)]
    pub fn bits(self) -> u32 {
        let unsigned = self.unsigned_abs();
        let unsigned_bits = unsigned.bit_len();

        // NOTE: We need to deal with two special cases:
        //   - the number is 0
        //   - the number is a negative power of `2`. These numbers are written as
        //     `0b11..1100..00`.
        //   In the case of a negative power of two, the number of bits required
        //   to represent the negative signed value is equal to the number of
        //   bits required to represent its absolute value as an unsigned
        //   integer. This is best illustrated by an example: the number of bits
        //   required to represent `-128` is `8` since it is equal to `i8::MIN`
        //   and, therefore, obviously fits in `8` bits. This is equal to the
        //   number of bits required to represent `128` as an unsigned integer
        //   (which fits in a `u8`).  However, the number of bits required to
        //   represent `128` as a signed integer is `9`, as it is greater than
        //   `i8::MAX`.  In the general case, an extra bit is needed to
        //   represent the sign.
        let bits = if self.count_zeros() == self.trailing_zeros() {
            // `self` is zero or a negative power of two
            unsigned_bits
        } else {
            unsigned_bits + 1
        };

        bits as u32
    }

    /// Creates a `Signed` from a sign and an absolute value. Returns the value
    /// and a bool that is true if the conversion caused an overflow.
    #[inline(always)]
    pub fn overflowing_from_sign_and_abs(sign: Sign, abs: Uint<BITS, LIMBS>) -> (Self, bool) {
        let value = Self(match sign {
            Sign::Positive => abs,
            Sign::Negative => twos_complement(abs),
        });

        (value, value.sign() != sign)
    }

    /// Creates a `Signed` from an absolute value and a negative flag. Returns
    /// `None` if it would overflow as `Signed`.
    #[inline(always)]
    pub fn checked_from_sign_and_abs(sign: Sign, abs: Uint<BITS, LIMBS>) -> Option<Self> {
        let (result, overflow) = Self::overflowing_from_sign_and_abs(sign, abs);
        if overflow {
            None
        } else {
            Some(result)
        }
    }

    /// Splits a Signed into its absolute value and negative flag.
    #[inline(always)]
    pub fn into_sign_and_abs(self) -> (Sign, Uint<BITS, LIMBS>) {
        let sign = self.sign();
        let abs = match sign {
            Sign::Positive => self.0,
            Sign::Negative => twos_complement(self.0),
        };
        (sign, abs)
    }

    #[inline(always)]
    pub fn to_be_bytes<const BYTES: usize>(self) -> [u8; BYTES] {
        self.0.to_be_bytes()
    }

    #[inline(always)]
    pub fn to_le_bytes<const BYTES: usize>(self) -> [u8; BYTES] {
        self.0.to_le_bytes()
    }

    #[inline(always)]
    pub fn from_be_bytes<const BYTES: usize>(bytes: [u8; BYTES]) -> Self {
        Self(Uint::from_be_bytes::<BYTES>(bytes))
    }

    #[inline(always)]
    pub fn from_le_bytes<const BYTES: usize>(bytes: [u8; BYTES]) -> Self {
        Self(Uint::from_le_bytes::<BYTES>(bytes))
    }

    pub fn try_from_be_slice(slice: &[u8]) -> Option<Self> {
        Some(Self(Uint::try_from_be_slice(slice)?))
    }

    pub fn try_from_le_slice(slice: &[u8]) -> Option<Self> {
        Some(Self(Uint::try_from_le_slice(slice)?))
    }

    pub const fn as_limbs(&self) -> &[u64; LIMBS] {
        self.0.as_limbs()
    }

    pub const fn into_limbs(self) -> [u64; LIMBS] {
        self.0.into_limbs()
    }

    pub const fn from_limbs(limbs: [u64; LIMBS]) -> Self {
        Self(Uint::from_limbs(limbs))
    }

    pub fn from_limbs_slice(slice: &[u64]) -> Self {
        Self(Uint::from_limbs_slice(slice))
    }

    pub fn cast<const B: usize, const L: usize>(self) -> Signed<B, L> {
        let (sign, abs) = self.into_sign_and_abs();

        let value = match sign {
            Sign::Negative => twos_complement(ruint::Uint::from(abs)),
            Sign::Positive => ruint::Uint::from(abs),
        };

        Signed(value)
    }
}
