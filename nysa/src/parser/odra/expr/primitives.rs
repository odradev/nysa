use syn::{parse_quote, BinOp};

use super::parse;
use crate::{
    model::{ir::NysaVar, NysaExpression},
    parser::odra::var::IsField,
    utils::to_snake_case_ident,
};
use quote::quote;

pub fn read_variable_or_parse(
    expr: &NysaExpression,
    storage_fields: &[NysaVar],
) -> Result<syn::Expr, &'static str> {
    match expr {
        NysaExpression::Variable { name } => parse_variable(name, None, storage_fields),
        _ => parse(expr, storage_fields),
    }
}

/// Parses an assign expression (=, +=, -=, *=, /=).
///
/// In solidity there is left-hand and right-hand statement
/// Eg.
/// right         left
///
/// totalSupply = balanceOf[msg.sender]
///
/// In Odra, if we update Variable or Mapping, there is a single expression.
/// Eg.
/// self.total_supply.set(self.balance_of.get(odra::contract_env::caller())).
///
/// To parse any kind of an assign statement we need to treat it a single statement
/// and parse both sides at once.
///
/// # Arguments
///
/// * `left` - Left-hand side solidity expression.
/// * `right` - Right-hand side solidity expression.
/// * `value_expr` - An optional operator (eg. +, -)
/// * `storage_fields` - A slice containing all the contract storage fields.
pub fn assign(
    left: &NysaExpression,
    right: &NysaExpression,
    operator: Option<BinOp>,
    storage_fields: &[NysaVar],
) -> Result<syn::Expr, &'static str> {
    if operator.is_none() {
        return if let NysaExpression::ArraySubscript { array, key } = left {
            let array = parse(array, storage_fields)?;
            let key = parse(&key.clone().unwrap(), storage_fields)?;
            let value = read_variable_or_parse(right, storage_fields)?;
            Ok(parse_quote!(#array.set(&#key, #value)))
        } else if let NysaExpression::Variable { name } = left {
            let right = read_variable_or_parse(right, storage_fields)?;
            parse_variable(&name, Some(right), storage_fields)
        } else {
            Err("Unsupported expr assign")
        };
    }

    match left {
        NysaExpression::ArraySubscript { array, key } => {
            let key_expr = key.clone().map(|boxed| boxed.clone());
            let value_expr = read_variable_or_parse(right, storage_fields)?;
            let current_value_expr = parse_mapping(array, &key_expr, None, storage_fields)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_mapping(array, &key_expr, Some(new_value), storage_fields)
        }
        NysaExpression::Variable { name } => {
            let current_value_expr = parse_variable(&name, None, storage_fields)?;
            let value_expr = read_variable_or_parse(right, storage_fields)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_variable(&name, Some(new_value), storage_fields)
        }
        _ => parse(left, storage_fields),
    }
}

/// Parses a single value interactions.
///
/// In solidity referring to a contract storage value and a local variable is the same.
/// When interacting with an Odra value, we need to know more context:
/// 1. If we use a module field or a local value
/// 2. If we reading/writing a value
///
/// # Arguments
///
/// * `id` - A variable identifier.
/// * `value_expr` - An optional value, if is some, it means we write to a var
/// * `storage_fields` - A slice containing all the contract storage fields.
///
/// # Returns
///
/// A parsed syn expression.
pub fn parse_variable(
    id: &str,
    value_expr: Option<syn::Expr>,
    storage_fields: &[NysaVar],
) -> Result<syn::Expr, &'static str> {
    let is_field = id.is_field(storage_fields);
    let ident = to_snake_case_ident(id);
    let self_ty = is_field.then(|| quote!(self.));
    if let Some(value) = value_expr {
        match is_field {
            // Variable update must use the `set` function
            true => Ok(parse_quote!(#self_ty #ident.set(#value))),
            // regular, local value
            false => Ok(parse_quote!(#ident = #value)),
        }
    } else {
        match is_field {
            true => Ok(parse_quote!(odra::UnwrapOrRevert::unwrap_or_revert(#self_ty #ident.get()))),
            false => Ok(parse_quote!(#self_ty #ident)),
        }
    }
}

pub fn parse_mapping(
    array_expr: &NysaExpression,
    key_expr: &Option<NysaExpression>,
    value_expr: Option<syn::Expr>,
    storage_fields: &[NysaVar],
) -> Result<syn::Expr, &'static str> {
    let array = parse(array_expr, storage_fields)?;

    if let Some(expr) = key_expr {
        let key = parse(expr, storage_fields)?;
        // TODO: check if it is a local array or contract storage.
        // TODO: what if the type does not implement Default trait.
        if let Some(value) = value_expr {
            Ok(parse_quote!(#array.set(&#key, #value)))
        } else {
            Ok(parse_quote!(odra::UnwrapOrRevert::unwrap_or_revert(#array.get(&#key))))
        }
    } else {
        Err("Unspecified key")
    }
}
