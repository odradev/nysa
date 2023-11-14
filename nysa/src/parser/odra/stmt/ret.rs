use syn::parse_quote;

use crate::model::ir::Expression;
use crate::parser::context::{
    ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
    TypeInfo,
};
use crate::parser::odra::expr;
use crate::ParserError;

/// Builds a syn::Stmt return statement.
/// 
/// ## Solidity example
/// `return x + 10;`
/// 
/// ## Arguments
/// * expr - returned value expression
/// * ctx - parser context
pub(super) fn ret<T>(expr: &Expression, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let ret_expr = ctx
        .current_fn()
        .ret_ty();

    // to find out the type of returned value, parsing `expr` need more context
    // in it needs to know the type from the function signature.
    ctx.push_contextual_expr(ret_expr);
    let ret = expr::primitives::get_var_or_parse(expr, ctx)?;
    ctx.drop_contextual_expr();
    Ok(parse_quote!(return #ret;))
}

/// Builds an empty (returning Unit type) syn::Stmt return statement .
/// 
/// ## Solidity example
/// `return;`
pub(super) fn ret_unit() -> Result<syn::Stmt, ParserError> {
    Ok(parse_quote!(return;))
}
