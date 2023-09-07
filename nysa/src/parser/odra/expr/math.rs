use super::{num, primitives};
use crate::{
    model::ir::NysaExpression,
    parser::context::{
        ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
    },
    ParserError,
};
use syn::parse_quote;

pub(crate) fn add<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &NysaExpression,
    right: &NysaExpression,
    t: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = match left {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::get_var_or_parse(left, t),
    }?;
    let right = match right {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::get_var_or_parse(right, t),
    }?;
    Ok(parse_quote!(#left + #right))
}

pub(crate) fn sub<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &NysaExpression,
    right: &NysaExpression,
    t: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = match left {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::get_var_or_parse(left, t),
    }?;
    let right = match right {
        NysaExpression::NumberLiteral { ty, value } => num::to_generic_int_expr(ty, value),
        _ => primitives::get_var_or_parse(right, t),
    }?;
    Ok(parse_quote!(#left - #right))
}
