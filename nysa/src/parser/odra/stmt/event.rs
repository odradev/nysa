use crate::model::ir::Expression;
use crate::parser::context::{
    ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
};
use crate::parser::odra::expr;
use crate::{utils, ParserError};
use syn::parse_quote;

pub(super) fn emit<T>(expr: &Expression, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    match expr {
        Expression::Func(name, args) => {
            let event_ident =
                TryInto::<String>::try_into(*name.to_owned()).map(|name| utils::to_ident(&name))?;
            let args: Vec<syn::Expr> = args
                .iter()
                .map(|e| expr::primitives::get_var_or_parse(e, ctx))
                .collect::<Result<_, _>>()?;
            ctx.register_event(&event_ident);

            Ok(parse_quote!(
                <#event_ident as odra::types::event::OdraEvent>::emit(
                    #event_ident::new(#(#args),*)
                );
            ))
        }
        _ => panic!("Invalid Emit statement"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ir::{Stmt, Type, Var};
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
            quote!(<DataUpdated as odra::types::event::OdraEvent>::emit(
            DataUpdated::new()
        );),
        );
    }

    // #[test]
    // fn emit_with_args() {
    //     let stmt = Stmt::Emit(Expression::Func(
    //         Box::new(Expression::Variable("DataUpdated".to_string())),
    //         vec![
    //             Expression::BoolLiteral(false),
    //             Expression::NumberLiteral(NumSize::U8, vec![100]),
    //         ],
    //     ));

    //     assert_tokens_eq(
    //         unsafe_parse_with_empty_context(stmt),
    //         quote!(<DataUpdated as odra::types::event::OdraEvent>::emit(
    //         DataUpdated::new(false, 100u8.into())
    //     );),
    //     );
    // }

    #[test]
    fn emit_with_context_args() {
        let global_ctx = GlobalContext::new(vec![], vec![], vec![], vec![], vec![]);
        let storage = vec![Var {
            name: "my_var".to_string(),
            ty: Type::Bool,
            initializer: None,
        }];
        let contract_ctx = ContractContext::new(&global_ctx, &storage);
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
            quote!(<DataUpdated as odra::types::event::OdraEvent>::emit(
            DataUpdated::new(self.my_var.get_or_default(), x)
        );),
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
    #[should_panic]
    fn emit_invalid_stmt() {
        let stmt = Stmt::Emit(Expression::Fail);
        let _ = parse_with_empty_context(stmt);
    }
}
