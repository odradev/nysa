use solidity_parser::pt;
use syn::{parse_quote, BinOp};

use super::parse;
use crate::{utils::to_snake_case_ident, var::IsField};
use quote::quote;

pub fn read_value(
    expr: &pt::Expression,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Expr, &'static str> {
    match expr {
        pt::Expression::Variable(id) => parse_variable(id, None, storage_fields),
        _ => parse(expr, storage_fields),
    }
}

pub fn assign(
    left: &pt::Expression,
    right: &pt::Expression,
    operator: Option<BinOp>,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Expr, &'static str> {
    if operator.is_none() {
        return if let pt::Expression::ArraySubscript(_, array_expr, key_expr) = left {
            let array = parse(&*array_expr, storage_fields)?;
            let key = parse(&key_expr.clone().unwrap(), storage_fields)?;
            let value = read_value(right, storage_fields)?;
            Ok(parse_quote!(#array.set(&#key, #value)))
        } else if let pt::Expression::Variable(id) = left {
            let right = parse(right, storage_fields)?;
            parse_variable(id, Some(right), storage_fields)
        } else {
            Err("Unsupported expr assign")
        };
    }

    match left {
        pt::Expression::ArraySubscript(id, array_expr, key_expr) => {
            let key_expr = key_expr.clone().map(|boxed| *boxed.clone());
            let value_expr = parse(&right, storage_fields)?;
            let current_value_expr =
                parse_mapping(array_expr, key_expr.clone(), None, storage_fields)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_mapping(array_expr, key_expr, Some(new_value), storage_fields)
        }
        pt::Expression::Variable(id) => {
            let current_value_expr = parse_variable(id, None, storage_fields)?;
            let value_expr = parse(&right, storage_fields)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_variable(id, Some(new_value), storage_fields)
        }
        _ => parse(left, storage_fields),
    }
}

pub fn parse_variable(
    id: &pt::Identifier,
    value_expr: Option<syn::Expr>,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Expr, &'static str> {
    let is_field = id.is_field(storage_fields);
    let ident = id.name.as_str();
    let ident = to_snake_case_ident(ident);
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
    array_expr: &pt::Expression,
    key_expr: Option<pt::Expression>,
    value_expr: Option<syn::Expr>,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Expr, &'static str> {
    let array = parse(array_expr, storage_fields)?;

    if let Some(expr) = key_expr {
        let key = parse(&expr, storage_fields)?;
        // TODO: check if it is a local array or contract storage.
        // TODO: what if the type does not implement Default trait.
        if let Some(value) = value_expr {
            Ok(parse_quote!(#array.set(&#key, #value)))
        } else {
            Ok(parse_quote!(#array.get(&#key).unwrap_or_default()))
        }
    } else {
        Err("Unspecified key")
    }
}
