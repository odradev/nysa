use crate::error::ParserResult;
use crate::model::ir::Expression;
use crate::parser::common::stmt::StatementParserContext;
use crate::parser::common::ContractErrorParser;
use crate::parser::syn_utils::AsStatement;

/// Generates a `revert` statement with an error message.
///
/// # Arguments
///
/// * `error_msg` - The error message to include in the `revert` statement.
///
/// # Returns
///
/// Returns a `ParserResult` containing the generated `revert` statement as a `syn::Stmt`.
pub(crate) fn revert_with_msg<T: ContractErrorParser>(error_msg: &str) -> ParserResult<syn::Stmt> {
    Ok(T::revert_with_err(error_msg).as_statement())
}

/// Generates a `revert` statement with an optional error message.
///
/// # Arguments
///
/// * `msg` - An optional expression representing the error message.
/// * `ctx` - A mutable reference to the context object that provides information about the contract, storage, types, etc.
///
/// # Returns
///
/// Returns a `ParserResult` containing the generated `revert` statement as a `syn::Stmt`.
pub(crate) fn revert<T, P>(msg: &Option<Expression>, ctx: &mut T) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
    P: ContractErrorParser,
{
    match msg {
        Some(msg) => P::revert(None, msg, ctx),
        None => P::revert(None, &Expression::StringLiteral(String::from("")), ctx),
    }
    .map(AsStatement::as_statement)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ir::Stmt;
    use crate::parser::test_utils::{
        parse_with_empty_context, unsafe_parse_with_empty_context, assert_tokens_eq
    };
    use quote::quote;

    #[test]
    fn revert_with_no_msg() {
        let stmt = Stmt::Revert(None);

        assert_tokens_eq(
            unsafe_parse_with_empty_context(stmt),
            quote!(self.env().revert(odra::ExecutionError::User(1u16));),
        );
    }

    #[test]
    fn revert_with_msg() {
        let stmt = Stmt::Revert(Some(Expression::StringLiteral(
            "An error occurred".to_string(),
        )));

        assert_tokens_eq(
            unsafe_parse_with_empty_context(stmt),
            quote!(
                self.env().revert(odra::ExecutionError::User(1u16));
            ),
        );
    }

    #[test]
    fn revert_with_error() {
        let stmt = Stmt::RevertWithError("MyError".to_string());

        assert_tokens_eq(
            unsafe_parse_with_empty_context(stmt),
            quote!(self.env().revert(Error::MyError);),
        )
    }

    #[test]
    fn invalid_revert_stmt() {
        let stmt = Stmt::Revert(Some(Expression::Placeholder));
        let result = parse_with_empty_context(stmt);

        assert!(result.is_err());
    }
}
