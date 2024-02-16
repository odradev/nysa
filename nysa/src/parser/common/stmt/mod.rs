use crate::{error::ParserResult, model::ir::Stmt, Parser};

use super::{expr, StatementParserContext};

mod block;
mod control_flow;
mod error;
mod event;
pub mod ext;
mod ret;
mod variables;

/// Parses a nysa statement into a syn::Stmt.
///
/// ## Arguments
/// * stmt - a nysa statement
/// * is_semi - indicates if the `stmt` ends with a semicolon
/// * ctx - parser context
pub fn parse_statement<T, P>(stmt: &Stmt, is_semi: bool, ctx: &mut T) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    match stmt {
        Stmt::Expression(expr) => expr::parse_expr::<_, P>(expr, is_semi, ctx),
        Stmt::VarDefinition(name, ty, init) => variables::definition::<_, P>(name, ty, init, ctx),
        Stmt::VarDeclaration(name, ty) => variables::declaration(name, ty, ctx),
        Stmt::Return(expr) => ret::ret::<_, P>(expr, ctx),
        Stmt::ReturnVoid => ret::ret_unit(),
        Stmt::If(assertion, if_body) => control_flow::if_stmt::<_, P>(assertion, if_body, ctx),
        Stmt::IfElse(assertion, if_body, else_body) => {
            control_flow::if_else_stmt::<_, P>(assertion, if_body, else_body, ctx)
        }
        Stmt::Block(stmts) => block::block::<_, P>(stmts, ctx),
        Stmt::ReturningBlock(stmts) => block::ret_block::<_, P>(stmts, ctx),
        Stmt::Emit(expr) => event::emit::<_, P>(expr, ctx),
        Stmt::Revert(msg) => error::revert::<_, P::ContractErrorParser>(msg, ctx),
        Stmt::RevertWithError(msg) => error::revert_with_msg::<P::ContractErrorParser>(msg),
        Stmt::While(assertion, block) => control_flow::while_loop::<_, P>(assertion, block, ctx),
        #[cfg(test)]
        Stmt::Fail => Err(crate::ParserError::InvalidStatement("Fail")),
        _ => panic!("Unsupported statement {:?}", stmt),
    }
}
