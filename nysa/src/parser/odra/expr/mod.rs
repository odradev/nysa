use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::model::ir::{self, Expression, Type, Var};
use crate::parser::context::{
    ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, ItemType, StorageInfo, TypeInfo,
};
use crate::utils;
use crate::ParserError;

use super::stmt::parse_statement;
use super::ty;

mod array;
pub(crate) mod error;
mod math;
mod num;
mod op;
pub(crate) mod primitives;
#[cfg(test)]
mod test;

pub fn parse<T>(expression: &Expression, ctx: &mut T) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    match expression {
        Expression::Require { condition, error } => error::revert(Some(condition), error, ctx),
        Expression::Placeholder => Err(ParserError::EmptyExpression),
        Expression::ZeroAddress => Ok(parse_quote!(None)),
        Expression::Message(msg) => msg.try_into(),
        Expression::Collection { name, key } => {
            let keys = vec![*key.clone()];
            primitives::parse_collection(name, &keys, None, ctx)
        }
        Expression::NestedCollection { name, keys } => {
            let keys = vec![keys.0.clone(), keys.1.clone()];
            primitives::parse_collection(name, &keys, None, ctx)
        }
        Expression::Variable { name } => {
            let ident = utils::to_snake_case_ident(name);
            let self_ty = ctx
                .type_from_string(name)
                .filter(|i| matches!(i, ItemType::Storage(_)))
                .map(|_| quote!(self.));
            Ok(parse_quote!(#self_ty #ident))
        }
        Expression::Assign { left, right } => primitives::assign(left, right, None, ctx),
        Expression::AssignDefault { left } => primitives::assign_default(left, ctx),
        Expression::StringLiteral(string) => {
            Ok(parse_quote!(odra::prelude::string::String::from(#string)))
        }
        Expression::Compare {
            var_left,
            left,
            var_right,
            right,
            op,
        } => {
            let op = match op {
                ir::Op::Less => parse_quote!(<),
                ir::Op::LessEq => parse_quote!(<=),
                ir::Op::More => parse_quote!(>),
                ir::Op::MoreEq => parse_quote!(>=),
                ir::Op::Eq => parse_quote!(==),
                ir::Op::NotEq => parse_quote!(!=),
            };
            op::bin_op(var_left, var_right, left, right, op, ctx)
        }
        Expression::Add { left, right } => math::add(left, right, ctx),
        Expression::Multiply { left, right } => math::mul(left, right, ctx),
        Expression::Divide { left, right } => math::div(left, right, ctx),
        Expression::Subtract { left, right } => math::sub(left, right, ctx),
        Expression::AssignSubtract { left, right } => {
            let expr = primitives::assign(left, right, Some(parse_quote!(-)), ctx)?;
            Ok(expr)
        }
        Expression::AssignAdd { left, right } => {
            let expr = primitives::assign(left, right, Some(parse_quote!(+)), ctx)?;
            Ok(expr)
        }
        Expression::Or { left, right } => {
            op::bin_op(&None, &None, left, right, parse_quote!(||), ctx)
        }
        Expression::Increment { expr } => {
            let expr = parse(expr, ctx)?;
            Ok(parse_quote!(#expr += 1))
        }
        Expression::Decrement { expr } => {
            let expr = parse(expr, ctx)?;
            Ok(parse_quote!(#expr -= 1))
        }
        Expression::MemberAccess { expr, name } => parse_member_access(name, expr, ctx),
        Expression::NumberLiteral { ty, value } => num::to_typed_int_expr(ty, value),
        Expression::Func { name, args } => parse_func(name, args, ctx),
        Expression::SuperCall { name, args } => parse_super_call(name, args, ctx),
        Expression::ExternalCall {
            variable,
            fn_name,
            args,
        } => parse_ext_call(variable, fn_name, args, ctx),
        Expression::TypeInfo { ty, property } => {
            let ty = parse(ty, ctx)?;
            let property = match property.as_str() {
                "max" => format_ident!("MAX"),
                "min" => format_ident!("MIN"),
                p => return Err(ParserError::UnknownProperty(p.to_string())),
            };
            Ok(parse_quote!(#ty::#property))
        }
        Expression::Type { ty } => {
            let ty = ty::parse_plain_type_from_ty(ty, ctx)?;
            Ok(parse_quote!(#ty))
        }
        Expression::Power { left, right } => {
            let left = primitives::get_var_or_parse(left, ctx)?;
            let right = primitives::get_var_or_parse(right, ctx)?;
            Ok(parse_quote!(#left.pow(#right)))
        }
        Expression::BoolLiteral(b) => Ok(parse_quote!(#b)),
        Expression::Not { expr } => {
            let expr = primitives::get_var_or_parse(expr, ctx)?;
            Ok(parse_quote!(!(#expr)))
        }
        Expression::BytesLiteral { bytes } => {
            let arr = bytes
                .iter()
                .map(|v| quote::quote!(#v))
                .collect::<Punctuated<TokenStream, Token![,]>>();
            let size = bytes.len();
            Ok(parse_quote!([#arr]))
        }
        Expression::ArrayLiteral { values } => {
            let arr = values
                .iter()
                .map(|expression| parse(expression, ctx))
                .map(|e| match e {
                    Ok(r) => Ok(quote!(#r)),
                    Err(e) => Err(e),
                })
                .collect::<Result<Punctuated<TokenStream, Token![,]>, ParserError>>()?;
            Ok(parse_quote!(odra::prelude::vec![#arr]))
        }
        Expression::Initializer(expr) => {
            if let box Expression::Func { name, args } = expr {
                if let box Expression::Type { ty: Type::Array(_) } = name {
                    let args = parse_many(&args, ctx)?;
                    return Ok(parse_quote!(odra::prelude::vec::Vec::with_capacity(#(#args),*)));
                }
            }
            dbg!(expr);
            todo!()
        }
        Expression::Statement(s) => parse_statement(s, false, ctx)
            .map(|stmt| match stmt {
                syn::Stmt::Expr(e) => Ok(e),
                _ => Err(ParserError::InvalidExpression),
            })
            .unwrap(),
        Expression::UnknownExpr => panic!("Unknown expression"),
    }
}

pub fn parse_many<T>(expressions: &[Expression], ctx: &mut T) -> Result<Vec<syn::Expr>, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    expressions
        .iter()
        .map(|e| parse(e, ctx))
        .collect::<Result<Vec<syn::Expr>, _>>()
}

fn parse_func<T>(
    fn_name: &Expression,
    args: &[Expression],
    ctx: &mut T,
) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let args = parse_many(&args, ctx)?;
    // Context allows us to distinct an external contract initialization from a regular function call
    if let Some(class_name) = ctx.as_contract_name(fn_name) {
        ctx.register_external_call(&class_name);
        // Storing a reference to a contract is disallowed, and in the constructor an external contract
        // should be considered an address, otherwise, a reference should be created
        return if ctx.current_fn().is_constructor() {
            Ok(parse_quote!(#(#args),*))
        } else {
            let ref_ident = format_ident!("{}Ref", class_name);
            let addr = args.get(0);
            Ok(parse_quote!(#ref_ident::at(&odra::UnwrapOrRevert::unwrap_or_revert(#addr))))
        };
    }
    match parse(fn_name, ctx) {
        Ok(name) => Ok(parse_quote!(self.#name(#(#args),*))),
        Err(err) => Err(err),
    }
}

//TODO: change naming
fn parse_ext_call<T>(
    variable: &str,
    fn_name: &str,
    args: &[Expression],
    ctx: &mut T,
) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let fn_ident = utils::to_snake_case_ident(fn_name);
    let args = parse_many(&args, ctx)?;
    let var_ident = utils::to_snake_case_ident(variable);

    // If in solidity code a reference is a contract may be a field,
    // but in odra we store only an address, so a ref must be built
    // from the address.
    // If a ref was created from an address, the function may be called
    // straight away.
    match ctx.type_from_string(variable) {
        Some(ItemType::Storage(Var {
            ty: Type::Custom(ty),
            ..
        })) => {
            let ty = ctx.type_from_string(&ty);
            if let Some(ItemType::Contract(class_name)) | Some(ItemType::Interface(class_name)) = ty
            {
                ext_call(variable, &class_name, fn_ident, args, ctx)
            } else {
                Ok(parse_quote!(#var_ident.#fn_ident(#(#args),*)))
            }
        }
        Some(ItemType::Contract(class_name)) | Some(ItemType::Interface(class_name)) => {
            ext_call(variable, &class_name, fn_ident, args, ctx)
        }
        Some(ItemType::Storage(Var {
            ty: Type::Array(ty),
            ..
        }))
        | Some(ItemType::Local(Var {
            ty: Type::Array(ty),
            ..
        })) => array::fn_call(variable, fn_ident, args, ctx),
        _ => Ok(parse_quote!(#var_ident.#fn_ident(#(#args),*))),
    }
}

fn ext_call<T>(
    addr_var: &str,
    class_name: &str,
    fn_ident: Ident,
    args: Vec<syn::Expr>,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    ctx.register_external_call(&class_name);
    let ref_ident = format_ident!("{}Ref", class_name);
    let addr = args.get(0);

    let addr = primitives::get_var_or_parse(&Expression::from(addr_var), ctx)?;
    Ok(parse_quote!(
        #ref_ident::at(&odra::UnwrapOrRevert::unwrap_or_revert(#addr)).#fn_ident(#(#args),*)
    ))
}

fn parse_super_call<T>(
    fn_name: &str,
    args: &[Expression],
    ctx: &mut T,
) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let fn_name = utils::to_prefixed_snake_case_ident("super_", fn_name);
    let args = parse_many(&args, ctx)?;
    Ok(parse_quote!(self.#fn_name(#(#args),*)))
}

fn parse_member_access<T>(
    member_name: &str,
    expr: &Expression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    match ctx.type_from_expression(expr) {
        Some(ItemType::Enum(ty)) => {
            let ty = format_ident!("{}", ty);
            let member: syn::Member = format_ident!("{}", member_name).into();
            Ok(parse_quote!(#ty::#member))
        }
        Some(ItemType::Storage(Var {
            ty: Type::Array(_), ..
        })) => array::read_property(member_name, expr, ctx),
        _ => {
            let base_expr: syn::Expr = parse(expr, ctx)?;
            let member: syn::Member = format_ident!("{}", member_name).into();
            Ok(parse_quote!(#base_expr.#member))
        }
    }
}
