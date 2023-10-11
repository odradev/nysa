use core::cmp::{Ord, Ordering, PartialOrd};

use super::utils;

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, Hash, derive_more::Deref, derive_more::DerefMut,
)]
pub struct Unsigned<const BITS: usize, const LIMBS: usize>(pub(crate) ruint::Uint<BITS, LIMBS>);

impl<const BITS: usize, const LIMBS: usize> Unsigned<BITS, LIMBS> {
    pub const MIN: Self = utils::zero();
    pub const MAX: Self = utils::max();
    pub const ZERO: Self = utils::zero();
    pub const ONE: Self = utils::one();

    pub const fn from_limbs(limbs: [u64; LIMBS]) -> Self {
        Self(ruint::Uint::<BITS, LIMBS>::from_limbs(limbs))
    }

    pub fn from_limbs_slice(slice: &[u64]) -> Self {
        Self(ruint::Uint::<BITS, LIMBS>::from_limbs_slice(slice))
    }

    pub fn pow(self, exp: Self) -> Self {
        Self(self.0.pow(exp.0))
    }

    pub fn from<T>(value: T) -> Self
    where
        ruint::Uint<BITS, LIMBS>: ruint::UintTryFrom<T>,
    {
        Self(ruint::Uint::from(value))
    }

    pub fn cast<const B: usize, const L: usize>(self) -> Unsigned<B, L> {
        Unsigned(ruint::Uint::from(self.0))
    }
}

impl<const BITS: usize, const LIMBS: usize> Ord for Unsigned<BITS, LIMBS> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> PartialOrd for Unsigned<BITS, LIMBS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
