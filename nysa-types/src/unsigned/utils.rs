use crate::Unsigned;

pub(super) const fn one<const BITS: usize, const LIMBS: usize>() -> Unsigned<BITS, LIMBS> {
    let mut limbs = [0; LIMBS];
    limbs[0] = 1;
    Unsigned(ruint::Uint::from_limbs(limbs))
}

pub(super) const fn zero<const BITS: usize, const LIMBS: usize>() -> Unsigned<BITS, LIMBS> {
    Unsigned(ruint::Uint::<BITS, LIMBS>::ZERO)
}

pub(super) const fn max<const BITS: usize, const LIMBS: usize>() -> Unsigned<BITS, LIMBS> {
    Unsigned(ruint::Uint::<BITS, LIMBS>::MAX)
}