use super::num;
use crate::{
    model::ir::NysaExpression,
    parser::{
        context::{
            ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
        },
        odra::expr::primitives,
    },
    utils, ParserError,
};
use syn::parse_quote;

pub(crate) fn add<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &NysaExpression,
    right: &NysaExpression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = num::to_generic_int_expr_or_parse(left, ctx)?;
    let right = num::to_generic_int_expr_or_parse(right, ctx)?;
    Ok(parse_quote!( (#left + #right) ))
}

pub(crate) fn sub<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &NysaExpression,
    right: &NysaExpression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = num::to_generic_int_expr_or_parse(left, ctx)?;
    let right = num::to_generic_int_expr_or_parse(right, ctx)?;
    Ok(parse_quote!( (#left - #right) ))
}

pub(crate) fn div<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &NysaExpression,
    right: &NysaExpression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = match left {
        NysaExpression::Assign {
            left: box NysaExpression::Variable { name },
            right,
        } => {
            let ident = utils::to_snake_case_ident(name);
            let expr = num::to_generic_int_expr_or_parse(left, ctx)?;
            parse_quote!({ #expr; #ident})
        }
        _ => num::to_generic_int_expr_or_parse(left, ctx)?,
    };
    let right = num::to_generic_int_expr_or_parse(right, ctx)?;
    Ok(parse_quote!( (#left / #right) ))
}

pub(crate) fn mul<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &NysaExpression,
    right: &NysaExpression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = num::to_generic_int_expr_or_parse(left, ctx)?;
    let right = num::to_generic_int_expr_or_parse(right, ctx)?;
    Ok(parse_quote!( (#left * #right) ))
}

pub(crate) fn eval<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    expr: &NysaExpression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let eval_or_parse = |e: &NysaExpression, ctx: &mut T| {
        if let NysaExpression::Variable { name } = e {
            let expr = primitives::get_var_or_parse(expr, ctx)?;
            let ident = utils::to_snake_case_ident(name);
            Ok(parse_quote!({#expr; #ident}))
        } else {
            primitives::get_var_or_parse(expr, ctx)
        }
    };
    match expr {
        NysaExpression::Assign { left, right } => eval_or_parse(left, ctx),
        NysaExpression::AssignSubtract { left, right } => eval_or_parse(left, ctx),
        NysaExpression::AssignAdd { left, right } => eval_or_parse(left, ctx),
        NysaExpression::AssignDefault { left } => eval_or_parse(left, ctx),
        NysaExpression::Increment { expr } => eval_or_parse(expr, ctx),
        NysaExpression::Decrement { expr } => eval_or_parse(expr, ctx),
        _ => primitives::get_var_or_parse(expr, ctx),
    }
}
