use crate::Unsigned;

use super::{utils::twos_complement, Sign, Signed};
use core::{cmp, ops};
use ruint::Uint;

impl<const BITS: usize, const LIMBS: usize> Signed<BITS, LIMBS> {
    #[inline(always)]
    #[must_use]
    pub fn overflowing_abs(self) -> (Self, bool) {
        if BITS == 0 {
            return (self, false);
        }
        if self == Self::MIN {
            (self, true)
        } else {
            (Self(self.unsigned_abs()), false)
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_abs(self) -> Option<Self> {
        match self.overflowing_abs() {
            (value, false) => Some(value),
            _ => None,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn saturating_abs(self) -> Self {
        match self.overflowing_abs() {
            (value, false) => value,
            _ => Self::MAX,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_abs(self) -> Self {
        self.overflowing_abs().0
    }

    #[inline(always)]
    #[must_use]
    pub fn unsigned_abs(self) -> Uint<BITS, LIMBS> {
        self.into_sign_and_abs().1
    }

    #[inline(always)]
    #[must_use]
    pub fn overflowing_neg(self) -> (Self, bool) {
        if BITS == 0 {
            return (self, false);
        }
        if self == Self::MIN {
            (self, true)
        } else {
            (Self(twos_complement(self.0)), false)
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_neg(self) -> Option<Self> {
        match self.overflowing_neg() {
            (value, false) => Some(value),
            _ => None,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn saturating_neg(self) -> Self {
        match self.overflowing_neg() {
            (value, false) => value,
            _ => Self::MAX,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }

    #[inline(always)]
    #[must_use]
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (unsigned, _) = self.0.overflowing_add(rhs.0);
        let result = Self(unsigned);

        // NOTE: Overflow is determined by checking the sign of the operands and
        //   the result.
        let overflow = matches!(
            (self.sign(), rhs.sign(), result.sign()),
            (Sign::Positive, Sign::Positive, Sign::Negative)
                | (Sign::Negative, Sign::Negative, Sign::Positive)
        );

        (result, overflow)
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.overflowing_add(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn saturating_add(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_add(rhs);
        if overflow {
            match result.sign() {
                Sign::Positive => Self::MIN,
                Sign::Negative => Self::MAX,
            }
        } else {
            result
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }

    #[inline(always)]
    #[must_use]
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (unsigned, _) = self.0.overflowing_sub(rhs.0);
        let result = Self(unsigned);

        // NOTE: Overflow is determined by checking the sign of the operands and
        //   the result.
        let overflow = matches!(
            (self.sign(), rhs.sign(), result.sign()),
            (Sign::Positive, Sign::Negative, Sign::Negative)
                | (Sign::Negative, Sign::Positive, Sign::Positive)
        );

        (result, overflow)
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.overflowing_sub(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_sub(rhs);
        if overflow {
            match result.sign() {
                Sign::Positive => Self::MIN,
                Sign::Negative => Self::MAX,
            }
        } else {
            result
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }

    #[inline(always)]
    #[must_use]
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        if self.is_zero() || rhs.is_zero() {
            return (Self::ZERO, false);
        }
        let sign = self.sign() * rhs.sign();
        let (unsigned, overflow_mul) = self.unsigned_abs().overflowing_mul(rhs.unsigned_abs());
        let (result, overflow_conv) = Self::overflowing_from_sign_and_abs(sign, unsigned);

        (result, overflow_mul || overflow_conv)
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.overflowing_mul(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_mul(rhs);
        if overflow {
            match self.sign() * rhs.sign() {
                Sign::Positive => Self::MAX,
                Sign::Negative => Self::MIN,
            }
        } else {
            result
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }

    #[inline(always)]
    #[must_use]
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        assert!(!rhs.is_zero(), "attempt to divide by zero");
        let sign = self.sign() * rhs.sign();

        let unsigned = self.unsigned_abs() / rhs.unsigned_abs();
        let (result, overflow_conv) = Self::overflowing_from_sign_and_abs(sign, unsigned);

        (result, overflow_conv && !result.is_zero())
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::MIN && rhs == Self::MINUS_ONE) {
            None
        } else {
            Some(self.overflowing_div(rhs).0)
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn saturating_div(self, rhs: Self) -> Self {
        match self.overflowing_div(rhs) {
            (value, false) => value,
            // MIN / -1 is the only possible saturating overflow
            _ => Self::MAX,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_div(self, rhs: Self) -> Self {
        self.overflowing_div(rhs).0
    }

    #[inline(always)]
    #[must_use]
    pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        if self == Self::MIN && rhs == Self::MINUS_ONE {
            (Self::ZERO, true)
        } else {
            let div_res = self / rhs;
            (self - div_res * rhs, false)
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::MIN && rhs == Self::MINUS_ONE) {
            None
        } else {
            Some(self.overflowing_rem(rhs).0)
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_rem(self, rhs: Self) -> Self {
        self.overflowing_rem(rhs).0
    }

    #[inline(always)]
    const fn pow_sign(self, exp: Uint<BITS, LIMBS>) -> Sign {
        let is_exp_odd = BITS != 0 && exp.as_limbs()[0] % 2 == 1;
        if is_exp_odd && self.is_negative() {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn pow(self, exp: Unsigned<BITS, LIMBS>) -> Self {
        self.wrapping_pow(exp.0)
    }

    #[inline(always)]
    #[must_use]
    pub fn overflowing_pow(self, exp: Uint<BITS, LIMBS>) -> (Self, bool) {
        if BITS == 0 {
            return (Self::ZERO, false);
        }

        let sign = self.pow_sign(exp);

        let (unsigned, overflow_pow) = self.unsigned_abs().overflowing_pow(exp);
        let (result, overflow_conv) = Self::overflowing_from_sign_and_abs(sign, unsigned);

        (result, overflow_pow || overflow_conv)
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_pow(self, exp: Uint<BITS, LIMBS>) -> Option<Self> {
        let (result, overflow) = self.overflowing_pow(exp);
        if overflow {
            None
        } else {
            Some(result)
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn saturating_pow(self, exp: Uint<BITS, LIMBS>) -> Self {
        let (result, overflow) = self.overflowing_pow(exp);
        if overflow {
            match self.pow_sign(exp) {
                Sign::Positive => Self::MAX,
                Sign::Negative => Self::MIN,
            }
        } else {
            result
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_pow(self, exp: Uint<BITS, LIMBS>) -> Self {
        self.overflowing_pow(exp).0
    }

    #[inline(always)]
    #[must_use]
    pub fn overflowing_shl(self, rhs: usize) -> (Self, bool) {
        if rhs >= 256 {
            (Self::ZERO, true)
        } else {
            (Self(self.0 << rhs), false)
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_shl(self, rhs: usize) -> Option<Self> {
        match self.overflowing_shl(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_shl(self, rhs: usize) -> Self {
        self.overflowing_shl(rhs).0
    }

    #[inline(always)]
    #[must_use]
    pub fn overflowing_shr(self, rhs: usize) -> (Self, bool) {
        if rhs >= 256 {
            (Self::ZERO, true)
        } else {
            (Self(self.0 >> rhs), false)
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn checked_shr(self, rhs: usize) -> Option<Self> {
        match self.overflowing_shr(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn wrapping_shr(self, rhs: usize) -> Self {
        self.overflowing_shr(rhs).0
    }

    #[inline(always)]
    #[must_use]
    pub fn asr(self, rhs: usize) -> Self {
        // Avoid shifting if we are going to know the result regardless of the value.
        if rhs == 0 || BITS == 0 {
            return self;
        }

        if rhs >= BITS - 1 {
            match self.sign() {
                Sign::Positive => return Self::ZERO,
                Sign::Negative => return Self::MINUS_ONE,
            }
        }

        match self.sign() {
            // Perform the shift.
            Sign::Positive => self.wrapping_shr(rhs),
            Sign::Negative => {
                // We need to do: `for 0..shift { self >> 1 | 2^255 }`
                // We can avoid the loop by doing: `self >> shift | ~(2^(255 - shift) - 1)`
                // where '~' represents ones complement
                let two: Uint<BITS, LIMBS> = Uint::from(2);
                let bitwise_or = Self(
                    !(two.pow(Uint::<BITS, LIMBS>::from(BITS - rhs))
                        - Uint::<BITS, LIMBS>::from(1)),
                );
                (self.wrapping_shr(rhs)) | bitwise_or
            }
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn asl(self, rhs: usize) -> Option<Self> {
        if rhs == 0 || BITS == 0 {
            Some(self)
        } else {
            let result = self.wrapping_shl(rhs);
            if result.sign() != self.sign() {
                // Overflow occurred
                None
            } else {
                Some(result)
            }
        }
    }
}

// Implement Shl and Shr only for types <= usize, since U256 uses .as_usize()
// which panics
macro_rules! impl_shift {
    ($($t:ty),+) => {
        // We are OK with wrapping behavior here because it's how Rust behaves with the primitive
        // integer types.

        // $t <= usize: cast to usize
        $(
            impl<const BITS: usize, const LIMBS: usize> ops::Shl<$t> for Signed<BITS, LIMBS> {
                type Output = Self;

                #[inline(always)]
                fn shl(self, rhs: $t) -> Self::Output {
                    self.wrapping_shl(rhs as usize)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> ops::ShlAssign<$t> for Signed<BITS, LIMBS> {
                #[inline(always)]
                fn shl_assign(&mut self, rhs: $t) {
                    *self = *self << rhs;
                }
            }

            impl<const BITS: usize, const LIMBS: usize> ops::Shr<$t> for Signed<BITS, LIMBS> {
                type Output = Self;

                #[inline(always)]
                fn shr(self, rhs: $t) -> Self::Output {
                    self.wrapping_shr(rhs as usize)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> ops::ShrAssign<$t> for Signed<BITS, LIMBS> {
                #[inline(always)]
                fn shr_assign(&mut self, rhs: $t) {
                    *self = *self >> rhs;
                }
            }
        )+
    };
}

impl_shift!(i8, u8, i16, u16, i32, u32, isize, usize);

// cmp
impl<const BITS: usize, const LIMBS: usize> cmp::PartialOrd for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BITS: usize, const LIMBS: usize> cmp::Ord for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use cmp::Ordering::*;
        use Sign::*;

        match (self.into_sign_and_abs(), other.into_sign_and_abs()) {
            ((Positive, _), (Negative, _)) => Greater,
            ((Negative, _), (Positive, _)) => Less,
            ((Positive, this), (Positive, other)) => this.cmp(&other),
            ((Negative, this), (Negative, other)) => other.cmp(&this),
        }
    }
}

// arithmetic ops - implemented above
impl<T, const BITS: usize, const LIMBS: usize> ops::Add<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self.wrapping_add(rhs.into())
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::AddAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::Sub<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        self.wrapping_sub(rhs.into())
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::SubAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::Mul<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.wrapping_mul(rhs.into())
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::MulAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::Div<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self.wrapping_div(rhs.into())
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::DivAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::Rem<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        self.wrapping_rem(rhs.into())
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::RemAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Self>,
{
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs;
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitAnd for Signed<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitAndAssign for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitOr for Signed<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitOrAssign for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitXor for Signed<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitXorAssign for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Neg for Signed<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        self.wrapping_neg()
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Not for Signed<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
