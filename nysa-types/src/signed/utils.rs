use crate::signed::Signed;
use ruint::Uint;

/// Two's complement is a method used in binary arithmetic to represent signed integers.
/// In this system, the most significant bit (leftmost bit) represents the sign of the number,
/// where 0 typically indicates a non-negative (positive or zero) value, and 1 indicates a negative value.
///
/// Here's how two's complement works:
///
/// ### Positive Numbers:
/// For non-negative numbers (including zero), the leftmost bit is 0, and the rest of the bits represent
/// the magnitude of the number using the standard binary representation.
///
/// ### Negative Numbers:
/// For negative numbers, the leftmost bit is 1, and the rest of the bits represent
/// the magnitude of the number in a way that allows for easy addition and subtraction.
///
/// To represent a negative number -x using two's complement:
///
/// Find the binary representation of the positive counterpart of x (let's call it y).
/// Invert all the bits in y to obtain the one's complement of y.
/// Add 1 to the one's complement to obtain the two's complement representation.
#[inline(always)]
pub(super) fn twos_complement<const BITS: usize, const LIMBS: usize>(
    u: Uint<BITS, LIMBS>,
) -> Uint<BITS, LIMBS> {
    if BITS == 0 {
        return u;
    }
    (!u).overflowing_add(Uint::<BITS, LIMBS>::from(1)).0
}

/// Compile-time equality of signed integers.
#[inline(always)]
pub(super) const fn const_eq<const BITS: usize, const LIMBS: usize>(
    left: &Signed<BITS, LIMBS>,
    right: &Signed<BITS, LIMBS>,
) -> bool {
    if BITS == 0 {
        return true;
    }

    let mut i = 0;
    let llimbs = left.0.as_limbs();
    let rlimbs = right.0.as_limbs();
    while i < LIMBS {
        if llimbs[i] != rlimbs[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Compute the max value at compile time.
pub(super) const fn max<const BITS: usize, const LIMBS: usize>() -> Signed<BITS, LIMBS> {
    if LIMBS == 0 {
        return zero();
    }

    let mut limbs = [u64::MAX; LIMBS];
    limbs[LIMBS - 1] &= Signed::<BITS, LIMBS>::MASK; // unset all high bits
    limbs[LIMBS - 1] &= !Signed::<BITS, LIMBS>::SIGN_BIT; // unset the sign bit
    Signed(Uint::from_limbs(limbs))
}

pub(super) const fn min<const BITS: usize, const LIMBS: usize>() -> Signed<BITS, LIMBS> {
    if LIMBS == 0 {
        return zero();
    }

    let mut limbs = [0; LIMBS];
    limbs[LIMBS - 1] = Signed::<BITS, LIMBS>::SIGN_BIT;
    Signed(Uint::from_limbs(limbs))
}

pub(super) const fn zero<const BITS: usize, const LIMBS: usize>() -> Signed<BITS, LIMBS> {
    let limbs = [0; LIMBS];
    Signed(Uint::from_limbs(limbs))
}

pub(super) const fn one<const BITS: usize, const LIMBS: usize>() -> Signed<BITS, LIMBS> {
    if LIMBS == 0 {
        return zero();
    }

    let mut limbs = [0; LIMBS];
    limbs[0] = 1;
    Signed(Uint::from_limbs(limbs))
}

/// Location of the sign bit within the highest limb.
pub(super) const fn sign_bit(bits: usize) -> u64 {
    if bits == 0 {
        return 0;
    }
    let bits = bits % 64;
    if bits == 0 {
        1 << 63
    } else {
        1 << (bits - 1)
    }
}

/// Mask to apply to the highest limb to get the correct number of bits.
#[must_use]
pub(super) const fn mask(bits: usize) -> u64 {
    if bits == 0 {
        return 0;
    }
    let bits = bits % 64;
    if bits == 0 {
        u64::MAX
    } else {
        (1 << bits) - 1
    }
}
