use syn::{parse_quote, BinOp};

use crate::{
    model::ir::NysaExpression,
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
    left_var: &Option<String>,
    right_var: &Option<String>,
    left: &NysaExpression,
    right: &NysaExpression,
    op: BinOp,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let left = math::eval(left, ctx)?;
    let right = math::eval(right, ctx)?;

    Ok(parse_quote!(#left #op #right))
}
