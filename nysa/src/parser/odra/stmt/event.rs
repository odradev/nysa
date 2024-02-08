use crate::error::ParserResult;
use crate::model::ir::Expression;
use crate::parser::context::{
    ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
    TypeInfo,
};
use crate::parser::odra::expr;
use crate::{utils, ParserError};

use super::syn_utils;

/// Parses a statement emitting an event into a `syn::Stmt`.
///
/// # Solidity example
/// `emit OwnershipTransferred(oldOwner, newOwner);`
pub(super) fn emit<T>(expr: &Expression, ctx: &mut T) -> ParserResult<syn::Stmt>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    match expr {
        Expression::Func(name, args) => {
            let event_ident = TryInto::<String>::try_into(*name.to_owned()).map(utils::to_ident)?;
            let args: Vec<syn::Expr> = args
                .iter()
                .map(|e| expr::primitives::get_var_or_parse(e, ctx))
                .collect::<ParserResult<_>>()?;
            ctx.register_event(&event_ident);

            Ok(syn_utils::emit_event(event_ident, &args))
        }
        _ => Err(ParserError::InvalidExpression(String::from(
            "Invalid Emit statement",
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ir::{Stmt, Type, Var};
    use crate::model::ContractData;
    use crate::parser::context::{ContractContext, GlobalContext, LocalContext};
    use crate::parser::odra::stmt::parse_statement;
    use crate::parser::odra::stmt::test::{
        parse_with_empty_context, unsafe_parse_with_empty_context,
    };
    use crate::parser::odra::test::assert_tokens_eq;
    use quote::quote;

    #[test]
    fn emit_no_args() {
        let stmt = Stmt::Emit(Expression::Func(
            Box::new(Expression::Variable("DataUpdated".to_string())),
            vec![],
        ));

        assert_tokens_eq(
            unsafe_parse_with_empty_context(stmt),
            quote!(self.env().emit_event(DataUpdated::new());),
        );
    }

    #[test]
    fn emit_with_args() {
        let stmt = Stmt::Emit(Expression::Func(
            Box::new(Expression::Variable("DataUpdated".to_string())),
            vec![Expression::BoolLiteral(false)],
        ));

        assert_tokens_eq(
            unsafe_parse_with_empty_context(stmt),
            quote!(self.env().emit_event(DataUpdated::new(false));),
        );
    }

    #[test]
    fn emit_with_context_args() {
        let mut global_ctx = GlobalContext::default();
        let storage = vec![Var {
            name: "my_var".to_string(),
            ty: Type::Bool,
            initializer: None,
            is_immutable: false,
        }];
        let data = ContractData::with_storage("test", storage);

        let contract_ctx = ContractContext::new(&mut global_ctx, data);
        let mut ctx = LocalContext::new(contract_ctx);

        let stmt = Stmt::Emit(Expression::Func(
            Box::new(Expression::Variable("DataUpdated".to_string())),
            vec![
                Expression::Variable("my_var".to_string()),
                Expression::Variable("x".to_string()),
            ],
        ));

        let result = parse_statement(&stmt, true, &mut ctx).expect("Couldn't parse statement");

        assert_tokens_eq(
            result,
            quote!(self.env().emit_event(DataUpdated::new(self.my_var.get_or_default(), x));),
        );
    }

    #[test]
    fn emit_with_invalid_arg() {
        let stmt = Stmt::Emit(Expression::Func(
            Box::new(Expression::Variable("DataUpdated".to_string())),
            vec![Expression::Fail],
        ));

        assert!(parse_with_empty_context(stmt).is_err())
    }

    #[test]
    fn emit_invalid_stmt() {
        let stmt = Stmt::Emit(Expression::Fail);
        assert_eq!(
            parse_with_empty_context(stmt),
            Err(ParserError::InvalidExpression(String::from(
                "Invalid Emit statement"
            )))
        );
    }
}
