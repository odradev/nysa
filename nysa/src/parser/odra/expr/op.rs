use syn::{parse_quote, BinOp};

use crate::{
    model::ir::{Expression, UnaryOp},
    parser::{
        context::{
            ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
        },
        odra::expr::math,
    },
    ParserError,
};

pub(crate) fn bin_op<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &Expression,
    right: &Expression,
    op: BinOp,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = math::eval(left, ctx)?;
    let right = math::eval(right, ctx)?;

    Ok(parse_quote!(#left #op #right))
}

pub(crate) fn unary_op<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    expr: &Expression,
    op: &UnaryOp,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let expr = math::eval(expr, ctx)?;

    Ok(match op {
        UnaryOp::Not => parse_quote!(!#expr),
        UnaryOp::Plus => parse_quote!(#expr),
        UnaryOp::Minus => parse_quote!(-#expr),
    })
}

