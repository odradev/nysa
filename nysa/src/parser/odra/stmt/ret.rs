use syn::parse_quote;

use crate::model::ir::Expression;
use crate::parser::context::{
    ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
};
use crate::parser::odra::expr;
use crate::ParserError;

pub(super) fn ret<T>(expr: &Expression, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let ret = expr::primitives::get_var_or_parse(expr, ctx)?;
    Ok(parse_quote!(return #ret;))
}

pub(super) fn ret_unit() -> Result<syn::Stmt, ParserError> {
    Ok(parse_quote!(return;))
}
