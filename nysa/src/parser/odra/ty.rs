use syn::parse_quote;

use crate::model::ir::{NysaExpression, NysaType};

/// Parses solidity statement into a syn type.
///
/// Panics if the input is an expression of type other than [NysaExpression::Type].
pub fn parse_odra_ty(ty: &NysaType) -> syn::Type {
    match ty {
        NysaType::Mapping(key, value) => {
            let key = parse_plain_type_from_expr(key);
            let value = parse_plain_type_from_expr(value);
            parse_quote!(odra::Mapping<#key, #value>)
        }
        NysaType::Address => parse_quote!(odra::Variable<Option<odra::types::Address>>),
        NysaType::String => parse_quote!(odra::Variable<String>),
        NysaType::Bool => parse_quote!(odra::Variable<bool>),
        NysaType::Int(_) => parse_quote!(odra::Variable<i16>),
        NysaType::Uint(size) => match size {
            8 => parse_quote!(odra::Variable<u8>),
            16 => parse_quote!(odra::Variable<u16>),
            32 => parse_quote!(odra::Variable<u32>),
            64 => parse_quote!(odra::Variable<u64>),
            128 => parse_quote!(odra::Variable<odra::types::U128>),
            256 => parse_quote!(odra::Variable<odra::types::U256>),
            512 => parse_quote!(odra::Variable<odra::types::U512>),
            _ => panic!("Unsupported unit {}.", size),
        },
        _ => panic!("Unsupported type."),
    }
}

pub fn parse_plain_type_from_expr(expr: &NysaExpression) -> syn::Type {
    match expr {
        NysaExpression::Type { ty } => parse_plain_type_from_ty(ty),
        _ => panic!("Not a type. {:?}", expr),
    }
}

pub fn parse_plain_type_from_ty(ty: &NysaType) -> syn::Type {
    match ty {
        NysaType::Address => parse_quote!(Option<odra::types::Address>),
        NysaType::String => parse_quote!(String),
        NysaType::Bool => parse_quote!(bool),
        NysaType::Int(_) => parse_quote!(i16),
        NysaType::Uint(size) => match size {
            8 => parse_quote!(u8),
            16 => parse_quote!(u16),
            32 => parse_quote!(u32),
            64 => parse_quote!(u64),
            128 => parse_quote!(odra::types::U128),
            256 => parse_quote!(odra::types::U256),
            512 => parse_quote!(odra::types::U512),
            _ => panic!("Unsupported unit {}.", size),
        },
        NysaType::Mapping(key, value) => {
            let key = parse_plain_type_from_expr(key);
            let value = parse_plain_type_from_expr(value);
            parse_quote!(odra::Mapping<#key, #value>)
        }
        NysaType::Bytes(_) => todo!(),
        NysaType::Custom(_) => todo!(),
        NysaType::Contract(_) => parse_quote!(Option<odra::types::Address>),
    }
}
