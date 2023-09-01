use proc_macro2::TokenStream;
use syn::{parse_quote, BinOp};

use super::parse;
use crate::{
    model::ir::{NysaExpression, NysaType},
    parser::{context::Context, odra::var::AsVariable},
    utils::to_snake_case_ident,
    ParserError,
};
use quote::quote;

pub fn read_variable_or_parse(
    expr: &NysaExpression,
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    match expr {
        NysaExpression::Variable { name } => parse_variable(name, None, ctx),
        _ => parse(expr, ctx),
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
/// * `ctx` - parser Context.
pub fn assign(
    left: &NysaExpression,
    right: &NysaExpression,
    operator: Option<BinOp>,
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    if operator.is_none() {
        return if let NysaExpression::Mapping { name, key } = left {
            let value = read_variable_or_parse(right, ctx)?;
            let keys = vec![*key.clone()];
            parse_mapping(name, &keys, Some(value), ctx)
        } else if let NysaExpression::Mapping2 { name, keys } = left {
            let keys = vec![keys.0.clone(), keys.1.clone()];
            let value = read_variable_or_parse(right, ctx)?;
            parse_mapping(name, &keys, Some(value), ctx)
        } else if let NysaExpression::Variable { name } = left {
            let right = read_variable_or_parse(right, ctx)?;
            parse_variable(&name, Some(right), ctx)
        } else {
            Err(ParserError::UnexpectedExpression(
                String::from(
                    "NysaExpression::Mapping, NysaExpression::Mapping2 or NysaExpression::Variable",
                ),
                left.clone(),
            ))
        };
    }

    match left {
        NysaExpression::Mapping { name, key } => {
            let keys = vec![*key.clone()];
            let value_expr = read_variable_or_parse(right, ctx)?;
            let current_value_expr = parse_mapping(name, &keys, None, ctx)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_mapping(name, &keys, Some(new_value), ctx)
        }
        NysaExpression::Mapping2 { name, keys } => {
            let keys = vec![keys.0.clone(), keys.1.clone()];
            let value_expr = read_variable_or_parse(right, ctx)?;
            let current_value_expr = parse_mapping(name, &keys, None, ctx)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_mapping(name, &keys, Some(new_value), ctx)
        }
        NysaExpression::Variable { name } => {
            let current_value_expr = parse_variable(&name, None, ctx)?;
            let value_expr = read_variable_or_parse(right, ctx)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_variable(&name, Some(new_value), ctx)
        }
        _ => parse(left, ctx),
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
/// * `ctx` - A slice containing all the contract storage fields.
///
/// # Returns
///
/// A parsed syn expression.
pub fn parse_variable(
    id: &str,
    value_expr: Option<syn::Expr>,
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    let var = id.as_var(ctx);
    let ident = to_snake_case_ident(id);
    let self_ty = var.is_ok().then(|| quote!(self.));
    if let Some(value) = value_expr {
        match var {
            // Variable update must use the `set` function
            Ok(_) => Ok(parse_quote!(#self_ty #ident.set(#value))),
            // regular, local value
            Err(_) => Ok(parse_quote!(#ident = #value)),
        }
    } else {
        match var {
            Ok(ty) => Ok(get_expr(quote!(#self_ty #ident), None, ty)),
            Err(_) => Ok(parse_quote!(#self_ty #ident)),
        }
    }
}

pub fn parse_mapping(
    name: &str,
    keys_expr: &[NysaExpression],
    value_expr: Option<syn::Expr>,
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    let ident = to_snake_case_ident(name);
    let field_ts = quote!(self.#ident);
    let ty = name.as_var(ctx)?;

    match keys_expr.len() {
        0 => return Err(ParserError::InvalidMapping),
        1 => {
            let key_expr = keys_expr.first().unwrap();
            let key = read_variable_or_parse(key_expr, ctx)?;
            if let Some(value) = value_expr {
                Ok(parse_quote!(#field_ts.set(&#key, #value)))
            } else {
                Ok(get_expr(field_ts, Some(key), ty))
            }
        }
        n => {
            let mut token_stream = field_ts;

            for i in 0..n - 1 {
                let key = read_variable_or_parse(&keys_expr[0], ctx)?;
                token_stream.extend(quote!(.get_instance(&#key)));
            }
            let key = read_variable_or_parse(keys_expr.last().unwrap(), ctx)?;
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
        NysaType::Contract(_) => parse_quote!(#stream.get(#key).unwrap_or(None)),
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
