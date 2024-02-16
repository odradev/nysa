use crate::{
    error::ParserResult,
    model::ir::Stmt,
    parser::common::{self, EventEmitParser, StatementParserContext},
    OdraParser,
};

/// Parses a nysa statement into a syn::Stmt.
///
/// ## Arguments
/// * stmt - a nysa statement
/// * is_semi - indicates if the `stmt` ends with a semicolon
/// * ctx - parser context
pub fn parse_statement<T>(stmt: &Stmt, is_semi: bool, ctx: &mut T) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
{
    common::stmt::parse_statement::<_, OdraParser>(stmt, is_semi, ctx)
}

impl EventEmitParser for OdraParser {
    fn parse_emit_stmt(
        event_ident: proc_macro2::Ident,
        args: Vec<syn::Expr>,
    ) -> ParserResult<syn::Stmt> {
        Ok(super::syn_utils::stmt::emit_event(event_ident, &args))
    }
}
