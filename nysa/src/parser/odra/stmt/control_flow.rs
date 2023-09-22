use syn::parse_quote;

use crate::model::ir::{Expression, Stmt};
use crate::parser::context::{
    ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
};
use crate::parser::odra::expr;
use crate::ParserError;

pub(super) fn if_stmt<T>(
    assertion: &Expression,
    body: &Stmt,
    ctx: &mut T,
) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let assertion = expr::parse(assertion, ctx)?;
    let if_body = super::parse_statement(body, true, ctx)?;
    let result: syn::Stmt = parse_quote!(if #assertion #if_body);
    Ok(result)
}

pub(super) fn if_else_stmt<T>(
    assertion: &Expression,
    if_body: &Stmt,
    else_body: &Stmt,
    ctx: &mut T,
) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let if_expr = if_stmt(assertion, if_body, ctx)?;
    let else_body = super::parse_statement(else_body, true, ctx)?;
    Ok(parse_quote!(#if_expr else #else_body))
}

pub(super) fn while_loop<T>(
    assertion: &Expression,
    block: &Stmt,
    ctx: &mut T,
) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let assertion = expr::parse(assertion, ctx)?;
    let block = super::parse_statement(block, false, ctx)?;

    if matches!(block, syn::Stmt::Expr(syn::Expr::Block(_))) {
        Ok(parse_quote!(while #assertion #block))
    } else {
        Err(ParserError::InvalidStatement("syn::Block expected"))
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
