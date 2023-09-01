use quote::format_ident;
use quote::quote;
use syn::parse_quote;

use crate::model::ir::NysaExpression;
use crate::model::ir::NysaType;
use crate::parser::context::Context;
use crate::utils;
use crate::utils::to_snake_case_ident;
use crate::ParserError;

use super::ty;
use super::var::AsVariable;

pub(crate) mod error;
mod math;
mod num;
mod op;
pub(crate) mod primitives;

pub fn parse(expression: &NysaExpression, ctx: &mut Context) -> Result<syn::Expr, ParserError> {
    match expression {
        NysaExpression::Require { condition, error } => error::revert(Some(condition), error, ctx),
        NysaExpression::Placeholder => Err(ParserError::EmptyExpression),
        NysaExpression::ZeroAddress => Ok(parse_quote!(None)),
        NysaExpression::Message(msg) => msg.try_into(),
        NysaExpression::Mapping { name, key } => {
            let keys = vec![*key.clone()];
            primitives::parse_mapping(name, &keys, None, ctx)
        }
        NysaExpression::Mapping2 { name, keys } => {
            let keys = vec![keys.0.clone(), keys.1.clone()];
            primitives::parse_mapping(name, &keys, None, ctx)
        }
        NysaExpression::Mapping3 { name, keys } => {
            todo!()
        }
        NysaExpression::Variable { name } => {
            let ident = to_snake_case_ident(&name);
            let self_ty = name.as_var(ctx).is_ok().then(|| quote!(self.));
            Ok(parse_quote!(#self_ty #ident))
        }
        NysaExpression::Assign { left, right } => primitives::assign(left, right, None, ctx),
        NysaExpression::StringLiteral(string) => {
            Ok(parse_quote!(odra::prelude::string::String::from(#string)))
        }
        NysaExpression::LessEqual { left, right } => op::bin_op(left, right, parse_quote!(<=), ctx),
        NysaExpression::MoreEqual { left, right } => op::bin_op(left, right, parse_quote!(>=), ctx),
        NysaExpression::Less { left, right } => op::bin_op(left, right, parse_quote!(<), ctx),
        NysaExpression::More { left, right } => op::bin_op(left, right, parse_quote!(>), ctx),
        NysaExpression::Add { left, right } => math::add(left, right, ctx),
        NysaExpression::Subtract { left, right } => math::sub(left, right, ctx),
        NysaExpression::Equal { left, right } => op::bin_op(left, right, parse_quote!(==), ctx),
        NysaExpression::NotEqual { left, right } => op::bin_op(left, right, parse_quote!(!=), ctx),
        NysaExpression::AssignSubtract { left, right } => {
            let expr = primitives::assign(left, right, Some(parse_quote!(-)), ctx)?;
            Ok(expr)
        }
        NysaExpression::AssignAdd { left, right } => {
            let expr = primitives::assign(left, right, Some(parse_quote!(+)), ctx)?;
            Ok(expr)
        }
        NysaExpression::Increment { expr } => {
            let expr = parse(expr, ctx)?;
            Ok(parse_quote!(#expr += 1))
        }
        NysaExpression::Decrement { expr } => {
            let expr = parse(expr, ctx)?;
            Ok(parse_quote!(#expr -= 1))
        }
        NysaExpression::MemberAccess { expr, name } => {
            let base_expr: syn::Expr = parse(expr, ctx)?;
            let member: syn::Member = format_ident!("{}", name).into();
            Ok(parse_quote!(#base_expr.#member))
        }
        NysaExpression::NumberLiteral { ty, value } => num::to_typed_int_expr(ty, value),
        NysaExpression::Func { name, args } => parse_func(name, args, ctx),
        NysaExpression::SuperCall { name, args } => parse_super_call(name, args, ctx),
        NysaExpression::ExternalCall {
            variable,
            fn_name,
            args,
        } => parse_ext_call(variable, fn_name, args, ctx),
        NysaExpression::TypeInfo { ty, property } => {
            let ty = parse(ty, ctx)?;
            let property = match property.as_str() {
                "max" => format_ident!("MAX"),
                "min" => format_ident!("MIN"),
                p => return Err(ParserError::UnknownProperty(p.to_string())),
            };
            Ok(parse_quote!(#ty::#property))
        }
        NysaExpression::Type { ty } => {
            let ty = ty::parse_plain_type_from_ty(ty)?;
            Ok(parse_quote!(#ty))
        }
        NysaExpression::Power { left, right } => {
            let left = primitives::read_variable_or_parse(left, ctx)?;
            let right = primitives::read_variable_or_parse(right, ctx)?;
            Ok(parse_quote!(#left.pow(#right)))
        }
        NysaExpression::BoolLiteral(b) => Ok(parse_quote!(#b)),
        NysaExpression::Not { expr } => {
            let expr = primitives::read_variable_or_parse(expr, ctx)?;
            Ok(parse_quote!(!(#expr)))
        }
        NysaExpression::UnknownExpr => panic!("Unknown expression"),
    }
}

pub fn parse_many(
    expressions: &[NysaExpression],
    ctx: &mut Context,
) -> Result<Vec<syn::Expr>, ParserError> {
    expressions
        .iter()
        .map(|e| parse(e, ctx))
        .collect::<Result<Vec<syn::Expr>, _>>()
}

fn parse_func(
    fn_name: &NysaExpression,
    args: &[NysaExpression],
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    let args = parse_many(&args, ctx)?;
    // Context allows us to distinct an external contract initialization from a regular function call
    if let Some(class_name) = ctx.class(fn_name) {
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

fn parse_ext_call(
    variable: &str,
    fn_name: &str,
    args: &[NysaExpression],
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    let fn_ident = utils::to_snake_case_ident(fn_name);
    let args = parse_many(&args, ctx)?;
    let var_ident = utils::to_snake_case_ident(variable);

    // If in solidity code a reference is a contract may be a field,
    // but in odra we store only an address, so a ref must be built
    // from the address.
    // If a ref was created from an address, the function may be called
    // straight away.
    match variable.as_var(ctx) {
        Ok(NysaType::Contract(class_name)) => {
            ctx.register_external_call(&class_name);
            let ref_ident = format_ident!("{}Ref", class_name);
            let addr = args.get(0);

            let addr = primitives::read_variable_or_parse(
                &NysaExpression::Variable {
                    name: variable.to_string(),
                },
                ctx,
            )?;
            Ok(parse_quote!(
                #ref_ident::at(&odra::UnwrapOrRevert::unwrap_or_revert(#addr)).#fn_ident(#(#args),*)
            ))
        }
        _ => Ok(parse_quote!(#var_ident.#fn_ident(#(#args),*))),
    }
}

fn parse_super_call(
    fn_name: &str,
    args: &[NysaExpression],
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    let fn_name = utils::to_prefixed_snake_case_ident("super_", fn_name);
    let args = parse_many(&args, ctx)?;
    Ok(parse_quote!(self.#fn_name(#(#args),*)))
}
