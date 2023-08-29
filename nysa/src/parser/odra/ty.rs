use syn::parse_quote;

use crate::{
    model::ir::{NysaExpression, NysaType},
    ParserError,
};

/// Parses solidity statement into a syn type.
///
/// Panics if the input is an expression of type other than [NysaExpression::Type].
pub fn parse_odra_ty(ty: &NysaType) -> Result<syn::Type, ParserError> {
    match ty {
        NysaType::Mapping(key, value) => {
            let key = parse_plain_type_from_expr(key)?;
            let value = parse_plain_type_from_expr(value)?;
            Ok(parse_quote!(odra::Mapping<#key, #value>))
        }
        NysaType::Address => Ok(parse_quote!(odra::Variable<Option<odra::types::Address>>)),
        NysaType::String => Ok(parse_quote!(odra::Variable<String>)),
        NysaType::Bool => Ok(parse_quote!(odra::Variable<bool>)),
        NysaType::Int(_) => Ok(parse_quote!(odra::Variable<i16>)),
        NysaType::Uint(size) => match size {
            8 => Ok(parse_quote!(odra::Variable<u8>)),
            16 => Ok(parse_quote!(odra::Variable<u16>)),
            32 => Ok(parse_quote!(odra::Variable<u32>)),
            64 => Ok(parse_quote!(odra::Variable<u64>)),
            128 => Ok(parse_quote!(odra::Variable<odra::types::U128>)),
            256 => Ok(parse_quote!(odra::Variable<odra::types::U256>)),
            512 => Ok(parse_quote!(odra::Variable<odra::types::U512>)),
            _ => Err(ParserError::UnsupportedType(ty.clone())),
        },
        _ => Err(ParserError::UnsupportedType(ty.clone())),
    }
}

pub fn parse_plain_type_from_expr(expr: &NysaExpression) -> Result<syn::Type, ParserError> {
    match expr {
        NysaExpression::Type { ty } => parse_plain_type_from_ty(ty),
        _ => Err(ParserError::UnexpectedExpression(
            String::from("NysaExpression::Type"),
            expr.clone(),
        )),
    }
}

pub fn parse_plain_type_from_ty(ty: &NysaType) -> Result<syn::Type, ParserError> {
    match ty {
        NysaType::Address => Ok(parse_quote!(Option<odra::types::Address>)),
        NysaType::String => Ok(parse_quote!(String)),
        NysaType::Bool => Ok(parse_quote!(bool)),
        NysaType::Int(_) => Ok(parse_quote!(i16)),
        NysaType::Uint(size) => match size {
            8 => Ok(parse_quote!(u8)),
            16 => Ok(parse_quote!(u16)),
            32 => Ok(parse_quote!(u32)),
            64 => Ok(parse_quote!(u64)),
            128 => Ok(parse_quote!(odra::types::U128)),
            256 => Ok(parse_quote!(odra::types::U256)),
            512 => Ok(parse_quote!(odra::types::U512)),
            _ => Err(ParserError::UnsupportedType(ty.clone())),
        },
        NysaType::Mapping(key, value) => {
            let key = parse_plain_type_from_expr(key)?;
            let value = parse_plain_type_from_expr(value)?;
            Ok(parse_quote!(odra::Mapping<#key, #value>))
        }
        NysaType::Bytes(_) => Err(ParserError::UnsupportedType(ty.clone())),
        NysaType::Custom(_) => Err(ParserError::UnsupportedType(ty.clone())),
        NysaType::Contract(_) => Ok(parse_quote!(Option<odra::types::Address>)),
    }
}
