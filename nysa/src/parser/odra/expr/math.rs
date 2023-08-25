use super::{num, primitives};
use crate::model::ir::{NysaExpression, NysaVar};
use syn::parse_quote;

pub(crate) fn add(
    left: &NysaExpression,
    right: &NysaExpression,
    storage_fields: &[NysaVar],
) -> Result<syn::Expr, &'static str> {
    let left = match left {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::read_variable_or_parse(left, storage_fields)?,
    };
    let right = match right {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::read_variable_or_parse(right, storage_fields)?,
    };
    Ok(parse_quote!(#left + #right))
}

pub(crate) fn sub(
    left: &NysaExpression,
    right: &NysaExpression,
    storage_fields: &[NysaVar],
) -> Result<syn::Expr, &'static str> {
    let left = match left {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::read_variable_or_parse(left, storage_fields)?,
    };
    let right = match right {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::read_variable_or_parse(right, storage_fields)?,
    };
    Ok(parse_quote!(#left - #right))
}
