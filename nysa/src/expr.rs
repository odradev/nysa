use quote::format_ident;
use quote::quote;
use solidity_parser::pt;
use syn::parse_quote;

use crate::{utils::to_snake_case_ident, var::IsField};

use self::values::Expression;

mod error;
pub mod primitives;
mod values;

/// Parses solidity expression into a syn expression.
///
/// Todo: to handle remaining expressions.
pub fn parse(
    expression: &pt::Expression,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Expr, &'static str> {
    let value = Expression::from(expression);
    match value {
        Expression::Require { condition, error } => error::revert(condition, error, storage_fields),
        Expression::Wildcard => Err("Empty identifier"),
        Expression::ZeroAddress => Ok(parse_quote!(None)),
        Expression::Message(msg) => msg.try_into(),
        Expression::Expr(expr) => match expression {
            pt::Expression::ArraySubscript(_, array_expression, key_expression) => {
                let key_expr = key_expression.clone().map(|key_expr| *key_expr);
                primitives::parse_mapping(array_expression, key_expr, None, storage_fields)
            }
            pt::Expression::MemberAccess(_, expression, id) => {
                let base_expr: syn::Expr = parse(expression, storage_fields)?;
                let member: syn::Member = format_ident!("{}", id.name).into();
                Ok(parse_quote!(#base_expr.#member))
            }
            pt::Expression::Assign(_, le, re) => {
                let le: &pt::Expression = le;
                let re: &pt::Expression = re;
                primitives::assign(le, re, None, storage_fields)
            }
            pt::Expression::Variable(id) => {
                let name = id.name.as_str();
                let ident = to_snake_case_ident(name);
                let self_ty = id.is_field(storage_fields).then(|| quote!(self.));
                Ok(parse_quote!(#self_ty #ident))
            }
            pt::Expression::FunctionCall(_, name, args) => match parse(name, storage_fields) {
                Ok(name) => {
                    let args = parse_many(args, storage_fields)?;
                    Ok(parse_quote!(self.#name(#(#args),*)))
                }
                Err(err) => Err(err),
            },
            pt::Expression::LessEqual(_, l, r) => {
                let l = parse(l, storage_fields)?;
                let r = parse(r, storage_fields)?;
                Ok(parse_quote!(#l <= #r))
            }
            pt::Expression::MoreEqual(_, l, r) => {
                let l = parse(l, storage_fields)?;
                let r = parse(r, storage_fields)?;
                Ok(parse_quote!(#l >= #r))
            }
            pt::Expression::NumberLiteral(_, num) => {
                let (sign, digs) = num.to_u32_digits();
                let num = digs[0];
                Ok(parse_quote!(#num))
            }
            pt::Expression::Add(_, l, r) => {
                let l = parse(l, storage_fields)?;
                let r = parse(r, storage_fields)?;
                Ok(parse_quote!(#l + #r))
            }
            pt::Expression::Subtract(_, l, r) => {
                let l = parse(l, storage_fields)?;
                let r = parse(r, storage_fields)?;
                Ok(parse_quote!(#l - #r))
            }
            pt::Expression::PostIncrement(_, expression) => {
                let expr = parse(expression, storage_fields)?;
                // TODO: find out if it is a member or a local variable
                Ok(parse_quote!(#expr += 1))
            }
            pt::Expression::PostDecrement(_, expression) => {
                let expr = parse(expression, storage_fields)?;
                // TODO: find out if it is a member or a local variable
                Ok(parse_quote!(#expr -= 1))
            }
            pt::Expression::PreIncrement(_, _) => {
                let expr = parse(expression, storage_fields)?;
                Ok(parse_quote!(#expr += 1))
            }
            pt::Expression::PreDecrement(_, _) => {
                let expr = parse(expression, storage_fields)?;
                Ok(parse_quote!(#expr -= 1))
            }
            pt::Expression::Equal(_, left, right) => {
                let left = primitives::read_value(&*left, storage_fields)?;
                let right = primitives::read_value(&*right, storage_fields)?;
                Ok(parse_quote!(#left == #right))
            }
            pt::Expression::NotEqual(_, left, right) => {
                let left = primitives::read_value(&*left, storage_fields)?;
                let right = primitives::read_value(&*right, storage_fields)?;
                Ok(parse_quote!(#left != #right))
            }
            pt::Expression::StringLiteral(strings) => {
                let strings = strings
                    .iter()
                    .map(|lit| lit.string.clone())
                    .collect::<Vec<_>>();
                let string = strings.join(",");
                Ok(parse_quote!(#string))
            }
            pt::Expression::AssignSubtract(_, left, right) => {
                let expr = primitives::assign(left, right, Some(parse_quote!(-)), storage_fields)?;
                Ok(expr)
            }
            pt::Expression::AssignAdd(_, left, right) => {
                let expr = primitives::assign(left, right, Some(parse_quote!(+)), storage_fields)?;
                Ok(expr)
            }
            pt::Expression::Type(_, ty) => {
                Ok(parse_quote!(None))
            }
            _ => panic!("Unsupported expression {:?}", expression),
        },
    }
}

pub fn parse_many(
    expressions: &[pt::Expression],
    storage_fields: &[&pt::VariableDefinition],
) -> Result<Vec<syn::Expr>, &'static str> {
    expressions
        .iter()
        .map(|e| parse(e, storage_fields))
        .collect::<Result<Vec<syn::Expr>, _>>()
}
