use super::syn_utils;
use crate::error::ParserResult;
use crate::model::ir::{Expression, Stmt};
use crate::parser::context::{
    ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
    TypeInfo,
};
use crate::parser::odra::expr;
use crate::ParserError;

pub(super) fn if_stmt<T>(
    assertion: &Expression,
    body: &Stmt,
    ctx: &mut T,
) -> ParserResult<syn::Stmt>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let assertion = expr::parse(assertion, ctx)?;
    let if_body = super::parse_statement(body, true, ctx)?;
    Ok(syn_utils::if_stmt(assertion, if_body))
}

pub(super) fn if_else_stmt<T>(
    assertion: &Expression,
    if_body: &Stmt,
    else_body: &Stmt,
    ctx: &mut T,
) -> ParserResult<syn::Stmt>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let assertion = expr::parse(assertion, ctx)?;
    let if_body = super::parse_statement(if_body, true, ctx)?;
    let else_body = super::parse_statement(else_body, true, ctx)?;
    Ok(syn_utils::if_else_stmt(assertion, if_body, else_body))
}

pub(super) fn while_loop<T>(
    assertion: &Expression,
    block: &Stmt,
    ctx: &mut T,
) -> ParserResult<syn::Stmt>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let assertion = expr::parse(assertion, ctx)?;
    let block = super::parse_statement(block, false, ctx)?;

    match block {
        syn::Stmt::Expr(syn::Expr::Block(_)) => Ok(syn_utils::while_loop(assertion, block)),
        _ => Err(ParserError::InvalidStatement("syn::Block expected")),
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::parser::odra::stmt::test::{
        parse_with_empty_context, unsafe_parse_with_empty_context,
    };
    use crate::parser::odra::test::assert_tokens_eq;

    use super::*;

    #[test]
    fn valid_while_stmt() {
        let stmt = Stmt::While(Expression::BoolLiteral(true), Box::new(Stmt::Block(vec![])));
        assert_tokens_eq(unsafe_parse_with_empty_context(stmt), quote!(while true {}));
    }

    #[test]
    fn invalid_while_stmt() {
        let stmt = Stmt::While(Expression::BoolLiteral(true), Box::new(Stmt::ReturnVoid));

        assert_eq!(
            parse_with_empty_context(stmt),
            Err(ParserError::InvalidStatement("syn::Block expected"))
        );
    }
}
