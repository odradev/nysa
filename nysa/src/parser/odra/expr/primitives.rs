use proc_macro2::TokenStream;
use syn::{parse_quote, BinOp};

use super::parse;
use crate::{
    model::ir::{NysaExpression, NysaType, NysaVar},
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
        return if let NysaExpression::Mapping { name, key } = left {
            let value = read_variable_or_parse(right, storage_fields)?;
            let keys = vec![*key.clone()];
            parse_mapping(name, &keys, Some(value), storage_fields)
        } else if let NysaExpression::Mapping2 { name, keys } = left {
            let keys = vec![keys.0.clone(), keys.1.clone()];
            let value = read_variable_or_parse(right, storage_fields)?;
            parse_mapping(name, &keys, Some(value), storage_fields)
        } else if let NysaExpression::Variable { name } = left {
            let right = read_variable_or_parse(right, storage_fields)?;
            parse_variable(&name, Some(right), storage_fields)
        } else {
            Err("Unsupported expr assign")
        };
    }

    match left {
        NysaExpression::Mapping { name, key } => {
            let keys = vec![*key.clone()];
            let value_expr = read_variable_or_parse(right, storage_fields)?;
            let current_value_expr = parse_mapping(name, &keys, None, storage_fields)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_mapping(name, &keys, Some(new_value), storage_fields)
        }
        NysaExpression::Mapping2 { name, keys } => {
            let keys = vec![keys.0.clone(), keys.1.clone()];
            let value_expr = read_variable_or_parse(right, storage_fields)?;
            let current_value_expr = parse_mapping(name, &keys, None, storage_fields)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_mapping(name, &keys, Some(new_value), storage_fields)
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
    let self_ty = is_field.is_some().then(|| quote!(self.));
    if let Some(value) = value_expr {
        match is_field {
            // Variable update must use the `set` function
            Some(_) => Ok(parse_quote!(#self_ty #ident.set(#value))),
            // regular, local value
            None => Ok(parse_quote!(#ident = #value)),
        }
    } else {
        match is_field {
            Some(ty) => Ok(get_expr(quote!(#self_ty #ident), None, ty)),
            None => Ok(parse_quote!(#self_ty #ident)),
        }
    }
}

pub fn parse_mapping(
    name: &str,
    keys_expr: &[NysaExpression],
    value_expr: Option<syn::Expr>,
    storage_fields: &[NysaVar],
) -> Result<syn::Expr, &'static str> {
    let ident = to_snake_case_ident(name);
    let field_ts = quote!(self.#ident);
    let ty = name
        .is_field(storage_fields)
        .expect("Mapping must be a field");

    match keys_expr.len() {
        0 => panic!("Invalid mapping keys"),
        1 => {
            let key_expr = keys_expr.first().unwrap();
            let key = read_variable_or_parse(key_expr, storage_fields)?;
            if let Some(value) = value_expr {
                Ok(parse_quote!(#field_ts.set(&#key, #value)))
            } else {
                Ok(get_expr(field_ts, Some(key), ty))
            }
        }
        n => {
            let mut token_stream = field_ts;

            for i in 0..n - 1 {
                let key = read_variable_or_parse(&keys_expr[0], storage_fields)?;
                token_stream.extend(quote!(.get_instance(&#key)));
            }
            let key = read_variable_or_parse(keys_expr.last().unwrap(), storage_fields)?;
            if let Some(value) = value_expr {
                Ok(parse_quote!(#token_stream.set(&#key, #value)))
            } else {
                Ok(get_expr(token_stream, Some(key), ty))
            }
        }
    }
}

fn get_expr(stream: TokenStream, key_expr: Option<syn::Expr>, ty: NysaType) -> syn::Expr {
    let key = key_expr.clone().map(|k| quote!(&#k));
    match ty {
        NysaType::Address => parse_quote!(#stream.get(#key).unwrap_or(None)),
        NysaType::String | NysaType::Bool | NysaType::Uint(_) | NysaType::Int(_) => {
            parse_quote!(#stream.get_or_default(#key))
        }
        NysaType::Mapping(_, v) => {
            let ty = NysaType::try_from(&*v).unwrap();
            get_expr(stream, key_expr, ty)
        }
        _ => parse_quote!(odra::UnwrapOrRevert::unwrap_or_revert(#stream.get(#key))),
    }
}

pub fn to_generic_lit_expr<N: num_traits::Num + ToString>(num: N) -> syn::Expr {
    syn::Expr::Lit(syn::ExprLit {
        attrs: vec![],
        lit: syn::Lit::Int(syn::LitInt::new(
            &num.to_string(),
            proc_macro2::Span::call_site(),
        )),
    })
}
