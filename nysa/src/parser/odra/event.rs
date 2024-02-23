use std::vec;

use crate::{
    error::ParserResult,
    parser::common::{EventEmitParser, EventParser},
    OdraParser,
};

use super::syn_utils::{attr, stmt};

impl EventParser for OdraParser {
    fn derive_attrs() -> Vec<syn::Attribute> {
        vec![attr::derive_odra_event()]
    }
}

impl EventEmitParser for OdraParser {
    fn parse_emit_stmt(
        event_ident: proc_macro2::Ident,
        args: Vec<syn::Expr>,
    ) -> ParserResult<syn::Stmt> {
        Ok(stmt::emit_event(event_ident, &args))
    }
}
