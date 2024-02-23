use crate::error::ParserResult;
use crate::model::ir::{Expression, Stmt};
use crate::parser::common::stmt::StatementParserContext;
use crate::{parser, Parser, ParserError};

pub(super) fn if_stmt<T, P>(
    assertion: &Expression,
    body: &Stmt,
    ctx: &mut T,
) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    let assertion = parser::common::expr::parse::<_, P>(assertion, ctx)?;
    let if_body = super::parse_statement::<_, P>(body, true, ctx)?;
    Ok(parser::syn_utils::if_stmt(assertion, if_body))
}

pub(super) fn if_else_stmt<T, P>(
    assertion: &Expression,
    if_body: &Stmt,
    else_body: &Stmt,
    ctx: &mut T,
) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    let assertion = parser::common::expr::parse::<_, P>(assertion, ctx)?;
    let if_body = super::parse_statement::<_, P>(if_body, true, ctx)?;
    let else_body = super::parse_statement::<_, P>(else_body, true, ctx)?;
    Ok(parser::syn_utils::if_else_stmt(
        assertion, if_body, else_body,
    ))
}

pub(super) fn while_loop<T, P>(
    assertion: &Expression,
    block: &Stmt,
    ctx: &mut T,
) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    let assertion = parser::common::expr::parse::<_, P>(assertion, ctx)?;
    let block = super::parse_statement::<_, P>(block, false, ctx)?;

    match block {
        syn::Stmt::Expr(syn::Expr::Block(_)) => Ok(parser::syn_utils::while_loop(assertion, block)),
        _ => Err(ParserError::InvalidStatement("syn::Block expected")),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::test_utils::{
        assert_tokens_eq, parse_with_empty_context, unsafe_parse_with_empty_context,
    };
    use quote::quote;

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
