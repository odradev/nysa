use super::num;
use crate::model::ir::MathOp;
use crate::parser::context::ErrorInfo;
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

/// Parses a math operation expression to `syn::Expr`.
pub(crate) fn parse_op<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
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
    let left_expr = eval_in_context(left, right, ctx)?;
    let right_expr = eval_in_context(right, left, ctx)?;
    Ok(parse_quote!( (#left_expr #op #right_expr) ))
}

/// Parses an expression to `syn::Expr` that returns a value.
///
/// In Solidity is allowed to write `x = y + z < 3`, which assigns the sum of y + z to x and compares it to 3.
/// Such a syntax would be invalid in Rust, so the left-hand side expression be evaluated.
///
/// The example expression should be parsed to `{x = y + z; x}`.
/// The assignment happens inside a block which returns the updated value.
///
/// Examples
/// x = 1
/// x += 1
/// y -= x
/// x *= 2
/// z /= 1
pub(crate) fn eval<
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
        Expression::Assign(left, _) => eval_or_parse(left, ctx),
        Expression::AssignAnd(left, _, _) => eval_or_parse(left, ctx),
        Expression::Increment(expr) => eval_or_parse(expr, ctx),
        Expression::Decrement(expr) => eval_or_parse(expr, ctx),
        Expression::NumberLiteral(values) => num::to_typed_int_expr(values, ctx),
        _ => primitives::get_var_or_parse(expr, ctx),
    }
}

/// Parses an expression in the context of another expression.
///
/// Sometimes an expression to be parsed correctly, cannot be analyzed in isolation.
/// Let's take a look at a comparison expression: y - x > 123.
/// If we analyze both sides: `y - x` and `123` separately, we'd end up with incorrect code.
/// Analyzing the `123` expression the exact numeric type is unknown but required by the `>` operator.
/// From the context `ctx` we can find out the type of `y - x` expression, and the same type
/// apply to the `123` literal.
///
/// In this example let's assume `y` and `x` are of `nysa_type::256` then, the subtraction result
/// is the same type and finally the `123` literal should be parsed to`nysa_types::U256::from_limbs_slice(&[123u64])`
pub(crate) fn eval_in_context<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
>(
    expr: &Expression,
    context_expr: &Expression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    ctx.push_contextual_expr(context_expr.clone());
    let expr = eval(expr, ctx)?;
    ctx.drop_contextual_expr();
    Ok(expr)
}

fn pow<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
>(
    left: &Expression,
    right: &Expression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left_expr = eval_in_context(left, right, ctx)?;
    let right_expr = eval_in_context(right, left, ctx)?;
    Ok(parse_quote!(#left_expr.pow(#right_expr)))
}
