use core::{iter, ops, str};

use derive_more::{Deref, DerefMut, From, Index, IndexMut, IntoIterator};

mod formatting;

/// A byte array of fixed length (`[u8; N]`).
///
/// This type allows to control serialization, deserialization or bitwise arithmetic on fixed-length
/// byte arrays.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deref,
    DerefMut,
    From,
    Index,
    IndexMut,
    IntoIterator,
)]
#[repr(transparent)]
pub struct FixedBytes<const N: usize>(#[into_iterator(owned, ref, ref_mut)] pub [u8; N]);

// crate::impl_fb_traits!(FixedBytes<N>, N, const);

impl<const N: usize> Default for FixedBytes<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const N: usize> From<&[u8; N]> for FixedBytes<N> {
    #[inline]
    fn from(bytes: &[u8; N]) -> Self {
        Self(*bytes)
    }
}

impl<const N: usize> From<&mut [u8; N]> for FixedBytes<N> {
    #[inline]
    fn from(bytes: &mut [u8; N]) -> Self {
        Self(*bytes)
    }
}

impl<const N: usize> TryFrom<&[u8]> for FixedBytes<N> {
    type Error = core::array::TryFromSliceError;

    #[inline]
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        <&Self>::try_from(slice).copied()
    }
}

impl<const N: usize> TryFrom<&mut [u8]> for FixedBytes<N> {
    type Error = core::array::TryFromSliceError;

    #[inline]
    fn try_from(slice: &mut [u8]) -> Result<Self, Self::Error> {
        Self::try_from(&*slice)
    }
}

impl<'a, const N: usize> TryFrom<&'a [u8]> for &'a FixedBytes<N> {
    type Error = core::array::TryFromSliceError;

    #[inline]
    fn try_from(slice: &'a [u8]) -> Result<&'a FixedBytes<N>, Self::Error> {
        // SAFETY: `FixedBytes<N>` is `repr(transparent)` for `[u8; N]`
        <&[u8; N]>::try_from(slice).map(|array_ref| unsafe { core::mem::transmute(array_ref) })
    }
}

impl<'a, const N: usize> TryFrom<&'a mut [u8]> for &'a mut FixedBytes<N> {
    type Error = core::array::TryFromSliceError;

    #[inline]
    fn try_from(slice: &'a mut [u8]) -> Result<&'a mut FixedBytes<N>, Self::Error> {
        // SAFETY: `FixedBytes<N>` is `repr(transparent)` for `[u8; N]`
        <&mut [u8; N]>::try_from(slice).map(|array_ref| unsafe { core::mem::transmute(array_ref) })
    }
}

impl<const N: usize> From<FixedBytes<N>> for [u8; N] {
    #[inline]
    fn from(s: FixedBytes<N>) -> Self {
        s.0
    }
}

impl<const N: usize> AsRef<[u8; N]> for FixedBytes<N> {
    #[inline]
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8; N]> for FixedBytes<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8]> for FixedBytes<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for FixedBytes<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<const N: usize> ops::BitAnd for FixedBytes<N> {
    type Output = Self;

    #[inline]
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<const N: usize> ops::BitAndAssign for FixedBytes<N> {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        // Note: `slice::Iter` has better codegen than `array::IntoIter`
        iter::zip(self, &rhs).for_each(|(a, b)| *a &= *b);
    }
}

impl<const N: usize> ops::BitOr for FixedBytes<N> {
    type Output = Self;

    #[inline]
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<const N: usize> ops::BitOrAssign for FixedBytes<N> {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        // Note: `slice::Iter` has better codegen than `array::IntoIter`
        iter::zip(self, &rhs).for_each(|(a, b)| *a |= *b);
    }
}

impl<const N: usize> ops::BitXor for FixedBytes<N> {
    type Output = Self;

    #[inline]
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}

impl<const N: usize> ops::BitXorAssign for FixedBytes<N> {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        // Note: `slice::Iter` has better codegen than `array::IntoIter`
        iter::zip(self, &rhs).for_each(|(a, b)| *a ^= *b);
    }
}

impl<const N: usize> str::FromStr for FixedBytes<N> {
    type Err = const_hex::FromHexError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = [0u8; N];
        const_hex::decode_to_slice(s, &mut buf)?;
        Ok(Self(buf))
    }
}

impl<const N: usize> FixedBytes<N> {
    pub const ZERO: Self = Self([0u8; N]);

