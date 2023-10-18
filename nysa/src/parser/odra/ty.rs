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
pub fn parse_state_ty<T: TypeInfo>(ty: &Type, t: &T) -> Result<syn::Type, ParserError> {
    match ty {
        Type::Mapping(key, value) => {
            let key = parse_type_from_expr(key, t)?;
            let value = parse_type_from_expr(value, t)?;
            Ok(parse_quote!(odra::Mapping<#key, #value>))
        }
        Type::Address => Ok(parse_quote!(odra::Variable<Option<odra::types::Address>>)),
        Type::String => Ok(parse_quote!(odra::Variable<odra::prelude::string::String>)),
        Type::Bool => Ok(parse_quote!(odra::Variable<bool>)),
        Type::Int(size) => match size {
            8 => Ok(parse_quote!(odra::Variable<nysa_types::I8>)),
            16 => Ok(parse_quote!(odra::Variable<nysa_types::I16>)),
            24 => Ok(parse_quote!(odra::Variable<nysa_types::I24>)),
            32 => Ok(parse_quote!(odra::Variable<nysa_types::I32>)),
            40 => Ok(parse_quote!(odra::Variable<nysa_types::I40>)),
            48 => Ok(parse_quote!(odra::Variable<nysa_types::I48>)),
            56 => Ok(parse_quote!(odra::Variable<nysa_types::I56>)),
            64 => Ok(parse_quote!(odra::Variable<nysa_types::I64>)),
            72 => Ok(parse_quote!(odra::Variable<nysa_types::I72>)),
            80 => Ok(parse_quote!(odra::Variable<nysa_types::I80>)),
            88 => Ok(parse_quote!(odra::Variable<nysa_types::I88>)),
            96 => Ok(parse_quote!(odra::Variable<nysa_types::I96>)),
            104 => Ok(parse_quote!(odra::Variable<nysa_types::I104>)),
            112 => Ok(parse_quote!(odra::Variable<nysa_types::I112>)),
            120 => Ok(parse_quote!(odra::Variable<nysa_types::I120>)),
            128 => Ok(parse_quote!(odra::Variable<nysa_types::I128>)),
            136 => Ok(parse_quote!(odra::Variable<nysa_types::I136>)),
            144 => Ok(parse_quote!(odra::Variable<nysa_types::I144>)),
            152 => Ok(parse_quote!(odra::Variable<nysa_types::I152>)),
            160 => Ok(parse_quote!(odra::Variable<nysa_types::I160>)),
            168 => Ok(parse_quote!(odra::Variable<nysa_types::I168>)),
            176 => Ok(parse_quote!(odra::Variable<nysa_types::I176>)),
            184 => Ok(parse_quote!(odra::Variable<nysa_types::I184>)),
            192 => Ok(parse_quote!(odra::Variable<nysa_types::I192>)),
            200 => Ok(parse_quote!(odra::Variable<nysa_types::I200>)),
            208 => Ok(parse_quote!(odra::Variable<nysa_types::I208>)),
            216 => Ok(parse_quote!(odra::Variable<nysa_types::I216>)),
            224 => Ok(parse_quote!(odra::Variable<nysa_types::I224>)),
            232 => Ok(parse_quote!(odra::Variable<nysa_types::I232>)),
            240 => Ok(parse_quote!(odra::Variable<nysa_types::I240>)),
            248 => Ok(parse_quote!(odra::Variable<nysa_types::I248>)),
            256 => Ok(parse_quote!(odra::Variable<nysa_types::I256>)),
            _ => Err(ParserError::UnsupportedType(ty.clone())),
        },
        Type::Uint(size) => match size {
            8 => Ok(parse_quote!(odra::Variable<nysa_types::U8>)),
            16 => Ok(parse_quote!(odra::Variable<nysa_types::U16>)),
            24 => Ok(parse_quote!(odra::Variable<nysa_types::U24>)),
            32 => Ok(parse_quote!(odra::Variable<nysa_types::U32>)),
            40 => Ok(parse_quote!(odra::Variable<nysa_types::U40>)),
            48 => Ok(parse_quote!(odra::Variable<nysa_types::U48>)),
            56 => Ok(parse_quote!(odra::Variable<nysa_types::U56>)),
            64 => Ok(parse_quote!(odra::Variable<nysa_types::U64>)),
            72 => Ok(parse_quote!(odra::Variable<nysa_types::U72>)),
            80 => Ok(parse_quote!(odra::Variable<nysa_types::U80>)),
            88 => Ok(parse_quote!(odra::Variable<nysa_types::U88>)),
            96 => Ok(parse_quote!(odra::Variable<nysa_types::U96>)),
            104 => Ok(parse_quote!(odra::Variable<nysa_types::U104>)),
            112 => Ok(parse_quote!(odra::Variable<nysa_types::U112>)),
            120 => Ok(parse_quote!(odra::Variable<nysa_types::U120>)),
            128 => Ok(parse_quote!(odra::Variable<nysa_types::U128>)),
            136 => Ok(parse_quote!(odra::Variable<nysa_types::U136>)),
            144 => Ok(parse_quote!(odra::Variable<nysa_types::U144>)),
            152 => Ok(parse_quote!(odra::Variable<nysa_types::U152>)),
            160 => Ok(parse_quote!(odra::Variable<nysa_types::U160>)),
            168 => Ok(parse_quote!(odra::Variable<nysa_types::U168>)),
            176 => Ok(parse_quote!(odra::Variable<nysa_types::U176>)),
            184 => Ok(parse_quote!(odra::Variable<nysa_types::U184>)),
            192 => Ok(parse_quote!(odra::Variable<nysa_types::U192>)),
            200 => Ok(parse_quote!(odra::Variable<nysa_types::U200>)),
            208 => Ok(parse_quote!(odra::Variable<nysa_types::U208>)),
            216 => Ok(parse_quote!(odra::Variable<nysa_types::U216>)),
            224 => Ok(parse_quote!(odra::Variable<nysa_types::U224>)),
            232 => Ok(parse_quote!(odra::Variable<nysa_types::U232>)),
            240 => Ok(parse_quote!(odra::Variable<nysa_types::U240>)),
            248 => Ok(parse_quote!(odra::Variable<nysa_types::U248>)),
            256 => Ok(parse_quote!(odra::Variable<nysa_types::U256>)),
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
                context::ItemType::Struct(_) => {
                    let ident = format_ident!("{}", name);
                    parse_quote!(odra::Variable<#ident>)
                }
            })
            .ok_or(ParserError::InvalidType),
        Type::Bytes(i) => {
            let size = *i as usize;
            Ok(parse_quote!(odra::Variable<nysa_types::FixedBytes<#size>>))
        }
        Type::Array(ty) => {
            let ty = parse_type_from_ty(ty, t)?;
            Ok(parse_quote!(odra::Variable<Vec<#ty>>))
        }
        Type::Unknown => Err(ParserError::InvalidType),
    }
}

pub fn parse_type_from_expr<T: TypeInfo>(
    expr: &Expression,
    t: &T,
) -> Result<syn::Type, ParserError> {
    let err = || ParserError::UnexpectedExpression(String::from("Expression::Type"), expr.clone());
    match expr {
        Expression::Type(ty) => parse_type_from_ty(ty, t),
        Expression::Variable(name) => match t.type_from_string(name) {
            Some(context::ItemType::Enum(_) | context::ItemType::Struct(_)) => {
                let ident = format_ident!("{}", name);
                Ok(parse_quote!(#ident))
            }
            _ => Err(err()),
        },
        _ => Err(err()),
    }
}

pub fn parse_type_from_ty<T: TypeInfo>(ty: &Type, t: &T) -> Result<syn::Type, ParserError> {
    match ty {
        Type::Address => Ok(parse_quote!(Option<odra::types::Address>)),
        Type::String => Ok(parse_quote!(odra::prelude::string::String)),
        Type::Bool => Ok(parse_quote!(bool)),
        Type::Int(size) => match size {
            8 => Ok(parse_quote!(nysa_types::I8)),
            16 => Ok(parse_quote!(nysa_types::I16)),
            24 => Ok(parse_quote!(nysa_types::I24)),
            32 => Ok(parse_quote!(nysa_types::I32)),
            40 => Ok(parse_quote!(nysa_types::I40)),
            48 => Ok(parse_quote!(nysa_types::I48)),
            56 => Ok(parse_quote!(nysa_types::I56)),
            64 => Ok(parse_quote!(nysa_types::I64)),
            72 => Ok(parse_quote!(nysa_types::I72)),
            80 => Ok(parse_quote!(nysa_types::I80)),
            88 => Ok(parse_quote!(nysa_types::I88)),
            96 => Ok(parse_quote!(nysa_types::I96)),
            104 => Ok(parse_quote!(nysa_types::I104)),
            112 => Ok(parse_quote!(nysa_types::I112)),
            120 => Ok(parse_quote!(nysa_types::I120)),
            128 => Ok(parse_quote!(nysa_types::I128)),
            136 => Ok(parse_quote!(nysa_types::I136)),
            144 => Ok(parse_quote!(nysa_types::I144)),
            152 => Ok(parse_quote!(nysa_types::I152)),
            160 => Ok(parse_quote!(nysa_types::I160)),
            168 => Ok(parse_quote!(nysa_types::I168)),
            176 => Ok(parse_quote!(nysa_types::I176)),
            184 => Ok(parse_quote!(nysa_types::I184)),
            192 => Ok(parse_quote!(nysa_types::I192)),
            200 => Ok(parse_quote!(nysa_types::I200)),
            208 => Ok(parse_quote!(nysa_types::I208)),
            216 => Ok(parse_quote!(nysa_types::I216)),
            224 => Ok(parse_quote!(nysa_types::I224)),
            232 => Ok(parse_quote!(nysa_types::I232)),
            240 => Ok(parse_quote!(nysa_types::I240)),
            248 => Ok(parse_quote!(nysa_types::I248)),
            256 => Ok(parse_quote!(nysa_types::I256)),
            _ => Err(ParserError::UnsupportedType(ty.clone())),
        },
        Type::Uint(size) => match size {
            8 => Ok(parse_quote!(nysa_types::U8)),
            16 => Ok(parse_quote!(nysa_types::U16)),
            24 => Ok(parse_quote!(nysa_types::U24)),
            32 => Ok(parse_quote!(nysa_types::U32)),
            40 => Ok(parse_quote!(nysa_types::U40)),
            48 => Ok(parse_quote!(nysa_types::U48)),
            56 => Ok(parse_quote!(nysa_types::U56)),
            64 => Ok(parse_quote!(nysa_types::U64)),
            72 => Ok(parse_quote!(nysa_types::U72)),
            80 => Ok(parse_quote!(nysa_types::U80)),
            88 => Ok(parse_quote!(nysa_types::U88)),
            96 => Ok(parse_quote!(nysa_types::U96)),
            104 => Ok(parse_quote!(nysa_types::U104)),
            112 => Ok(parse_quote!(nysa_types::U112)),
            120 => Ok(parse_quote!(nysa_types::U120)),
            128 => Ok(parse_quote!(nysa_types::U128)),
            136 => Ok(parse_quote!(nysa_types::U136)),
            144 => Ok(parse_quote!(nysa_types::U144)),
            152 => Ok(parse_quote!(nysa_types::U152)),
            160 => Ok(parse_quote!(nysa_types::U160)),
            168 => Ok(parse_quote!(nysa_types::U168)),
            176 => Ok(parse_quote!(nysa_types::U176)),
            184 => Ok(parse_quote!(nysa_types::U184)),
            192 => Ok(parse_quote!(nysa_types::U192)),
            200 => Ok(parse_quote!(nysa_types::U200)),
            208 => Ok(parse_quote!(nysa_types::U208)),
            216 => Ok(parse_quote!(nysa_types::U216)),
            224 => Ok(parse_quote!(nysa_types::U224)),
            232 => Ok(parse_quote!(nysa_types::U232)),
            240 => Ok(parse_quote!(nysa_types::U240)),
            248 => Ok(parse_quote!(nysa_types::U248)),
            256 => Ok(parse_quote!(nysa_types::U256)),
            _ => Err(ParserError::UnsupportedType(ty.clone())),
        },
        Type::Mapping(key, value) => {
            let key = parse_type_from_expr(key, t)?;
            let value = parse_type_from_expr(value, t)?;
            Ok(parse_quote!(odra::Mapping<#key, #value>))
        }
        Type::Bytes(len) => {
            let size = *len as usize;
            Ok(parse_quote!(nysa_types::FixedBytes<#size>))
        }
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
                context::ItemType::Struct(_) => {
                    let ident = format_ident!("{}", name);
                    parse_quote!(#ident)
                }
            })
            .ok_or(ParserError::InvalidType),
        Type::Array(ty) => {
            let ty = parse_type_from_ty(ty, t)?;
            Ok(parse_quote!(odra::prelude::vec::Vec<#ty>))
        }
        Type::Unknown => Err(ParserError::InvalidType),
    }
}
