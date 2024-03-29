use proc_macro2::{Ident, TokenStream};
use syn::{parse_quote, punctuated::Punctuated, BinOp, Token};

use super::{
    num, parse,
    syn_utils::{self, ArrayReader, DefaultValue, ReadValue, UnwrapOrNone, UnwrapOrRevert},
};
use crate::{
    error::ParserResult,
    formatted_invalid_expr,
    model::ir::{Expression, TupleItem, Type, Var},
    parser::{
        context::{
            self, ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext,
            ItemType, StorageInfo, TypeInfo,
        },
        odra::{
            expr::array,
            syn_utils::{in_context, AsExpression, AsSelfField},
        },
    },
    utils::{self, to_snake_case_ident},
    ParserError,
};
use quote::{format_ident, quote, ToTokens};

pub fn get_var_or_parse<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
>(
    expr: &Expression,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    match expr {
        Expression::Variable(name) => get_var(name, ctx),
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
/// self.total_supply.set(self.balance_of.get(self.env().caller())).
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
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
    O: Into<BinOp> + Clone,
>(
    left: &Expression,
    right: Option<&Expression>,
    operator: Option<O>,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    in_context(left, ctx, |ctx| match right {
        Some(right) => match left {
            Expression::Collection(name, keys) => {
                update_collection(name, keys, right, operator, ctx)
            }
            Expression::Variable(name) => update_variable(name, right, operator, ctx),
            Expression::Tuple(left_items) => update_tuple(left_items, right, operator, ctx),
            Expression::MemberAccess(field, var) => {
                let l = parse(left, ctx)?;
                let r = parse(right, ctx)?;
                Ok(parse_quote!(#l = #r))
            }
            _ => todo!(),
        },
        None => assign_default(left, ctx),
    })
}

/// Parses a single set value interaction.
///
/// In solidity referring to a contract storage value and a local variable is the same.
/// When interacting with an Odra value, we need to know more context if we use a module field or a local value
///
/// # Arguments
///
/// * `id` - A variable identifier.
/// * `value_expr` - am expression that writes to a var
/// * `ctx` - Parser context
///
/// # Returns
///
/// A parsed syn expression.
pub fn set_var<T: StorageInfo + TypeInfo + FnContext>(
    id: &str,
    value_expr: syn::Expr,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let item_type = ctx.type_from_string(id);
    let var = var(&item_type, id)?;

    match item_type {
        // Variable update must use the `set` function
        Some(ItemType::Storage(_)) => Ok(parse_quote!(#var.set(#value_expr))),
        _ => Ok(parse_quote!(#var = #value_expr)),
    }
}

/// Parses a single get value interactions.
///
/// In solidity referring to a contract storage value and a local variable is the same.
/// When interacting with an Odra value, we need to know more context if we use a module field or a local value
///
/// # Arguments
///
/// * `id` - A variable identifier.
/// * `ctx` - A slice containing all the contract storage fields.
///
/// # Returns
///
/// A parsed syn expression.
pub fn get_var<T: StorageInfo + TypeInfo + FnContext>(
    id: &str,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let item_type = ctx.type_from_string(id);
    let var = var(&item_type, id)?;

    match item_type {
        Some(ItemType::Storage(v)) => Ok(to_read_expr(var, None, &v.ty, ctx)),
        _ => Ok(var),
    }
}

pub fn parse_collection<T>(
    name: &str,
    keys_expr: &[Expression],
    value_expr: Option<syn::Expr>,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let ident = to_snake_case_ident(name);

    let item_type = ctx
        .type_from_string(name)
        .ok_or(ParserError::InvalidExpression(
            "unknown item type".to_string(),
        ))?;
    match &item_type {
        ItemType::Storage(v) => parse_storage_collection(ident, keys_expr, value_expr, &v.ty, ctx),
        ItemType::Local(v) => parse_local_collection(ident, keys_expr, value_expr, &v.ty, ctx),
        _ => formatted_invalid_expr!("unknown collection {:?}", item_type),
    }
}

fn assign_default<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
>(
    left: &Expression,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let err = || ParserError::UnexpectedExpression("Expression::Variable", left.clone());
    let default_expr = syn_utils::default();

    match left {
        Expression::Variable(name) => set_var(&name, default_expr, ctx),
        Expression::Collection(name, keys) => match ctx.type_from_string(name) {
            Some(ItemType::Storage(Var {
                ty: Type::Array(_), ..
            })) => array::replace_value(name, &keys[0], default_expr, ctx),
            _ => Err(err()),
        },
        _ => Err(err()),
    }
}

fn parse_local_collection<T>(
    var_ident: Ident,
    keys_expr: &[Expression],
    value_expr: Option<syn::Expr>,
    ty: &Type,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let keys_len = keys_expr.len();
    if keys_len == 0 {
        return Err(ParserError::InvalidCollection);
    }

    // A local mapping should not exists but eg. can be passed by a reference to a function.
    if let Type::Mapping(_, _) = ty {
        // iterate over nested mappings
        let key = match keys_expr.len() {
            0 => return Err(ParserError::InvalidCollection),
            1 => parse_storage_key(&keys_expr[0], ctx)?,
            _ => {
                let keys = keys_expr
                    .iter()
                    .map(|k| parse_storage_key(&k, ctx))
                    .collect::<ParserResult<Punctuated<syn::Expr, Token![,]>>>()?;
                keys.as_expression()
            }
        };
        if let Some(value) = value_expr {
            return Ok(parse_quote!(#var_ident.set(&#key, #value)));
        } else {
            return Ok(to_read_expr(var_ident, Some(key), ty, ctx));
        }
    }

    let mut collection = quote!(#var_ident);
    for i in 0..keys_len {
        let key = parse_local_key(&keys_expr[i], ctx)?;
        collection.extend(quote!([#key]));
    }

    let assign = value_expr.map(|e| quote!(= #e));
    Ok(parse_quote!(#collection #assign))
}

fn parse_storage_collection<T>(
    var_ident: Ident,
    keys_expr: &[Expression],
    value_expr: Option<syn::Expr>,
    ty: &Type,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let key = match keys_expr.len() {
        0 => return Err(ParserError::InvalidCollection),
        1 => parse_storage_key(&keys_expr[0], ctx)?,
        _ => {
            let keys = keys_expr
                .iter()
                .map(|k| parse_storage_key(&k, ctx))
                .collect::<ParserResult<Punctuated<syn::Expr, Token![,]>>>()?;
            keys.as_expression()
        }
    };

    let field = var_ident.as_self_field();
    match value_expr {
        Some(value) => Ok(parse_quote!(#field.set(&#key, #value))),
        None => Ok(to_read_expr(field, Some(key), ty, ctx)),
    }
}

fn parse_storage_key<T>(key: &Expression, ctx: &mut T) -> ParserResult<syn::Expr>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    match key {
        Expression::NumberLiteral(v) => num::to_typed_int_expr(v, ctx),
        _ => get_var_or_parse(key, ctx),
    }
}

fn parse_local_key<T>(key: &Expression, ctx: &mut T) -> ParserResult<syn::Expr>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    match num::try_to_generic_int_expr(key) {
        Ok(e) => Ok(e),
        Err(_) => get_var_or_parse(key, ctx),
    }
}

fn to_read_expr<F: ToTokens, T: StorageInfo + TypeInfo>(
    field: F,
    key_expr: Option<syn::Expr>,
    ty: &Type,
    ctx: &mut T,
) -> syn::Expr {
    let key = key_expr.clone().map(|k| quote!(&#k));
    match ty {
        Type::Address => <UnwrapOrNone as ReadValue>::expr(field, key),
        Type::Custom(name) => ctx
            .type_from_string(&name)
            .map(|ty| match ty {
                context::ItemType::Contract(_)
                | context::ItemType::Library(_)
                | context::ItemType::Interface(_) => <UnwrapOrNone as ReadValue>::expr(field, key),
                context::ItemType::Enum(_) => <DefaultValue as ReadValue>::expr(field, key),
                _ => <UnwrapOrRevert as ReadValue>::expr(field, key),
            })
            .unwrap(),
        Type::String | Type::Bool | Type::Uint(_) | Type::Int(_) => {
            <DefaultValue as ReadValue>::expr(field, key)
        }
        Type::Mapping(_, v) => {
            let ty = match ctx.type_from_expression(v) {
                Some(ItemType::Struct(s)) => Type::Custom(s.name),
                _ => Type::try_from(&**v).unwrap(),
            };

            to_read_expr(field, key_expr, &ty, ctx)
        }
        Type::Array(ty) => {
            let key = key_expr.and_then(|key| match &**ty {
                Type::Uint(size) => match size {
                    256..=512 => Some(quote!([#key.as_usize()])),
                    _ => Some(quote!([#key as usize])),
                },
                _ => Some(quote!([#key])),
            });
            <ArrayReader as ReadValue>::expr(field, key)
        }
        _ => <UnwrapOrRevert as ReadValue>::expr(field, key),
    }
}

fn update_collection<T, O>(
    name: &str,
    keys: &[Expression],
    right: &Expression,
    operator: Option<O>,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
    O: Into<BinOp>,
{
    if operator.is_none() {
        let value = get_var_or_parse(right, ctx)?;
        parse_collection(name, &keys, Some(value), ctx)
    } else {
        let op = operator.map(Into::<BinOp>::into);
        let value_expr = get_var_or_parse(right, ctx)?;
        let current_value_expr = parse_collection(name, &keys, None, ctx)?;
        let new_value: syn::Expr = parse_quote!(#current_value_expr #op #value_expr);
        parse_collection(name, &keys, Some(new_value), ctx)
    }
}

fn update_variable<T, O>(
    name: &str,
    right: &Expression,
    operator: Option<O>,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
    O: Into<BinOp>,
{
    if operator.is_none() {
        let right = get_var_or_parse(right, ctx)?;
        set_var(&name, right, ctx)
    } else {
        let op = operator.map(Into::<BinOp>::into);
        let current_value_expr = get_var(&name, ctx)?;
        let value_expr = get_var_or_parse(right, ctx)?;
        let new_value: syn::Expr = parse_quote!(#current_value_expr #op #value_expr);
        set_var(&name, new_value, ctx)
    }
}

fn update_tuple<T, O>(
    left: &[TupleItem],
    right: &Expression,
    operator: Option<O>,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
    O: Into<BinOp> + Clone,
{
    // a tuple that defines local variables
    // sol: (uint a, uint b) = (1, 1);
    if left
        .iter()
        .all(|i| matches!(i, TupleItem::Declaration(_, _)))
    {
        let items = left
            .iter()
            .filter_map(|i| match i {
                TupleItem::Declaration(ty, name) => Some((ty, name)),
                _ => None,
            })
            .map(|(e, n)| {
                let name = utils::to_snake_case_ident(n);
                let ty = TryFrom::try_from(e).unwrap();
                ctx.register_local_var(n, &ty);
                quote!(mut #name)
            })
            .collect::<syn::punctuated::Punctuated<TokenStream, syn::Token![,]>>();
        let values = super::parse(right, ctx)?;

        return Ok(parse_quote!(let (#items) = #values));
    } else {
        // a tuple that defines update a tuple - may be multiple local/state variables or mix of both.

        if let Expression::Tuple(values) = right {
            // The lvalue is a tuple
            // sol: (a, b) = (1, 1);
            // rs: {
            //   a = 1;
            //   b = 2;
            // }
            // However the syntax (a, b) = (1, 1) is correct in rust, if a variable is a state variable
            // Odra uses `set()` function not the `=` operator
            let items: Vec<syn::Stmt> = parse_tuple_statements(left, values, operator, ctx)?;
            return Ok(parse_quote!( { #(#items)* } ));
        } else {
            // The lvalue is an expression that returns a tuple.
            // sol: (a, b) = func_call();
            // rs: {
            //   let (_0, _1) = func_call();
            //   a = _0;
            //   b = _1;
            // }
            // Due to the same reason as above a more verbose syntax is required.
            let names = (0..left.len())
                .map(|idx| format_ident!("_{}", idx))
                .collect::<syn::punctuated::Punctuated<Ident, syn::Token![,]>>();
            let values = super::parse(right, ctx)?;

            let tmp_items = (0..left.len())
                .map(|idx| TupleItem::Expr(Expression::Variable(format!("_{}", idx))))
                .collect::<Vec<_>>();

            let assignment: Vec<syn::Stmt> =
                parse_tuple_statements(left, &tmp_items, operator, ctx)?;

            return Ok(parse_quote!({
                let (#names) = #values;
                #(#assignment)*
            }));
        }
    }
}

fn parse_tuple_statements<T, O>(
    left: &[TupleItem],
    right: &[TupleItem],
    operator: Option<O>,
    ctx: &mut T,
) -> Result<Vec<syn::Stmt>, ParserError>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
    O: Into<BinOp> + Clone,
{
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| {
            if let TupleItem::Expr(r) = r {
                match l {
                    TupleItem::Expr(l) => {
                        assign(l, Some(r), operator.clone(), ctx).map(|e| parse_quote!(#e;))
                    }
                    TupleItem::Wildcard => {
                        let value = super::parse(r, ctx)?;
                        Ok(parse_quote!(let _ =  #value;))
                    }
                    TupleItem::Declaration(_, _) => formatted_invalid_expr!("invalid tuple item"),
                }
            } else {
                formatted_invalid_expr!("invalid tuple item")
            }
        })
        .collect::<Result<Vec<syn::Stmt>, ParserError>>()
}

fn var(item_type: &Option<ItemType>, id: &str) -> ParserResult<syn::Expr> {
    let ident = to_snake_case_ident(id);

    match item_type {
        // Variable update must use the `set` function
        Some(ItemType::Storage(_)) => Ok(ident.as_self_field()),
        // regular, local value
        Some(ItemType::Local(_)) => Ok(ident.as_expression()),
        None => Ok(ident.as_expression()),
        _ => formatted_invalid_expr!("unknown variable {:?}", item_type),
    }
}
