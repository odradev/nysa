use super::{num, var};
use crate::error::ParserResult;
use crate::model::ir::MathOp;
use crate::parser::common::{ExpressionParser, StatementParserContext};
use crate::parser::syn_utils::in_context;
use crate::Parser;
use crate::{model::ir::Expression, utils};
use ::syn::parse_quote;

/// Parses a binary mathematical operation and returns a `syn::Expr` representing the operation.
///
/// This function takes the left and right expressions, the mathematical operation, and the context.
/// If the operation is a power operation, it calls the `pow` function.
/// Otherwise, it converts the operation into a `syn::BinOp` and evaluates the left and right expressions in the context.
pub(crate) fn parse_op<T: StatementParserContext, P: Parser>(
    left: &Expression,
    right: &Expression,
    op: &MathOp,
    ctx: &mut T,
) -> ParserResult<::syn::Expr> {
    // if *op == MathOp::Pow {
    //     return pow::<_, P>(left, right, ctx);
    // }
    // let op: syn::BinOp = op.into();
    // let left_expr = eval_in_context::<_, P>(left, right, ctx)?;
    // let right_expr = eval_in_context::<_, P>(right, left, ctx)?;

    <P::ExpressionParser as ExpressionParser>::parse_math_op(left, right, op, ctx)
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
pub(crate) fn eval<T: StatementParserContext, P: Parser>(
    expr: &Expression,
    ctx: &mut T,
) -> ParserResult<::syn::Expr> {
    let eval_or_parse = |e: &Expression, ctx: &mut T| {
        if let Expression::Variable(name) = e {
            let expr = var::parse_or_default::<_, P>(expr, ctx)?;
            let ident = utils::to_snake_case_ident(name);
            Ok(parse_quote!({#expr; #ident}))
        } else {
            var::parse_or_default::<_, P>(expr, ctx)
        }
    };

    match expr {
        Expression::Assign(left, _) => eval_or_parse(left, ctx),
        Expression::AssignAnd(left, _, _) => eval_or_parse(left, ctx),
        Expression::Increment(expr) => eval_or_parse(expr, ctx),
        Expression::Decrement(expr) => eval_or_parse(expr, ctx),
        Expression::NumberLiteral(values) => {
            num::to_typed_int_expr::<_, P::ExpressionParser>(values, ctx)
        }
        _ => var::parse_or_default::<_, P>(expr, ctx),
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
pub(crate) fn eval_in_context<T: StatementParserContext, P: Parser>(
    expr: &Expression,
    context_expr: &Expression,
    ctx: &mut T,
) -> ParserResult<::syn::Expr> {
    in_context(context_expr, ctx, |ctx| eval::<_, P>(expr, ctx))
}
