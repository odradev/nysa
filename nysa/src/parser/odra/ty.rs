use quote::format_ident;
use syn::parse_quote;

use crate::{
    model::ir::{Expression, Type},
    parser::context::{self, TypeInfo},
    ParserError,
};

/// Parses solidity statement into a syn type.
///
/// Panics if the input is an expression of type other than [Expression::Type].
pub fn parse_odra_ty<T: TypeInfo>(ty: &Type, t: &T) -> Result<syn::Type, ParserError> {
    match ty {
        Type::Mapping(key, value) => {
            let key = parse_plain_type_from_expr(key, t)?;
            let value = parse_plain_type_from_expr(value, t)?;
            Ok(parse_quote!(odra::Mapping<#key, #value>))
        }
        Type::Address => Ok(parse_quote!(odra::Variable<Option<odra::types::Address>>)),
        Type::String => Ok(parse_quote!(odra::Variable<odra::prelude::string::String>)),
        Type::Bool => Ok(parse_quote!(odra::Variable<bool>)),
        Type::Int(_) => Ok(parse_quote!(odra::Variable<i16>)),
        Type::Uint(size) => match size {
            0..=8 => Ok(parse_quote!(odra::Variable<u8>)),
            9..=16 => Ok(parse_quote!(odra::Variable<u16>)),
            17..=32 => Ok(parse_quote!(odra::Variable<u32>)),
            33..=64 => Ok(parse_quote!(odra::Variable<u64>)),
            65..=128 => Ok(parse_quote!(odra::Variable<odra::types::U128>)),
            129..=256 => Ok(parse_quote!(odra::Variable<odra::types::U256>)),
            257..=512 => Ok(parse_quote!(odra::Variable<odra::types::U512>)),
            _ => Err(ParserError::UnsupportedType(ty.clone())),
        },
        Type::Custom(name) => t
            .type_from_string(name)
            .map(|ty| match ty {
                context::ItemType::Contract(_) => {
                    parse_quote!(odra::Variable<Option<odra::types::Address>>)
                }
                context::ItemType::Interface(_) => {
                    parse_quote!(odra::Variable<Option<odra::types::Address>>)
                }
                context::ItemType::Enum(_) => {
                    let ident = format_ident!("{}", name);
                    parse_quote!(odra::Variable<#ident>)
                }
                context::ItemType::Event => todo!(),
                context::ItemType::Storage(_) => todo!(),
                context::ItemType::Local(_) => todo!(),
            })
            .ok_or(ParserError::InvalidType),
        Type::Bytes(i) => {
            let size = *i as usize;
            Ok(parse_quote!(odra::Variable<[u8; #size]>))
        }
        Type::Array(ty) => {
            let ty = parse_plain_type_from_ty(ty, t)?;
            Ok(parse_quote!(odra::Variable<Vec<#ty>>))
        }
        Type::Unknown => Err(ParserError::InvalidType),
    }
}

pub fn parse_plain_type_from_expr<T: TypeInfo>(
    expr: &Expression,
    t: &T,
) -> Result<syn::Type, ParserError> {
    let err = || ParserError::UnexpectedExpression(String::from("Expression::Type"), expr.clone());

    match expr {
        Expression::Type(ty) => parse_plain_type_from_ty(ty, t),
        Expression::Variable(name) => match t.type_from_string(name) {
            Some(context::ItemType::Enum(_)) => {
                let ident = format_ident!("{}", name);
                Ok(parse_quote!(#ident))
            }
            _ => Err(err()),
        },
        _ => Err(err()),
    }
}

pub fn parse_plain_type_from_ty<T: TypeInfo>(ty: &Type, t: &T) -> Result<syn::Type, ParserError> {
    match ty {
        Type::Address => Ok(parse_quote!(Option<odra::types::Address>)),
        Type::String => Ok(parse_quote!(odra::prelude::string::String)),
        Type::Bool => Ok(parse_quote!(bool)),
        Type::Int(_) => Ok(parse_quote!(i16)),
        Type::Uint(size) => match size {
            0..=8 => Ok(parse_quote!(u8)),
            9..=16 => Ok(parse_quote!(u16)),
            17..=32 => Ok(parse_quote!(u32)),
            33..=64 => Ok(parse_quote!(u64)),
            65..=128 => Ok(parse_quote!(odra::types::U128)),
            129..=256 => Ok(parse_quote!(odra::types::U256)),
            257..=512 => Ok(parse_quote!(odra::types::U512)),
            _ => Err(ParserError::UnsupportedType(ty.clone())),
        },
        Type::Mapping(key, value) => {
            let key = parse_plain_type_from_expr(key, t)?;
            let value = parse_plain_type_from_expr(value, t)?;
            Ok(parse_quote!(odra::Mapping<#key, #value>))
        }
        Type::Bytes(_) => Err(ParserError::UnsupportedType(ty.clone())),
        Type::Custom(name) => t
            .type_from_string(name)
            .map(|ty| match ty {
                context::ItemType::Contract(_) => parse_quote!(Option<odra::types::Address>),
                context::ItemType::Interface(_) => parse_quote!(Option<odra::types::Address>),
                context::ItemType::Enum(_) => {
                    let ident = format_ident!("{}", name);
                    parse_quote!(#ident)
                }
                context::ItemType::Event => todo!(),
                context::ItemType::Storage(_) => todo!(),
                context::ItemType::Local(_) => todo!(),
            })
            .ok_or(ParserError::InvalidType),
        Type::Array(ty) => {
            let ty = parse_plain_type_from_ty(ty, t)?;
            Ok(parse_quote!(odra::prelude::vec::Vec<#ty>))
        }
        Type::Unknown => Err(ParserError::InvalidType),
    }
}
