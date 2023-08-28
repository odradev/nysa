use syn::{parse_quote, BinOp};

use super::primitives;
use crate::{model::ir::NysaExpression, parser::odra::context::Context};

pub(crate) fn bin_op(
    left: &NysaExpression,
    right: &NysaExpression,
    op: BinOp,
    ctx: &mut Context,
) -> Result<syn::Expr, &'static str> {
    let left = primitives::read_variable_or_parse(left, ctx)?;
    let right = primitives::read_variable_or_parse(right, ctx)?;
    Ok(parse_quote!(#left #op #right))
}
