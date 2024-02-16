use crate::error::ParserResult;
use crate::model::ir::Expression;
use crate::parser::common::StatementParserContext;
use crate::parser::syn_utils;
use crate::Parser;

/// Builds a syn::Stmt return statement.
///
/// ## Solidity example
/// `return x + 10;`
///
/// ## Arguments
/// * expr - returned value expression
/// * ctx - parser context
pub(super) fn ret<T, P>(expr: &Expression, ctx: &mut T) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    let ret_ty = ctx.current_fn().ret_ty();
    // to find out the type of returned value, parsing `expr` need more context
    // in it needs to know the type from the function signature.
    ctx.push_contextual_expr(ret_ty);
    let ret = super::expr::var::parse_or_default::<_, P>(expr, ctx)?;
    ctx.drop_contextual_expr();
    Ok(syn_utils::ret(Some(ret)))
}

/// Builds an empty (returning Unit type) syn::Stmt return statement .
///
/// ## Solidity example
/// `return;`
pub(super) fn ret_unit() -> ParserResult<syn::Stmt> {
    Ok(syn_utils::ret(None))
}
