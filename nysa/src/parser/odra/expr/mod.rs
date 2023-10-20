use crate::model::ir::{Expression, Op, Stmt, TupleItem, Type, Var};
use crate::parser::context::{
    ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, ItemType, StorageInfo, TypeInfo,
};
use crate::utils;
use crate::ParserError;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_quote, punctuated::Punctuated, Token};

use super::stmt;
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
        Expression::Require(condition, error) => error::revert(Some(condition), error, ctx),
        Expression::Placeholder => Err(ParserError::InvalidExpression),
        Expression::ZeroAddress => Ok(parse_quote!(None)),
        Expression::Message(msg) => msg.try_into(),
        Expression::Collection(name, keys) => primitives::parse_collection(name, keys, None, ctx),
        Expression::Variable(name) => parse_variable(name, ctx),
        Expression::Assign(left, right) => {
            primitives::assign(left, right.as_deref(), None::<&Op>, ctx)
        }
        Expression::StringLiteral(string) => {
            Ok(parse_quote!(odra::prelude::string::String::from(#string)))
        }
        Expression::LogicalOp(left, right, op) => op::bin_op(left, right, op, ctx),
        Expression::MathOp(left, right, op) => math::parse_op(left, right, op, ctx),
        Expression::AssignAnd(left, right, op) => {
            primitives::assign(left, Some(right), Some(op), ctx)
        }
        Expression::Increment(expr) => {
            let expr = parse(expr, ctx)?;
            Ok(parse_quote!(#expr += nysa_types::Unsigned::ONE))
        }
        Expression::Decrement(expr) => {
            let expr = parse(expr, ctx)?;
            Ok(parse_quote!(#expr -= nysa_types::Unsigned::ONE))
        }
        Expression::MemberAccess(name, expr) => parse_member_access(name, expr, ctx),
        Expression::NumberLiteral(limbs) => num::to_typed_int_expr(limbs, ctx),
        Expression::Func(name, args) => parse_func(name, args, ctx),
        Expression::SuperCall(name, args) => parse_super_call(name, args, ctx),
        Expression::ExternalCall(var, fn_name, args) => parse_ext_call(var, fn_name, args, ctx),
        Expression::TypeInfo(ty, property) => parse_type_info(ty, property, ctx),
        Expression::Type(ty) => {
            let ty = ty::parse_type_from_ty(ty, ctx)?;
            Ok(parse_quote!(#ty))
        }
        Expression::BoolLiteral(b) => Ok(parse_quote!(#b)),
        Expression::Not(expr) => {
            let expr = primitives::get_var_or_parse(expr, ctx)?;
            Ok(parse_quote!(!(#expr)))
        }
        Expression::BytesLiteral(bytes) => parse_bytes_lit(bytes),
        Expression::ArrayLiteral(values) => parse_array_lit(values, ctx),
        Expression::Initializer(expr) => parse_init(expr, ctx),
        Expression::Statement(s) => parse_statement(s, ctx),
        Expression::BitwiseOp(left, right, op) => op::bin_op(left, right, op, ctx),
        Expression::UnaryOp(expr, op) => op::unary_op(expr, op, ctx),
        Expression::Tuple(items) => parse_tuple(items, ctx),
        #[cfg(test)]
        Expression::Fail => Err(ParserError::InvalidExpression),
    }
}

pub fn parse_expr<T>(
    expr: &Expression,
    is_semi: bool,
    ctx: &mut T,
) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let expr = parse(expr, ctx)?;
    if !is_semi {
        Ok(syn::Stmt::Expr(expr))
    } else {
        Ok(syn::Stmt::Semi(expr, Default::default()))
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
    if let Expression::Type(ty) = fn_name {
        // cast expression
        let ty = ty::parse_type_from_ty(ty, ctx)?;
        let arg = primitives::get_var_or_parse(args.first().unwrap(), ctx)?;
        return Ok(parse_quote!(#ty::from(*#arg)));
    }

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
    // but in Odra we store only an address, so a ref must be built
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
        Some(ItemType::Library(class_name)) => {
            lib_call(variable, fn_ident, args)
            // ext_call(variable, &class_name, fn_ident, args, ctx)
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

fn lib_call(
    lib_name: &str,
    fn_ident: Ident,
    args: Vec<syn::Expr>,
) -> Result<syn::Expr, ParserError> {
    let ident = format_ident!("{}", lib_name);
    Ok(parse_quote!(#ident::#fn_ident(#(#args),*)))
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

fn parse_variable<T: TypeInfo>(name: &str, ctx: &mut T) -> Result<syn::Expr, ParserError> {
    let ident = utils::to_snake_case_ident(name);
    let self_ty = ctx
        .type_from_string(name)
        .filter(|i| matches!(i, ItemType::Storage(_)))
        .map(|_| quote!(self.));
    Ok(parse_quote!(#self_ty #ident))
}

fn parse_array_lit<T>(values: &[Expression], ctx: &mut T) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let arr = values
        .iter()
        .map(|e| parse(e, ctx))
        .map(|e| match e {
            Ok(r) => Ok(quote!(#r)),
            Err(e) => Err(e),
        })
        .collect::<Result<Punctuated<TokenStream, Token![,]>, ParserError>>()?;
    Ok(parse_quote!(odra::prelude::vec![#arr]))
}

pub fn parse_bytes_lit(bytes: &[u8]) -> Result<syn::Expr, ParserError> {
    let arr = bytes
        .iter()
        .map(|v| quote::quote!(#v))
        .collect::<Punctuated<TokenStream, Token![,]>>();
    let size = bytes.len();
    Ok(parse_quote!(nysa_types::FixedBytes([#arr])))
}

fn parse_statement<T>(stmt: &Stmt, ctx: &mut T) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    stmt::parse_statement(stmt, false, ctx).map(|stmt| match stmt {
        syn::Stmt::Expr(e) => Ok(e),
        _ => Err(ParserError::InvalidExpression),
    })?
}

fn parse_type_info<T>(
    ty: &Expression,
    property: &str,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let ty = parse(ty, ctx)?;
    let property = match property {
        "max" => format_ident!("MAX"),
        "min" => format_ident!("MIN"),
        p => return Err(ParserError::UnknownProperty(p.to_string())),
    };
    Ok(parse_quote!(#ty::#property))
}

fn parse_init<T>(expr: &Expression, ctx: &mut T) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    if let Expression::Func(box Expression::Type(Type::Array(_)), args) = expr {
        let args = parse_many(&args, ctx)?;
        return Ok(parse_quote!(odra::prelude::vec::Vec::with_capacity(#(#args),*)));
    }
    dbg!(expr);
    todo!()
}

/// Parses [TupleItem] into an expression `(e1, e2, .., eN)`
fn parse_tuple<T>(items: &[TupleItem], ctx: &mut T) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let items = items
        .iter()
        .map(|i| match i {
            TupleItem::Expr(i) => parse(i, ctx),
            _ => Err(ParserError::InvalidExpression),
        })
        .collect::<Result<Vec<syn::Expr>, ParserError>>()?;

    Ok(parse_quote!( ( #(#items),* ) ))
}
