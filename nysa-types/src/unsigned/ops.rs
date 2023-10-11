use core::ops::{
    self, Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

use crate::Unsigned;

macro_rules! impl_bin_op {
    ( $trait:ident, $fn:ident, $trait_assign:ident, $fn_assign:ident, $fdel:ident) => {
        impl<const BITS: usize, const LIMBS: usize> $trait_assign<Unsigned<BITS, LIMBS>>
            for Unsigned<BITS, LIMBS>
        {
            fn $fn_assign(&mut self, rhs: Unsigned<BITS, LIMBS>) {
                *self = Unsigned((**self).$fdel(*rhs));
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait_assign<&Unsigned<BITS, LIMBS>>
            for Unsigned<BITS, LIMBS>
        {
            fn $fn_assign(&mut self, rhs: &Unsigned<BITS, LIMBS>) {
                *self = Unsigned((**self).$fdel(**rhs));
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait<Unsigned<BITS, LIMBS>>
            for Unsigned<BITS, LIMBS>
        {
            type Output = Unsigned<BITS, LIMBS>;

            fn $fn(self, rhs: Unsigned<BITS, LIMBS>) -> Self::Output {
                Unsigned((*self).$fdel(*rhs))
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait<&Unsigned<BITS, LIMBS>>
            for Unsigned<BITS, LIMBS>
        {
            type Output = Unsigned<BITS, LIMBS>;

            fn $fn(self, rhs: &Unsigned<BITS, LIMBS>) -> Self::Output {
                Unsigned((*self).$fdel(**rhs))
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait<Unsigned<BITS, LIMBS>>
            for &Unsigned<BITS, LIMBS>
        {
            type Output = Unsigned<BITS, LIMBS>;

            fn $fn(self, rhs: Unsigned<BITS, LIMBS>) -> Self::Output {
                Unsigned((**self).$fdel(*rhs))
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait<&Unsigned<BITS, LIMBS>>
            for &Unsigned<BITS, LIMBS>
        {
            type Output = Unsigned<BITS, LIMBS>;

            fn $fn(self, rhs: &Unsigned<BITS, LIMBS>) -> Self::Output {
                Unsigned((**self).$fdel(**rhs))
            }
        }
    };
}

impl_bin_op!(Add, add, AddAssign, add_assign, wrapping_add);
impl_bin_op!(Sub, sub, SubAssign, sub_assign, wrapping_sub);
impl_bin_op!(Div, div, DivAssign, div_assign, wrapping_div);
impl_bin_op!(Rem, rem, RemAssign, rem_assign, wrapping_rem);
impl_bin_op!(Mul, mul, MulAssign, mul_assign, wrapping_mul);

impl<const BITS: usize, const LIMBS: usize> ops::BitAnd for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitAndAssign for Unsigned<BITS, LIMBS> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitOr for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitOrAssign for Unsigned<BITS, LIMBS> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitXor for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitXorAssign for Unsigned<BITS, LIMBS> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Neg for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(self.wrapping_neg())
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Not for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::ShlAssign<usize> for Unsigned<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl_assign(&mut self, rhs: usize) {
        **self = self.wrapping_shl(rhs);
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::ShlAssign<&usize> for Unsigned<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl_assign(&mut self, rhs: &usize) {
        **self = self.wrapping_shl(*rhs);
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shl<usize> for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: usize) -> Self {
        Self(self.wrapping_shl(rhs))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shl<usize> for &Unsigned<BITS, LIMBS> {
    type Output = Unsigned<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: usize) -> Self::Output {
        Unsigned(self.wrapping_shl(rhs))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shl<&usize> for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: &usize) -> Self {
        Self(self.wrapping_shl(*rhs))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shl<&usize> for &Unsigned<BITS, LIMBS> {
    type Output = Unsigned<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: &usize) -> Self::Output {
        Unsigned(self.wrapping_shl(*rhs))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::ShrAssign<usize> for Unsigned<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr_assign(&mut self, rhs: usize) {
        **self = self.wrapping_shr(rhs);
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::ShrAssign<&usize> for Unsigned<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr_assign(&mut self, rhs: &usize) {
        **self = self.wrapping_shr(*rhs);
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shr<usize> for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: usize) -> Self {
        Self(self.wrapping_shr(rhs))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shr<usize> for &Unsigned<BITS, LIMBS> {
    type Output = Unsigned<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: usize) -> Self::Output {
        Unsigned(self.wrapping_shr(rhs))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shr<&Unsigned<BITS, LIMBS>> for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: &Unsigned<BITS, LIMBS>) -> Self {
        Self(self.wrapping_shr(rhs.to()))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shr<&Unsigned<BITS, LIMBS>> for &Unsigned<BITS, LIMBS> {
    type Output = Unsigned<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: &Unsigned<BITS, LIMBS>) -> Self::Output {
        Unsigned(self.wrapping_shr(rhs.to()))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::ShlAssign<Unsigned<BITS, LIMBS>> for Unsigned<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Unsigned<BITS, LIMBS>) {
        **self = self.wrapping_shl(rhs.to());
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::ShlAssign<&Unsigned<BITS, LIMBS>> for Unsigned<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl_assign(&mut self, rhs: &Unsigned<BITS, LIMBS>) {
        **self = self.wrapping_shl(rhs.to());
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shl<Unsigned<BITS, LIMBS>> for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: Unsigned<BITS, LIMBS>) -> Self {
        Self(self.wrapping_shl(rhs.to()))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shl<Unsigned<BITS, LIMBS>> for &Unsigned<BITS, LIMBS> {
    type Output = Unsigned<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: Unsigned<BITS, LIMBS>) -> Self::Output {
        Unsigned(self.wrapping_shl(rhs.to()))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shl<&Unsigned<BITS, LIMBS>> for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: &Unsigned<BITS, LIMBS>) -> Self {
        Self(self.wrapping_shl(rhs.to()))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shl<&Unsigned<BITS, LIMBS>> for &Unsigned<BITS, LIMBS> {
    type Output = Unsigned<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: &Unsigned<BITS, LIMBS>) -> Self::Output {
        Unsigned(self.wrapping_shl(rhs.to()))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::ShrAssign<Unsigned<BITS, LIMBS>> for Unsigned<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Unsigned<BITS, LIMBS>) {
        **self = self.wrapping_shr(rhs.to());
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::ShrAssign<&Unsigned<BITS, LIMBS>> for Unsigned<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr_assign(&mut self, rhs: &Unsigned<BITS, LIMBS>) {
        **self = self.wrapping_shr(rhs.to());
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shr<Unsigned<BITS, LIMBS>> for Unsigned<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: Unsigned<BITS, LIMBS>) -> Self {
        Self(self.wrapping_shr(rhs.to()))
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Shr<Unsigned<BITS, LIMBS>> for &Unsigned<BITS, LIMBS> {
    type Output = Unsigned<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: Unsigned<BITS, LIMBS>) -> Self::Output {
        Unsigned(self.wrapping_shr(rhs.to()))
    }
}

// impl<const BITS: usize, const LIMBS: usize> ops::Shr<&Unsigned<BITS, LIMBS>> for Unsigned<BITS, LIMBS> {
//     type Output = Self;

//     #[allow(clippy::inline_always)]
//     #[inline(always)]
//     fn shr(self, rhs: &Unsigned<BITS, LIMBS>) -> Self {
//         Self(self.wrapping_shr(rhs.to()))
//     }
// }

// impl<const BITS: usize, const LIMBS: usize> ops::Shr<&Unsigned<BITS, LIMBS>> for &Unsigned<BITS, LIMBS> {
//     type Output = Unsigned<BITS, LIMBS>;

//     #[allow(clippy::inline_always)]
//     #[inline(always)]
//     fn shr(self, rhs: &Unsigned<BITS, LIMBS>) -> Self::Output {
//         Unsigned(self.wrapping_shr(rhs.to()))
//     }
// }
