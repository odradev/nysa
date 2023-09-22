use syn::parse_quote;

use crate::model::ir::Stmt;
use crate::parser::context::{
    ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
};
use crate::ParserError;

pub(super) fn block<T>(stmts: &[Stmt], ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let stmts = parse(stmts.iter(), ctx)?;
    Ok(parse_quote!({ #(#stmts)* }))
}

pub(super) fn ret_block<T>(stmts: &[Stmt], ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let last_stmt = stmts
        .last()
        .map(|stmt| super::parse_statement(stmt, false, ctx))
        .ok_or(ParserError::InvalidStatement(
            "A statement expected but not found",
        ))??;
    let stmts = parse(stmts.iter().take(stmts.len() - 1), ctx)?;

    Ok(parse_quote!({
        #(#stmts)*
        #last_stmt
    }))
}

fn parse<'a, I, T>(stmts: I, ctx: &mut T) -> Result<Vec<syn::Stmt>, ParserError>
where
    I: Iterator<Item = &'a Stmt>,
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    stmts
        .map(|stmt| super::parse_statement(stmt, true, ctx))
        .collect::<Result<Vec<syn::Stmt>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ir::{Expression, Type};
    use crate::parser::odra::stmt::test::{
        parse_with_empty_context, unsafe_parse_with_empty_context,
    };
    use crate::parser::odra::test::assert_tokens_eq;
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