    #[inline]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    /// Compile-time equality. NOT constant-time equality.
    pub const fn const_eq(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < N {
            if self.0[i] != other.0[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Returns `true` if no bits are set.
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    /// Returns `true` if no bits are set.
    #[inline]
    pub const fn const_is_zero(&self) -> bool {
        self.const_eq(&Self::ZERO)
    }

    /// Computes the bitwise AND of two `FixedBytes`.
    pub const fn bit_and(self, rhs: Self) -> Self {
        let mut ret = Self::ZERO;
        let mut i = 0;
        while i < N {
            ret.0[i] = self.0[i] & rhs.0[i];
            i += 1;
        }
        ret
    }

    /// Computes the bitwise XOR of two `FixedBytes`.
    pub const fn sized_bit_and<const M: usize, const Z: usize>(
        self,
        rhs: FixedBytes<M>,
    ) -> FixedBytes<Z> {
        assert!(M <= Z && N <= Z);

        let lsh = self.resize::<Z>();
        let rhs = rhs.resize::<Z>();
        lsh.bit_and(rhs)
    }

    /// Computes the bitwise OR of two `FixedBytes`.
    pub const fn bit_or(self, rhs: Self) -> Self {
        let mut ret = Self::ZERO;
        let mut i = 0;
        while i < N {
            ret.0[i] = self.0[i] | rhs.0[i];
            i += 1;
        }
        ret
    }

    /// Computes the bitwise XOR of two `FixedBytes`.
    pub const fn sized_bit_or<const M: usize, const Z: usize>(
        self,
        rhs: FixedBytes<M>,
    ) -> FixedBytes<Z> {
        assert!(M <= Z && N <= Z);

        let lsh = self.resize::<Z>();
        let rhs = rhs.resize::<Z>();
        lsh.bit_or(rhs)
    }

    /// Computes the bitwise XOR of two `FixedBytes`.
    pub const fn bit_xor(self, rhs: Self) -> Self {
        let mut ret = Self::ZERO;
        let mut i = 0;
        while i < N {
            ret.0[i] = self.0[i] ^ rhs.0[i];
            i += 1;
        }
        ret
    }

    /// Computes the bitwise XOR of two `FixedBytes`.
    pub const fn sized_bit_xor<const M: usize, const Z: usize>(
        self,
        rhs: FixedBytes<M>,
    ) -> FixedBytes<Z> {
        assert!(M <= Z && N <= Z);

        let lsh = self.resize::<Z>();
        let rhs = rhs.resize::<Z>();
        lsh.bit_xor(rhs)
    }

    pub const fn resize<const M: usize>(self) -> FixedBytes<M> {
        let mut result = [0u8; M];
        let mut i = 0;
        while i < N {
            result[i] = self.0[i];
            i += 1;
        }
        FixedBytes(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sized_xor() {
        // a and b are of the same length
        let a = FixedBytes::new([0xb5, 0x0, 0x0, 0x0]);
        let b = FixedBytes::new([0xff, 0xff, 0xff, 0xff]);

        let actual = a.sized_bit_xor(b);
        let expected = FixedBytes::new([0x4a, 0xff, 0xff, 0xff]);
        assert_eq!(actual, expected);

        // a.len == 4 and b.len == 10
        let b = FixedBytes::new([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0a]);

        let actual = a.sized_bit_xor(b);
        let expected =
            FixedBytes::new([0x4a, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0a]);
        assert_eq!(actual, expected);

        // a.len == 4 and b.len == 10 but resized to 12
        let actual = a.sized_bit_xor(b);
        let expected = FixedBytes::new([
            0x4a, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0a, 0x00, 0x00,
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sized_and() {
        let a = FixedBytes::new([0b1100, 0b1010]);
        let b = FixedBytes::new([0b1010, 0b0101]);

        let actual = a.sized_bit_and(b);
        let expected = FixedBytes::new([0b1000, 0b0000]);
        assert_eq!(actual, expected);

        let b = FixedBytes::new([0b1010, 0b0101, 0b1111]);

        let actual = a.sized_bit_and(b);
        let expected = FixedBytes::new([0b1000, 0b0000, 0b0000]);
        assert_eq!(actual, expected);

        let actual = a.sized_bit_and(b);
        let expected = FixedBytes::new([0b1000, 0b0000, 0b0000, 0b0000]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sized_or() {
        let a = FixedBytes::new([0b1100, 0b1010]);
        let b = FixedBytes::new([0b1010, 0b0101]);

        let actual = a.sized_bit_or(b);
        let expected = FixedBytes::new([0b1110, 0b1111]);
        assert_eq!(actual, expected);

        let b = FixedBytes::new([0b1010, 0b0101, 0b1111]);

        let actual = a.sized_bit_or(b);
        let expected = FixedBytes::new([0b1110, 0b1111, 0b1111]);
        assert_eq!(actual, expected);

        let actual = a.sized_bit_or(b);
        let expected = FixedBytes::new([0b1110, 0b1111, 0b1111, 0b0000]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn index_access() {
        let a = FixedBytes::new([0b1100, 0b1010]);

        assert_eq!(a[0], 0b1100);
        assert_eq!(a[1], 0b1010);
    }

    #[test]
    #[should_panic]
    fn index_access_out_of_bounds() {
        let a = FixedBytes::new([0b1100, 0b1010]);

        assert_eq!(a[2], 0b1010);
    }

    #[test]
    fn comparisons() {
        assert!(
            FixedBytes::new([0b01111111, 0b11111111]) < FixedBytes::new([0b10000000, 0b00000000])
        );
        assert!(
            FixedBytes::new([0b01111111, 0b11111111]) <= FixedBytes::new([0b10000000, 0b00000000])
        );
        assert!(
            FixedBytes::new([0b10000000, 0b00000000]) >= FixedBytes::new([0b01111111, 0b11111111])
        );
        assert!(
            FixedBytes::new([0b10000000, 0b00000000]) != FixedBytes::new([0b01111111, 0b11111111])
        );

        assert!(
            FixedBytes::new([0b10000000, 0b00000000]) == FixedBytes::new([0b10000000, 0b00000000])
        );
        assert!(
            FixedBytes::new([0b10000000, 0b00000000]) <= FixedBytes::new([0b10000000, 0b00000000])
        );
        assert!(
            FixedBytes::new([0b10000000, 0b00000000]) >= FixedBytes::new([0b10000000, 0b00000000])
        );
    }
}
