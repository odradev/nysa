use crate::{
    error::ParserResult,
    model::ir::Stmt,
    parser::context::{
        ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
        TypeInfo,
    },
};

use super::expr;

mod block;
mod control_flow;
mod error;
mod event;
pub mod ext;
mod ret;
mod syn_utils;
mod variables;

/// Parses a nysa statement into a syn::Stmt.
///
/// ## Arguments
/// * stmt - a nysa statement
/// * is_semi - indicates if the `stmt` ends with a semicolon
/// * ctx - parser context
pub fn parse_statement<T>(stmt: &Stmt, is_semi: bool, ctx: &mut T) -> ParserResult<syn::Stmt>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    match stmt {
        Stmt::Expression(expr) => expr::parse_expr(expr, is_semi, ctx),
        Stmt::VarDefinition(name, ty, init) => variables::definition(name, ty, init, ctx),
        Stmt::VarDeclaration(name, ty) => variables::declaration(name, ty, ctx),
        Stmt::Return(expr) => ret::ret(expr, ctx),
        Stmt::ReturnVoid => ret::ret_unit(),
        Stmt::If(assertion, if_body) => control_flow::if_stmt(assertion, if_body, ctx),
        Stmt::IfElse(assertion, if_body, else_body) => {
            control_flow::if_else_stmt(assertion, if_body, else_body, ctx)
        }
        Stmt::Block(stmts) => block::block(stmts, ctx),
        Stmt::ReturningBlock(stmts) => block::ret_block(stmts, ctx),
        Stmt::Emit(expr) => event::emit(expr, ctx),
        Stmt::Revert(msg) => error::revert(msg, ctx),
        Stmt::RevertWithError(msg) => error::revert_with_msg(msg),
        Stmt::While(assertion, block) => control_flow::while_loop(assertion, block, ctx),
        #[cfg(test)]
        Stmt::Fail => Err(crate::ParserError::InvalidStatement("Fail")),
        _ => panic!("Unsupported statement {:?}", stmt),
    }
}

#[cfg(test)]
mod test;
