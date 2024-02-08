use odra_core::casper_types::{
    bytesrepr::{Error, FromBytes, ToBytes},
    CLType, CLTyped,
};

use crate::{FixedBytes, Signed, Unsigned};

pub const U64_SERIALIZED_LENGTH: usize = core::mem::size_of::<u64>();
pub const U128_SERIALIZED_LENGTH: usize = U64_SERIALIZED_LENGTH * 2;
pub const U192_SERIALIZED_LENGTH: usize = U64_SERIALIZED_LENGTH * 3;
pub const U256_SERIALIZED_LENGTH: usize = U64_SERIALIZED_LENGTH * 4;

macro_rules! impl_int_deser {
    ( $( $ty:ident ),* ) => {
        $(impl<const BITS: usize, const LIMBS: usize> ToBytes for $ty<BITS, LIMBS> {
            fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, Error> {
                Ok(self
                    .as_limbs()
                    .iter()
                    .map(|word| word.to_le_bytes().to_vec())
                    .flatten()
                    .collect::<alloc::vec::Vec<_>>())
            }

            fn serialized_length(&self) -> usize {
                U64_SERIALIZED_LENGTH * LIMBS
            }
        }

        impl<const BITS: usize, const LIMBS: usize> FromBytes for $ty<BITS, LIMBS> {
            fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
                if U64_SERIALIZED_LENGTH * LIMBS > bytes.len() {
                    return Err(Error::Formatting);
                } else {
                    let (bytes, remainder) = bytes.split_at(U64_SERIALIZED_LENGTH * LIMBS);

                    match LIMBS {
                        1 => {
                            let bytes = <[u8; U64_SERIALIZED_LENGTH]>::try_from(bytes)
                                .map_err(|_| Error::EarlyEndOfStream)?;
                            try_from_bytes(bytes, remainder)
                                .map(|(uint, remainder)| ($ty(uint), remainder))
                        }
                        2 => {
                            let bytes = <[u8; U128_SERIALIZED_LENGTH]>::try_from(bytes)
                                .map_err(|_| Error::EarlyEndOfStream)?;
                            try_from_bytes(bytes, remainder)
                                .map(|(uint, remainder)| ($ty(uint), remainder))
                        }
                        3 => {
                            let bytes = <[u8; U192_SERIALIZED_LENGTH]>::try_from(bytes)
                                .map_err(|_| Error::EarlyEndOfStream)?;
                            try_from_bytes(bytes, remainder)
                                .map(|(uint, remainder)| ($ty(uint), remainder))
                        }
                        4 => {
                            let bytes = <[u8; U256_SERIALIZED_LENGTH]>::try_from(bytes)
                                .map_err(|_| Error::EarlyEndOfStream)?;
                            try_from_bytes(bytes, remainder)
                                .map(|(uint, remainder)| ($ty(uint), remainder))
                        }
                        _ => Err(Error::Formatting),
                    }
                }
            }
        }

        impl<const BITS: usize, const LIMBS: usize> CLTyped for $ty<BITS, LIMBS> {
            fn cl_type() -> CLType {
                <alloc::vec::Vec<u64>>::cl_type()
            }
        })*
    };
}

impl_int_deser!(Unsigned, Signed);

impl<const N: usize> ToBytes for FixedBytes<N> {
    fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, Error> {
        self.0.to_bytes()
    }

    fn serialized_length(&self) -> usize {
        self.0.serialized_length()
    }
}

impl<const N: usize> FromBytes for FixedBytes<N> {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        FromBytes::from_bytes(bytes).map(|(b, remainder)| (FixedBytes(b), remainder))
    }
}

impl<const N: usize> CLTyped for FixedBytes<N> {
    fn cl_type() -> CLType {
        CLType::ByteArray(N as u32)
    }
}

#[inline]
fn try_from_bytes<const BITS: usize, const LIMBS: usize, const LEN: usize>(
    bytes: [u8; LEN],
    remainder: &[u8],
) -> Result<(ruint::Uint<BITS, LIMBS>, &[u8]), Error> {
    let value = ruint::Uint::try_from_le_slice(&bytes).ok_or(Error::Formatting)?;
    Ok((value, remainder))
}

#[cfg(test)]
mod t {
    use crate::{I32, U8};
    use odra_core::casper_types::bytesrepr::{FromBytes, ToBytes};

    #[test]
    fn ser_de() {
        let value = U8::from_limbs([7u64]);
        let bytes = value.to_bytes().unwrap();
        let deserialized = U8::from_bytes(&bytes).unwrap().0;

        assert_eq!(value, deserialized);

        let value = I32::from_limbs([(u32::MAX - 7) as u64]);
        let bytes = value.to_bytes().unwrap();
        let deserialized = I32::from_bytes(&bytes).unwrap().0;

        assert_eq!(value, deserialized);
    }
}
