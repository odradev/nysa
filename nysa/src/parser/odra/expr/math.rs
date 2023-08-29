use super::{num, primitives};
use crate::{model::ir::NysaExpression, parser::odra::context::Context, ParserError};
use syn::parse_quote;

pub(crate) fn add(
    left: &NysaExpression,
    right: &NysaExpression,
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    let left = match left {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::read_variable_or_parse(left, ctx),
    }?;
    let right = match right {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::read_variable_or_parse(right, ctx),
    }?;
    Ok(parse_quote!(#left + #right))
}

pub(crate) fn sub(
    left: &NysaExpression,
    right: &NysaExpression,
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    let left = match left {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::read_variable_or_parse(left, ctx),
    }?;
    let right = match right {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::read_variable_or_parse(right, ctx),
    }?;
    Ok(parse_quote!(#left - #right))
}
