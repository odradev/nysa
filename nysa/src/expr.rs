use core::panic;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::BinOp;
use syn::Token;

use crate::model::NysaExpression;
use crate::model::StorageField;
use crate::ty;
use crate::utils;
use crate::{utils::to_snake_case_ident, var::IsField};

pub mod error;
pub mod primitives;

pub fn parse(
    expression: &NysaExpression,
    storage_fields: &[StorageField],
) -> Result<syn::Expr, &'static str> {
    match expression {
        NysaExpression::Require { condition, error } => {
            error::revert(Some(condition), error, storage_fields)
        }
        NysaExpression::Wildcard => Err("Empty identifier"),
        NysaExpression::ZeroAddress => Ok(parse_quote!(None)),
        NysaExpression::Message(msg) => msg.try_into(),
        NysaExpression::ArraySubscript { array, key } => {
            primitives::parse_mapping(array, key, None, storage_fields)
        }
        NysaExpression::Variable { name } => {
            let ident = to_snake_case_ident(&name);
            let self_ty = name.is_field(storage_fields).then(|| quote!(self.));
            Ok(parse_quote!(#self_ty #ident))
        }
        NysaExpression::Assign { left, right } => {
            primitives::assign(left, right, None, storage_fields)
        }
        NysaExpression::StringLiteral(string) => Ok(parse_quote!(#string)),
        NysaExpression::LessEqual { left, right } => {
            bin_op(left, right, parse_quote!(<=), storage_fields)
        }
        NysaExpression::MoreEqual { left, right } => {
            bin_op(left, right, parse_quote!(>=), storage_fields)
        }
        NysaExpression::Less { left, right } => {
            bin_op(left, right, parse_quote!(<), storage_fields)
        }
        NysaExpression::More { left, right } => {
            bin_op(left, right, parse_quote!(>), storage_fields)
        }
        NysaExpression::Add { left, right } => bin_op(left, right, parse_quote!(+), storage_fields),
        NysaExpression::Subtract { left, right } => {
            bin_op(left, right, parse_quote!(-), storage_fields)
        }
        NysaExpression::Equal { left, right } => {
            bin_op(left, right, parse_quote!(==), storage_fields)
        }
        NysaExpression::NotEqual { left, right } => {
            bin_op(left, right, parse_quote!(!=), storage_fields)
        }
        NysaExpression::AssignSubtract { left, right } => {
            let expr = primitives::assign(left, right, Some(parse_quote!(-)), storage_fields)?;
            Ok(expr)
        }
        NysaExpression::AssignAdd { left, right } => {
            let expr = primitives::assign(left, right, Some(parse_quote!(+)), storage_fields)?;
            Ok(expr)
        }
        NysaExpression::Increment { expr } => {
            let expr = parse(expr, storage_fields)?;
            Ok(parse_quote!(#expr += 1))
        }
        NysaExpression::Decrement { expr } => {
            let expr = parse(expr, storage_fields)?;
            Ok(parse_quote!(#expr -= 1))
        }
        NysaExpression::MemberAccess { expr, name } => {
            let base_expr: syn::Expr = parse(expr, storage_fields)?;
            let member: syn::Member = format_ident!("{}", name).into();
            Ok(parse_quote!(#base_expr.#member))
        }
        NysaExpression::NumberLiteral { ty, value } => match *ty {
            "u32" => {
                let arr = utils::convert_to_array(value);
                let num = u32::from_le_bytes(arr);
                Ok(parse_quote!(#num))
            }
            "u64" => {
                let arr = utils::convert_to_array(value);
                let num = u64::from_le_bytes(arr);
                Ok(parse_quote!(#num))
            }
            "U256" => {
                let arr = value
                    .iter()
                    .map(|v| quote!(#v))
                    .collect::<Punctuated<TokenStream, Token![,]>>();
                Ok(parse_quote!(odra::types::U256::from_big_endian(&[#arr])))
            }
            _ => panic!("unknown type"),
        },
        NysaExpression::Func { name, args } => match parse(name, storage_fields) {
            Ok(name) => {
                let args = parse_many(&args, storage_fields)?;
                Ok(parse_quote!(self.#name(#(#args),*)))
            }
            Err(err) => Err(err),
        },
        NysaExpression::SuperCall { name, args } => {
            dbg!(name);
            let name = utils::to_prefixed_snake_case_ident("super_", name);
            dbg!(&name);
            let args = parse_many(&args, storage_fields)?;
            Ok(parse_quote!(self.#name(#(#args),*)))
        }
        NysaExpression::TypeInfo { ty, property } => {
            let ty = parse(ty, storage_fields)?;
            let property = match property.as_str() {
                "max" => format_ident!("MAX"),
                "min" => format_ident!("MIN"),
                _ => panic!("Unknown property {}", property),
            };
            Ok(parse_quote!(#ty::#property))
        }
        NysaExpression::Type { ty } => {
            let ty = ty::parse_plain_type_from_ty(ty);
            Ok(parse_quote!(#ty))
        }
        NysaExpression::Power { left, right } => {
            let left = primitives::read_variable_or_parse(left, storage_fields)?;
            let right = primitives::read_variable_or_parse(right, storage_fields)?;
            Ok(parse_quote!(#left.pow(#right)))
        }
        NysaExpression::BoolLiteral(b) => Ok(parse_quote!(#b)),
        NysaExpression::Expr(e) => panic!("Unknown expression {:?}", e),
    }
}

pub fn parse_many(
    expressions: &[NysaExpression],
    storage_fields: &[StorageField],
) -> Result<Vec<syn::Expr>, &'static str> {
    expressions
        .iter()
        .map(|e| parse(e, storage_fields))
        .collect::<Result<Vec<syn::Expr>, _>>()
}

fn bin_op(
    left: &NysaExpression,
    right: &NysaExpression,
    op: BinOp,
    storage_fields: &[StorageField],
) -> Result<syn::Expr, &'static str> {
    let left = primitives::read_variable_or_parse(left, storage_fields)?;
    let right = primitives::read_variable_or_parse(right, storage_fields)?;
    Ok(parse_quote!(#left #op #right))
}
