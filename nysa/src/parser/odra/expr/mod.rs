use quote::format_ident;
use quote::quote;
use syn::parse_quote;

use crate::model::ir::NysaExpression;
use crate::utils;
use crate::utils::to_snake_case_ident;
use crate::ParserError;

use super::context::Context;
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
        NysaExpression::StringLiteral(string) => Ok(parse_quote!(String::from(#string))),
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
        NysaExpression::Func { name, args } => {
            let args = parse_many(&args, ctx)?;

            if let Some(class_name) = ctx.class(name) {
                ctx.register_external_call(&class_name);

                let ref_ident = format_ident!("{}Ref", class_name);
                let addr = args.get(0);
                // let ident = format_ident!("{}", param_name);
                return Ok(
                    parse_quote!(#ref_ident::at(&odra::UnwrapOrRevert::unwrap_or_revert(#addr))),
                );
            }
            match parse(name, ctx) {
                Ok(name) => Ok(parse_quote!(self.#name(#(#args),*))),
                Err(err) => Err(err),
            }
        }
        NysaExpression::SuperCall { name, args } => {
            let name = utils::to_prefixed_snake_case_ident("super_", name);
            let args = parse_many(&args, ctx)?;
            Ok(parse_quote!(self.#name(#(#args),*)))
        }
        NysaExpression::ExternalCall {
            variable,
            fn_name,
            args,
        } => {
            let var = utils::to_snake_case_ident(variable);
            let fn_name = utils::to_snake_case_ident(fn_name);
            let args = parse_many(&args, ctx)?;
            Ok(parse_quote!(#var.#fn_name(#(#args),*)))
        }
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
