use super::syn_utils;
use crate::{
    error::ParserResult,
    model::ir::Expression,
    parser::common::{expr::parse, ContractErrorParser, StatementParserContext},
    utils, OdraParser, ParserError,
};

impl ContractErrorParser for OdraParser {
    fn revert<T: StatementParserContext>(
        condition: Option<&Expression>,
        msg: &Expression,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        match msg {
            Expression::StringLiteral(message) => Self::revert_with_str(condition, message, ctx),
            _ => Err(ParserError::UnexpectedExpression(
                "Error should be Expression::StringLiteral",
                msg.clone(),
            )),
        }
    }

    fn revert_with_str<T: StatementParserContext>(
        condition: Option<&Expression>,
        message: &str,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let error_num = match ctx.get_error(message) {
            Some(value) => value,
            None => {
                ctx.insert_error(message);
                ctx.error_count()
            }
        };

        let error = syn_utils::revert_user_error(error_num);

        match condition {
            Some(condition) => {
                let condition = parse::<_, OdraParser>(condition, ctx)?;
                Ok(crate::parser::syn_utils::if_not(condition, error))
            }
            None => Ok(error),
        }
    }

    fn revert_with_err(error_name: &str) -> syn::Expr {
        let error = utils::to_ident(error_name);
        syn_utils::revert(error)
    }
}
