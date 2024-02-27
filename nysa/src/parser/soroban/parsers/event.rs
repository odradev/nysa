use proc_macro2::Ident;
use syn::parse_quote;

use crate::{
    error::ParserResult,
    parser::{
        common::{EventEmitParser, EventParser},
        soroban::code,
    },
    SorobanParser,
};

impl EventEmitParser for SorobanParser {
    fn parse_emit_stmt(event_ident: Ident, args: Vec<syn::Expr>) -> ParserResult<syn::Stmt> {
        Ok(parse_quote!(env.events().publish((), #event_ident::new( #(#args.clone()),* ));))
    }
}

impl EventParser for SorobanParser {
    fn derive_attrs() -> Vec<syn::Attribute> {
        vec![code::attr::contracttype()]
    }
}
