use syn::parse_quote;

use crate::error::ParserResult;
use crate::model::ir::Stmt;
use crate::parser::common::stmt::StatementParserContext;
use crate::{Parser, ParserError};

/// Parses a block of statements and returns a `syn::Stmt`.
///
/// # Arguments
///
/// * `stmts` - A slice of `Stmt` representing the statements in the block.
/// * `ctx` - A mutable reference to a context object implementing various traits.
///
/// # Returns
///
/// A `ParserResult` containing the parsed `syn::Stmt`.
pub(super) fn block<T, P>(stmts: &[Stmt], ctx: &mut T) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    let stmts = stmts
        .iter()
        .map(|stmt| super::parse_statement::<T, P>(stmt, true, ctx))
        .collect::<ParserResult<Vec<syn::Stmt>>>()?;
    Ok(parse_quote!({ #(#stmts)* }))
}

/// Parses a block of statements and returns a `syn::Stmt` returning a value that
/// the last statement returns.
///
/// # Arguments
///
/// * `stmts` - A slice of `Stmt` representing the statements in the block.
/// * `ctx` - A mutable reference to a context object implementing various traits.
///
/// # Returns
///
/// A `ParserResult` containing the parsed `syn::Stmt`.
pub(super) fn ret_block<T, P>(stmts: &[Stmt], ctx: &mut T) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    let last_stmt = stmts
        .last()
        .map(|stmt| super::parse_statement::<T, P>(stmt, false, ctx))
        .ok_or(ParserError::InvalidStatement(
            "A statement expected but not found",
        ))??;
    let stmts = stmts
        .iter()
        .take(stmts.len() - 1)
        .map(|stmt| super::parse_statement::<T, P>(stmt, true, ctx))
        .collect::<ParserResult<Vec<syn::Stmt>>>()?;

    Ok(parse_quote!({
        #(#stmts)*
        #last_stmt
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ir::{Expression, Type};
    use crate::parser::test_utils::assert_tokens_eq;
    use crate::parser::test_utils::{parse_with_empty_context, unsafe_parse_with_empty_context};
    use quote::quote;

    #[test]
    fn invalid_block() {
        let block = Stmt::Block(vec![
            Stmt::Fail,
            Stmt::VarDefinition("x".to_string(), Type::Bool, Expression::BoolLiteral(true)),
        ]);

        assert!(parse_with_empty_context(block).is_err());
    }

    #[test]
    fn valid_block() {
        let block = Stmt::Block(vec![
            Stmt::VarDefinition("x".to_string(), Type::Bool, Expression::BoolLiteral(true)),
            Stmt::Expression(Expression::Assign(
                Box::new(Expression::Variable("x".to_string())),
                Some(Box::new(Expression::BoolLiteral(false))),
            )),
        ]);

        assert_tokens_eq(
            unsafe_parse_with_empty_context(block),
            quote!({
                let mut x = true;
                x = false;
            }),
        )
    }
}
