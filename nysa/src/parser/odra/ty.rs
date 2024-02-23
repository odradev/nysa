use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse_quote;

use crate::{
    error::ParserResult,
    model::ir::{Expression, Type},
    parser::{
        common::{ty::parse_type_from_expr, TypeParser},
        context::{ItemType, TypeInfo},
        syn_utils::AsType,
    },
    utils, OdraParser, ParserError,
};

use super::{expr::syn_utils, syn_utils::ty::*};

impl TypeParser for OdraParser {
    fn parse_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> ParserResult<syn::Type> {
        super::ty::parse_type_from_ty(ty, ctx)
    }

    fn parse_fixed_bytes(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        Ok(syn_utils::try_fixed_bytes(&args))
    }

    fn parse_serialize(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        Ok(syn_utils::serialize(&args))
    }

    fn parse_state_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> ParserResult<syn::Type> {
        super::ty::parse_state_ty(ty, ctx)
    }
}

/// Parses solidity statement into a syn type.
///
/// Panics if the input is an expression of type other than [Expression::Type].
pub fn parse_state_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> ParserResult<syn::Type> {
    match ty {
        Type::Mapping(key, value) => {
            let key = parse_type_from_expr::<_, OdraParser>(key, ctx)?;
            let (key, value) = compose_key(vec![key], value, ctx)?;
            let key = match key.len() {
                1 => key[0].clone(),
                _ => parse_quote!((#(#key,)*)),
            };

            Ok(map(key, value))
        }
        Type::Address => Ok(var(option(address()))),
        Type::String => Ok(var(string())),
        Type::Bool => Ok(var(bool())),
        Type::Int(size) => Ok(var(build_int(*size))),
        Type::Uint(size) => Ok(var(build_uint(*size))),
        Type::Custom(name) => ctx
            .type_from_string(name)
            .map(|ty| match ty {
                ItemType::Contract(_) | ItemType::Interface(_) => var(option(address())),
                ItemType::Enum(_) | ItemType::Struct(_) => var(utils::to_ident(name)),
                ItemType::Event => todo!(),
                ItemType::Storage(_) => todo!(),
                ItemType::Local(_) => todo!(),
                ItemType::Library(_) => todo!(),
            })
            .ok_or(ParserError::InvalidType),
        Type::Bytes(i) => Ok(var(fixed_bytes(*i as usize))),
        Type::Array(ty) => {
            let ty = parse_type_from_ty(ty, ctx)?;
            Ok(var(vec(ty)))
        }
        Type::Unknown => Err(ParserError::InvalidType),
    }
}

pub fn parse_type_from_ty<T: TypeInfo>(ty: &Type, t: &T) -> ParserResult<syn::Type> {
    match ty {
        Type::Address => Ok(option(address())),
        Type::String => Ok(string()),
        Type::Bool => Ok(bool()),
        Type::Int(size) => Ok(build_int(*size).as_type()),
        Type::Uint(size) => Ok(build_uint(*size).as_type()),
        Type::Mapping(key, value) => {
            let key = parse_type_from_expr::<_, OdraParser>(key, t)?;
            let value = parse_type_from_expr::<_, OdraParser>(value, t)?;
            Ok(map(key, value))
        }
        Type::Bytes(len) => Ok(fixed_bytes(*len as usize)),
        Type::Custom(name) => t
            .type_from_string(name)
            .map(|ty| match ty {
                ItemType::Contract(_) | ItemType::Interface(_) => option(address()),
                ItemType::Enum(_) => utils::to_ident(name).as_type(),
                ItemType::Struct(s) => {
                    let namespace = s
                        .namespace
                        .map(utils::to_snake_case_ident)
                        .map(|i| quote!(#i::));
                    let ident = utils::to_ident(name).as_type();
                    parse_quote!(#namespace #ident)
                }
                ItemType::Event => todo!(),
                ItemType::Storage(_) => todo!(),
                ItemType::Local(_) => todo!(),
                ItemType::Library(_) => todo!(),
            })
            .ok_or(ParserError::InvalidType),
        Type::Array(ty) => Ok(vec(parse_type_from_ty(ty, t)?)),
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

fn compose_key<T: TypeInfo>(
    parts: Vec<syn::Type>,
    value: &Expression,
    ctx: &T,
) -> ParserResult<(Vec<syn::Type>, syn::Type)> {
    if let Expression::Type(Type::Mapping(key, value)) = value {
        let key = parse_type_from_expr::<_, OdraParser>(key, ctx)?;
        compose_key(parts.into_iter().chain(vec![key]).collect(), value, ctx)
    } else {
        Ok((
            parts.to_vec(),
            parse_type_from_expr::<_, OdraParser>(value, ctx)?,
        ))
    }
}
