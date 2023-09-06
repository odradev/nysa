use proc_macro2::TokenStream;
use syn::{parse_quote, BinOp};

use super::parse;
use crate::{
    model::ir::{NysaExpression, NysaType, NysaVar},
    parser::{
        context::{
            self, ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, ItemType,
            StorageInfo, TypeInfo,
        },
        odra::expr::array,
    },
    utils::to_snake_case_ident,
    ParserError,
};
use quote::quote;

pub fn read_variable_or_parse<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    expr: &NysaExpression,
    t: &mut T,
) -> Result<syn::Expr, ParserError> {
    match expr {
        NysaExpression::Variable { name } => parse_variable(name, None, t),
        _ => parse(expr, t),
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
pub fn assign<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &NysaExpression,
    right: &NysaExpression,
    operator: Option<BinOp>,
    t: &mut T,
) -> Result<syn::Expr, ParserError> {
    // dbg!(left);
    // dbg!(right);
    if operator.is_none() {
        return if let NysaExpression::Collection { name, key } = left {
            let value = read_variable_or_parse(right, t)?;
            let keys = vec![*key.clone()];
            parse_collection(name, &keys, Some(value), t)
        } else if let NysaExpression::NestedCollection { name, keys } = left {
            let keys = vec![keys.0.clone(), keys.1.clone()];
            let value = read_variable_or_parse(right, t)?;
            parse_collection(name, &keys, Some(value), t)
        } else if let NysaExpression::Variable { name } = left {
            let right = read_variable_or_parse(right, t)?;
            parse_variable(&name, Some(right), t)
        } else {
            Err(ParserError::UnexpectedExpression(
                String::from(
                    "NysaExpression::Collection, NysaExpression::NestedCollection or NysaExpression::Variable",
                ),
                left.clone(),
            ))
        };
    }

    match left {
        NysaExpression::Collection { name, key } => {
            let keys = vec![*key.clone()];
            let value_expr = read_variable_or_parse(right, t)?;
            let current_value_expr = parse_collection(name, &keys, None, t)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_collection(name, &keys, Some(new_value), t)
        }
        NysaExpression::NestedCollection { name, keys } => {
            let keys = vec![keys.0.clone(), keys.1.clone()];
            let value_expr = read_variable_or_parse(right, t)?;
            let current_value_expr = parse_collection(name, &keys, None, t)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_collection(name, &keys, Some(new_value), t)
        }
        NysaExpression::Variable { name } => {
            let current_value_expr = parse_variable(&name, None, t)?;
            let value_expr = read_variable_or_parse(right, t)?;
            let new_value: syn::Expr = parse_quote!(#current_value_expr #operator #value_expr);
            parse_variable(&name, Some(new_value), t)
        }
        _ => parse(left, t),
    }
}

pub fn assign_default<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &NysaExpression,
    t: &mut T,
) -> Result<syn::Expr, ParserError> {
    if let NysaExpression::Variable { name } = left {
        let value_expr = parse_quote!(Default::default());
        parse_variable(&name, Some(value_expr), t)
    } else if let NysaExpression::Collection { name, key } = left {
        if let Some(context::ItemType::Storage(NysaVar {
            ty: NysaType::Array(ty),
            ..
        })) = t.type_from_string(name)
        {
            let default_value = parse_quote!(Default::default());
            array::replace_value(name, key, default_value, t)
        } else {
            Err(ParserError::UnexpectedExpression(
                String::from("NysaExpression::Variable"),
                left.clone(),
            ))
        }
    } else {
        Err(ParserError::UnexpectedExpression(
            String::from("NysaExpression::Variable"),
            left.clone(),
        ))
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
pub fn parse_variable<T: StorageInfo + TypeInfo + FnContext>(
    id: &str,
    value_expr: Option<syn::Expr>,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let item = ctx.type_from_string(id);
    let ident = to_snake_case_ident(id);

    if let Some(value) = value_expr {
        match item {
            // Variable update must use the `set` function
            Some(ItemType::Storage(_)) => Ok(parse_quote!(self.#ident.set(#value))),
            // regular, local value
            Some(ItemType::Local(_)) => Ok(parse_quote!(#ident = #value)),
            None => Ok(parse_quote!(#ident = #value)),
            _ => Err(ParserError::InvalidExpression),
        }
    } else {
        match item {
            Some(ItemType::Storage(v)) => Ok(get_expr(quote!(self.#ident), None, v.ty, ctx)),
            Some(ItemType::Local(_)) => Ok(parse_quote!(#ident)),
            None => Ok(parse_quote!(#ident)),
            _ => Err(ParserError::InvalidExpression),
        }
    }
}

pub fn parse_collection<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    name: &str,
    keys_expr: &[NysaExpression],
    value_expr: Option<syn::Expr>,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let ident = to_snake_case_ident(name);
    let field_ts = quote!(self.#ident);
    dbg!(ctx.type_from_string(name));
    let var = ctx
        .type_from_string(name)
        .map(|i| match i {
            ItemType::Storage(v) => Some(v),
            ItemType::Local(v) => Some(v),
            _ => None,
        })
        .flatten()
        .ok_or(ParserError::InvalidExpression)?;

    match keys_expr.len() {
        0 => return Err(ParserError::InvalidCollection),
        1 => {
            let key_expr = keys_expr.first().unwrap();
            let key = read_variable_or_parse(key_expr, ctx)?;
            if let Some(value) = value_expr {
                Ok(parse_quote!(#field_ts.set(&#key, #value)))
            } else {
                Ok(get_expr(field_ts, Some(key), var.ty, ctx))
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
                Ok(get_expr(token_stream, Some(key), var.ty, ctx))
            }
        }
    }
}

fn get_expr<T: StorageInfo + TypeInfo>(
    stream: TokenStream,
    key_expr: Option<syn::Expr>,
    ty: NysaType,
    t: &mut T,
) -> syn::Expr {
    let key = key_expr.clone().map(|k| quote!(&#k));
    match ty {
        NysaType::Address => parse_quote!(#stream.get(#key).unwrap_or(None)),
        NysaType::Custom(name) => t
            .type_from_string(&name)
            .map(|ty| match ty {
                context::ItemType::Contract(_) => {
                    parse_quote!(#stream.get(#key).unwrap_or(None))
                }
                context::ItemType::Interface(_) => {
                    parse_quote!(#stream.get(#key).unwrap_or(None))
                }
                context::ItemType::Enum(_) => parse_quote!(#stream.get_or_default(#key)),
                _ => parse_quote!(odra::UnwrapOrRevert::unwrap_or_revert(#stream.get(#key))),
            })
            .unwrap(),
        NysaType::String | NysaType::Bool | NysaType::Uint(_) | NysaType::Int(_) => {
            parse_quote!(#stream.get_or_default(#key))
        }
        NysaType::Mapping(_, v) => {
            let ty = NysaType::try_from(&*v).unwrap();
            get_expr(stream, key_expr, ty, t)
        }
        NysaType::Array(ty) => {
            let key = key_expr.and_then(|key| match &*ty {
                NysaType::Uint(size) => match size {
                    256..=512 => Some(quote!([#key.as_usize()])),
                    _ => Some(quote!([#key as usize])),
                },
                _ => Some(quote!([#key])),
            });
            parse_quote!(#stream.get_or_default()#key)
        }
        _ => parse_quote!(odra::UnwrapOrRevert::unwrap_or_revert(#stream.get(#key))),
    }
}
