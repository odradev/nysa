use super::num;
use crate::model::ir::MathOp;
use crate::{
    model::ir::Expression,
    parser::{
        context::{
            ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
        },
        odra::expr::primitives,
    },
    utils, ParserError,
};
use syn::parse_quote;

pub(crate) fn parse_op<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &Expression,
    right: &Expression,
    op: &MathOp,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    if *op == MathOp::Pow {
        return pow(left, right, ctx);
    }
    let op: syn::BinOp = op.into();
    let (left, right) = parse_expression(left, right, ctx)?;
    Ok(parse_quote!( (#left #op #right) ))
}

pub(crate) fn eval<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    expr: &Expression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let eval_or_parse = |e: &Expression, ctx: &mut T| {
        if let Expression::Variable(name) = e {
            let expr = primitives::get_var_or_parse(expr, ctx)?;
            let ident = utils::to_snake_case_ident(name);
            Ok(parse_quote!({#expr; #ident}))
        } else {
            primitives::get_var_or_parse(expr, ctx)
        }
    };
    match expr {
        Expression::Assign(left, right) => eval_or_parse(left, ctx),
        Expression::AssignAnd(left, right, op) => eval_or_parse(left, ctx),
        Expression::Increment(expr) => eval_or_parse(expr, ctx),
        Expression::Decrement(expr) => eval_or_parse(expr, ctx),
        _ => primitives::get_var_or_parse(expr, ctx),
    }
}

fn pow<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &Expression,
    right: &Expression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = primitives::get_var_or_parse(left, ctx)?;
    let right = primitives::get_var_or_parse(right, ctx)?;
    Ok(parse_quote!(#left.pow(#right)))
}

fn parse_expression<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &Expression,
    right: &Expression,
    ctx: &mut T,
) -> Result<(syn::Expr, syn::Expr), ParserError> {
    let left = match left {
        Expression::Assign(box Expression::Variable(name), _) => {
            let ident = utils::to_snake_case_ident(name);
            let expr = num::to_generic_int_expr_or_parse(left, ctx)?;
            parse_quote!({ #expr; #ident})
        }
        _ => num::to_generic_int_expr_or_parse(left, ctx)?,
    };
    let right = num::to_generic_int_expr_or_parse(right, ctx)?;
    Ok((left, right))
}
