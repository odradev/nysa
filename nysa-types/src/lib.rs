#![no_std]

extern crate alloc;

use core::cmp::{Ord, Ordering, PartialOrd};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[cfg(feature = "odra")]
mod odra;

macro_rules! uints {
    ( $($name:ident<$BITS:literal, $LIMBS:literal>),* ) => {

        $(
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
            pub struct $name(ruint::Uint<$BITS, $LIMBS>);

            impl core::ops::Deref for $name {
                type Target = ruint::Uint<$BITS, $LIMBS>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl core::ops::DerefMut for $name {

                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl $name {
                pub const MIN: Self = Self(ruint::Uint::<$BITS, $LIMBS>::ZERO);
                pub const MAX: Self = Self(ruint::Uint::<$BITS, $LIMBS>::MAX);
                pub const ZERO: Self = Self::from_limbs([0; $LIMBS]);

                pub const fn from_limbs(limbs: [u64; $LIMBS]) -> Self {
                    Self(ruint::Uint::<$BITS, $LIMBS>::from_limbs(limbs))
                }

                pub fn from_limbs_slice(slice: &[u64]) -> Self {
                    Self(ruint::Uint::<$BITS, $LIMBS>::from_limbs_slice(slice))
                }

                pub fn from<T>(value: T) -> $name
                where
                    ruint::Uint<$BITS, $LIMBS>: ruint::UintTryFrom<T>
                {
                    $name(ruint::Uint::from(value))
                }
            }

            impl Default for $name {
                #[inline]
                fn default() -> $name {
                    $name::ZERO
                }
            }

            impl Ord for $name {
                fn cmp(&self, rhs: &Self) -> Ordering {
                    self.0.cmp(&rhs.0)
                }
            }

            impl PartialOrd for $name {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }

        )*
    };
}

uints!(
    U8<8, 1>,
    U16<16, 1>,
    U24<24, 1>,
    U32<32, 1>,
    U40<40, 1>,
    U48<48, 1>,
    U56<56, 1>,
    U64<64, 1>,
    U72<72, 2>,
    U80<80, 2>,
    U88<88, 2>,
    U96<96, 2>,
    U104<104, 2>,
    U112<112, 2>,
    U120<120, 2>,
    U128<128, 2>,
    U136<136, 3>,
    U144<144, 3>,
    U152<152, 3>,
    U160<160, 3>,
    U168<168, 3>,
    U176<176, 3>,
    U184<184, 3>,
    U192<192, 3>,
    U200<200, 4>,
    U208<208, 4>,
    U216<216, 4>,
    U224<224, 4>,
    U232<232, 4>,
    U240<240, 4>,
    U248<248, 4>,
    U256<256, 4>
);

macro_rules! impl_bin_op {
    ( $ty: ident, $trait:ident, $fn:ident, $trait_assign:ident, $fn_assign:ident, $fdel:ident) => {
        impl $trait_assign<$ty> for $ty {
            fn $fn_assign(&mut self, rhs: $ty) {
                *self = $ty((**self).$fdel(*rhs));
            }
        }

        impl $trait_assign<&$ty> for $ty {
            fn $fn_assign(&mut self, rhs: &$ty) {
                *self = $ty((**self).$fdel(**rhs));
            }
        }

        impl $trait<$ty> for $ty {
            type Output = $ty;

            fn $fn(self, rhs: $ty) -> Self::Output {
                $ty((*self).$fdel(*rhs))
            }
        }

        impl $trait<&$ty> for $ty {
            type Output = $ty;

            fn $fn(self, rhs: &$ty) -> Self::Output {
                $ty((*self).$fdel(**rhs))
            }
        }

        impl $trait<$ty> for &$ty {
            type Output = $ty;

            fn $fn(self, rhs: $ty) -> Self::Output {
                $ty((**self).$fdel(*rhs))
            }
        }

        impl $trait<&$ty> for &$ty {
            type Output = $ty;

            fn $fn(self, rhs: &$ty) -> Self::Output {
                $ty((**self).$fdel(**rhs))
            }
        }
    };
}

macro_rules! impl_add {
    ( $($ty: ident),* ) => {
        $(impl_bin_op!($ty, Add, add, AddAssign, add_assign, wrapping_add);)*
    };
}
macro_rules! impl_sub {
    ( $($ty: ident),* ) => {
        $(impl_bin_op!($ty, Sub, sub, SubAssign, sub_assign, wrapping_sub);)*
    };
}
macro_rules! impl_div {
    ( $($ty: ident),* ) => {
        $(impl_bin_op!($ty, Div, div, DivAssign, div_assign, wrapping_div);)*
    };
}
macro_rules! impl_rem {
    ( $($ty: ident),* ) => {
        $(impl_bin_op!($ty, Rem, rem, RemAssign, rem_assign, wrapping_rem);)*
    };
}
macro_rules! impl_mul {
( $($ty: ident),* ) => {
        $(impl_bin_op!($ty, Mul, mul, MulAssign, mul_assign, wrapping_mul);)*
    };
}

macro_rules! impl_bin_ops {
    ( $($ty: ident),* ) => {
        impl_add!( $($ty),*);
        impl_sub!( $($ty),*);
        impl_div!( $($ty),*);
        impl_rem!( $($ty),*);
        impl_mul!( $($ty),*);
    };
}

impl_bin_ops!(
    U8, U16, U24, U32, U40, U48, U56, U64, U72, U80, U88, U96, U104, U112, U120, U128, U136, U144,
    U152, U160, U168, U176, U184, U192, U200, U208, U216, U224, U232, U240, U248, U256
);
