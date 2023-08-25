use syn::{BinOp, parse_quote};

use crate::model::ir::{NysaExpression, NysaVar};
use super::primitives;

pub(crate) fn bin_op(
    left: &NysaExpression,
    right: &NysaExpression,
    op: BinOp,
    storage_fields: &[NysaVar],
) -> Result<syn::Expr, &'static str> {
    let left = primitives::read_variable_or_parse(left, storage_fields)?;
    let right = primitives::read_variable_or_parse(right, storage_fields)?;
    Ok(parse_quote!(#left #op #right))
}
