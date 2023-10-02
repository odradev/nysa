use crate::{
    U104, U112, U120, U128, U136, U144, U152, U16, U160, U168, U176, U184, U192, U200, U208, U216,
    U224, U232, U24, U240, U248, U256, U32, U40, U48, U56, U64, U72, U8, U80, U88, U96,
};

pub const U64_SERIALIZED_LENGTH: usize = core::mem::size_of::<u64>();
pub const U128_SERIALIZED_LENGTH: usize = U64_SERIALIZED_LENGTH * 2;
pub const U192_SERIALIZED_LENGTH: usize = U64_SERIALIZED_LENGTH * 3;
pub const U256_SERIALIZED_LENGTH: usize = U64_SERIALIZED_LENGTH * 4;

macro_rules! impl_serialization {
    ( $( ($name:ident, $BITS:literal, $LIMBS:literal, $LEN:ident) ),* ) => {
        $(
            impl odra_types::casper_types::bytesrepr::ToBytes for $name {
                fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, odra_types::casper_types::bytesrepr::Error> {
                    Ok(self.as_limbs().iter().map(|word| word.to_le_bytes().to_vec()).flatten().collect::<alloc::vec::Vec<_>>())
                }

                fn serialized_length(&self) -> usize {
                    $LEN
                }
            }

            impl odra_types::casper_types::bytesrepr::FromBytes for $name {
                fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), odra_types::casper_types::bytesrepr::Error> {
                    if $LEN > bytes.len() {
                        return Err(odra_types::casper_types::bytesrepr::Error::Formatting);
                    } else {
                        let (bytes, remainder) = bytes.split_at($LEN);
                        let bytes = <[u8; $LEN]>::try_from(bytes).map_err(|_| odra_types::casper_types::bytesrepr::Error::EarlyEndOfStream)?;
                        let value = ruint::Uint::try_from_le_slice(&bytes)
                            .ok_or(odra_types::casper_types::bytesrepr::Error::Formatting)?;
                        Ok(($name(value), remainder))
                    }
                }
            }

            impl odra_types::casper_types::CLTyped for $name {
                fn cl_type() -> odra_types::casper_types::CLType {
                    <alloc::vec::Vec<u64>>::cl_type()
                }
            }

            impl odra_types::OdraItem for $name {
                fn is_module() -> bool {
                    false
                }
            }
        )*
    };
}

impl_serialization!(
    (U8, 8, 1, U64_SERIALIZED_LENGTH),
    (U16, 16, 1, U64_SERIALIZED_LENGTH),
    (U24, 24, 1, U64_SERIALIZED_LENGTH),
    (U32, 32, 1, U64_SERIALIZED_LENGTH),
    (U40, 40, 1, U64_SERIALIZED_LENGTH),
    (U48, 48, 1, U64_SERIALIZED_LENGTH),
    (U56, 56, 1, U64_SERIALIZED_LENGTH),
    (U64, 64, 1, U64_SERIALIZED_LENGTH),
    (U72, 72, 2, U128_SERIALIZED_LENGTH),
    (U80, 80, 2, U128_SERIALIZED_LENGTH),
    (U88, 88, 2, U128_SERIALIZED_LENGTH),
    (U96, 96, 2, U128_SERIALIZED_LENGTH),
    (U104, 104, 2, U128_SERIALIZED_LENGTH),
    (U112, 112, 2, U128_SERIALIZED_LENGTH),
    (U120, 120, 2, U128_SERIALIZED_LENGTH),
    (U128, 128, 2, U128_SERIALIZED_LENGTH),
    (U136, 136, 3, U192_SERIALIZED_LENGTH),
    (U144, 144, 3, U192_SERIALIZED_LENGTH),
    (U152, 152, 3, U192_SERIALIZED_LENGTH),
    (U160, 160, 3, U192_SERIALIZED_LENGTH),
    (U168, 168, 3, U192_SERIALIZED_LENGTH),
    (U176, 176, 3, U192_SERIALIZED_LENGTH),
    (U184, 184, 3, U192_SERIALIZED_LENGTH),
    (U192, 192, 3, U192_SERIALIZED_LENGTH),
    (U200, 200, 4, U256_SERIALIZED_LENGTH),
    (U208, 208, 4, U256_SERIALIZED_LENGTH),
    (U216, 216, 4, U256_SERIALIZED_LENGTH),
    (U224, 224, 4, U256_SERIALIZED_LENGTH),
    (U232, 232, 4, U256_SERIALIZED_LENGTH),
    (U240, 240, 4, U256_SERIALIZED_LENGTH),
    (U248, 248, 4, U256_SERIALIZED_LENGTH),
    (U256, 256, 4, U256_SERIALIZED_LENGTH)
);

#[cfg(test)]
mod t {

    use crate::U8;
    use odra_types::casper_types::bytesrepr::{FromBytes, ToBytes};

    #[test]
    fn ser_de() {
        let value = U8::from(44);
        let bytes = value.to_bytes().unwrap();
        assert_eq!(8, bytes.len());
        assert_eq!(value, U8::from_bytes(&bytes).unwrap().0);
    }
}
