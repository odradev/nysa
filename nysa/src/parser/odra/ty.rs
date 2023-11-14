use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse_quote;

use crate::{
    model::ir::{Expression, Type},
    utils, ParserError, parser::context::{TypeInfo, ItemType},
};

/// Parses solidity statement into a syn type.
///
/// Panics if the input is an expression of type other than [Expression::Type].
pub fn parse_state_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> Result<syn::Type, ParserError> {
    match ty {
        Type::Mapping(key, value) => {
            let key = parse_type_from_expr(key, ctx)?;
            let value = parse_type_from_expr(value, ctx)?;
            Ok(parse_quote!(odra::Mapping<#key, #value>))
        }
        Type::Address => Ok(parse_quote!(odra::Variable<Option<odra::types::Address>>)),
        Type::String => Ok(parse_quote!(odra::Variable<odra::prelude::string::String>)),
        Type::Bool => Ok(parse_quote!(odra::Variable<bool>)),
        Type::Int(size) => {
            let val = build_int(*size);
            Ok(parse_quote!(odra::Variable<#val>))
        },
        Type::Uint(size) => {
            let val = build_uint(*size);
            Ok(parse_quote!(odra::Variable<#val>))
        },
        Type::Custom(name) => ctx
            .type_from_string(name)
            .map(|ty| match ty {
                ItemType::Contract(_) | ItemType::Interface(_) => {
                    parse_quote!(odra::Variable<Option<odra::types::Address>>)
                }
                ItemType::Enum(_) | ItemType::Struct(_) => {
                    let ident = format_ident!("{}", name);
                    parse_quote!(odra::Variable<#ident>)
                }
                ItemType::Event => todo!(),
                ItemType::Storage(_) => todo!(),
                ItemType::Local(_) => todo!(),
                ItemType::Library(_) => todo!(),
            })
            .ok_or(ParserError::InvalidType),
        Type::Bytes(i) => {
            let size = *i as usize;
            Ok(parse_quote!(odra::Variable<nysa_types::FixedBytes<#size>>))
        }
        Type::Array(ty) => {
            let ty = parse_type_from_ty(ty, ctx)?;
            Ok(parse_quote!(odra::Variable<Vec<#ty>>))
        }
        Type::Unknown => Err(ParserError::InvalidType),
    }
}

pub fn parse_type_from_expr<T: TypeInfo>(
    expr: &Expression,
    ctx: &T,
) -> Result<syn::Type, ParserError> {
    let err = || ParserError::UnexpectedExpression(String::from("Expression::Type"), expr.clone());
    match expr {
        Expression::Type(ty) => parse_type_from_ty(ty, ctx),
        Expression::MemberAccess(f, box Expression::Variable(name)) => {
            let p = utils::to_snake_case_ident(name);
            let ident = format_ident!("{}", f);
            Ok(parse_quote!(#p::#ident))
        }
        Expression::Variable(name) => match ctx.type_from_string(name) {
            Some(ItemType::Enum(_) | ItemType::Struct(_)) => {
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
        Type::Int(size) => {
            let val = build_int(*size);
            Ok(parse_quote!(#val))
        },
        Type::Uint(size) => {
            let val = build_uint(*size);
            Ok(parse_quote!(#val))
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
                ItemType::Contract(_) | ItemType::Interface(_) => parse_quote!(Option<odra::types::Address>),
                ItemType::Enum(_) => {
                    let ident = format_ident!("{}", name);
                    parse_quote!(#ident)
                }
                ItemType::Struct(s) => {
                    let namespace = s
                        .namespace
                        .map(|ns| utils::to_snake_case_ident(ns))
                        .map(|i| quote!(#i::));
                    let ident = format_ident!("{}", name);
                    parse_quote!(#namespace #ident)
                }
                ItemType::Event => todo!(),
                ItemType::Storage(_) => todo!(),
                ItemType::Local(_) => todo!(),
                ItemType::Library(_) => todo!(),
            })
            .ok_or(ParserError::InvalidType),
        Type::Array(ty) => {
            let ty = parse_type_from_ty(ty, t)?;
            Ok(parse_quote!(odra::prelude::vec::Vec<#ty>))
        }
        Type::Unknown => Err(ParserError::InvalidType),
    }
}

fn build_int(size: u16) -> TokenStream {
    let s = format_ident!("I{}", size);
    quote::quote!(nysa_types::#s)
}

fn build_uint(size: u16) -> TokenStream {
    let s = format_ident!("U{}", size);
    quote::quote!(nysa_types::#s)
}
