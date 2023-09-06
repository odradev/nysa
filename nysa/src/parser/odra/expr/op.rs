use syn::{parse_quote, BinOp};

use super::primitives;
use crate::{
    model::ir::NysaExpression,
    parser::context::{
        ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
    },
    ParserError,
};

pub(crate) fn bin_op<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    left: &NysaExpression,
    right: &NysaExpression,
    op: BinOp,
    t: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = primitives::read_variable_or_parse(left, t)?;
    let right = primitives::read_variable_or_parse(right, t)?;
    Ok(parse_quote!(#left #op #right))
}
