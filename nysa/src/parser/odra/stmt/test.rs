use crate::{model::ir::Stmt, parser::context::test::EmptyContext, ParserError};

use super::parse_statement;

#[test]
#[should_panic]
fn fail() {
    let _ = parse_with_empty_context(Stmt::Unknown);
}

pub(super) fn unsafe_parse_with_empty_context(stmt: Stmt) -> syn::Stmt {
    parse_with_empty_context(stmt).expect("Couldn't parse statement")
}

pub(super) fn parse_with_empty_context(stmt: Stmt) -> Result<syn::Stmt, ParserError> {
    parse_statement(&stmt, true, &mut EmptyContext)
}
