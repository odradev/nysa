use syn::parse_quote;

use crate::{
    error::ParserResult,
    model::ir::{Expression, LogicalOp, Message},
    parser::{
        common::{expr::parse, ContractErrorParser, StatementParserContext},
        soroban::code,
        syn_utils,
    },
    ParserError, SorobanParser,
};

impl ContractErrorParser for SorobanParser {
    fn revert_with_str<T: StatementParserContext>(
        condition: Option<&Expression>,
        message: &str,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let error = parse_quote!(panic!(#message));
        match condition {
            Some(condition) => {
                let condition = parse::<_, Self>(condition, ctx)?;
                Ok(syn_utils::if_not(condition, error))
            }
            None => Ok(error),
        }
    }

    fn revert_with_err(error_name: &str) -> syn::Expr {
        todo!()
    }

    fn revert<T: StatementParserContext>(
        condition: Option<&Expression>,
        error: &Expression,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        match condition {
            Some(Expression::LogicalOp(
                box Expression::Message(Message::Sender),
                _,
                LogicalOp::Eq,
            )) => Ok(code::expr::auth_caller()),
            Some(Expression::LogicalOp(
                _,
                box Expression::Message(Message::Sender),
                LogicalOp::Eq,
            )) => Ok(code::expr::auth_caller()),
            _ => match error {
                Expression::StringLiteral(message) => {
                    Self::revert_with_str(condition, message, ctx)
                }
                _ => Err(ParserError::UnexpectedExpression(
                    "Error should be Expression::StringLiteral",
                    error.clone(),
                )),
            },
        }
    }
}
