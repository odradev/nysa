use crate::model::ir::Expression;
use crate::parser::context::{
    ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
    TypeInfo,
};
use crate::parser::odra::expr;
use crate::ParserError;
use syn::parse_quote;

pub(crate) fn revert_with_msg(error_msg: &str) -> Result<syn::Stmt, ParserError> {
    let expr = expr::error::revert_with_err(error_msg)?;
    Ok(parse_quote!(#expr;))
}

pub(crate) fn revert<T>(msg: &Option<Expression>, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    if let Some(error) = msg {
        let expr = expr::error::revert(None, error, ctx)?;
        Ok(parse_quote!(#expr;))
    } else {
        let expr = expr::error::revert_with_str(None, "", ctx)?;
        Ok(parse_quote!(#expr;))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ir::Stmt;
    use crate::parser::odra::stmt::test::{
        parse_with_empty_context, unsafe_parse_with_empty_context,
    };
    use crate::parser::odra::test::assert_tokens_eq;
    use quote::quote;

    #[test]
    fn revert_with_no_msg() {
        let stmt = Stmt::Revert(None);

        assert_tokens_eq(
            unsafe_parse_with_empty_context(stmt),
            quote!(odra::contract_env::revert(odra::types::ExecutionError::new(1u16, ""));),
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
                odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "An error occurred"));
            ),
        );
    }

    #[test]
    fn revert_with_error() {
        let stmt = Stmt::RevertWithError("MyError".to_string());

        assert_tokens_eq(
            unsafe_parse_with_empty_context(stmt),
            quote!(odra::contract_env::revert(Error::MyError);),
        )
    }

    #[test]
    fn invalid_revert_stmt() {
        let stmt = Stmt::Revert(Some(Expression::Placeholder));
        let result = parse_with_empty_context(stmt);

        assert!(result.is_err());
    }
}
