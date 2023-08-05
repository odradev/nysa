use quote::format_ident;
use quote::quote;
use syn::parse_quote;

use crate::{utils::to_snake_case_ident, var::IsField};

use self::values::NysaExpression;
use self::values::StorageField;

mod error;
pub mod primitives;
pub mod values;

pub fn parse(
    expression: &NysaExpression,
    storage_fields: &[StorageField],
) -> Result<syn::Expr, &'static str> {
    match expression {
        NysaExpression::Require { condition, error } => error::revert(condition, error, storage_fields),
        NysaExpression::Wildcard => Err("Empty identifier"),
        NysaExpression::ZeroAddress => Ok(parse_quote!(None)),
        NysaExpression::Message(msg) => msg.try_into(),
        NysaExpression::ArraySubscript { array, key } => {
            primitives::parse_mapping(array, key, None, storage_fields)
        },
        NysaExpression::Variable { name } => {
            let ident = to_snake_case_ident(&name);
            let self_ty = name.is_field(storage_fields).then(|| quote!(self.));
            Ok(parse_quote!(#self_ty #ident))
        },
        NysaExpression::Assign { left, right } => {
            primitives::assign(left, right, None, storage_fields)
        }
        NysaExpression::StringLiteral(string) => Ok(parse_quote!(#string)),
        NysaExpression::LessEqual { left, right } => {
            let l = parse(left, storage_fields)?;
            let r = parse(right, storage_fields)?;
            Ok(parse_quote!(#l <= #r))
        },
        NysaExpression::MoreEqual { left, right } => {
            let l = parse(left, storage_fields)?;
            let r = parse(right, storage_fields)?;
            Ok(parse_quote!(#l >= #r))
        },
        NysaExpression::Add { left, right } => {
            let l = parse(left, storage_fields)?;
            let r = parse(right, storage_fields)?;
            Ok(parse_quote!(#l + #r))
        },
        NysaExpression::Subtract { left, right } => {
            let l = parse(left, storage_fields)?;
            let r = parse(right, storage_fields)?;
            Ok(parse_quote!(#l - #r))
        },
        NysaExpression::Equal { left, right } => {
            let left = primitives::read_variable_or_parse(left, storage_fields)?;
            let right = primitives::read_variable_or_parse(right, storage_fields)?;
            Ok(parse_quote!(#left == #right))
        },
        NysaExpression::NotEqual { left, right } => {
            let left = primitives::read_variable_or_parse(left, storage_fields)?;
            let right = primitives::read_variable_or_parse(right, storage_fields)?;
            Ok(parse_quote!(#left != #right))
        },
        NysaExpression::AssignSubtract { left, right } =>  {
            let expr = primitives::assign(left, right, Some(parse_quote!(-)), storage_fields)?;
            Ok(expr)
        },
        NysaExpression::AssignAdd { left, right } => {
            let expr = primitives::assign(left, right, Some(parse_quote!(+)), storage_fields)?;
            Ok(expr)
        },
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
        },
        NysaExpression::NumberLiteral(num) => Ok(parse_quote!(#num)),
        NysaExpression::Func { name, args } => match parse(name, storage_fields) {
            Ok(name) => {
                let args = parse_many(&args, storage_fields)?;
                Ok(parse_quote!(self.#name(#(#args),*)))
            }
            Err(err) => Err(err),
        },
        NysaExpression::Expr(_) => Err("Unknown expression"),
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
