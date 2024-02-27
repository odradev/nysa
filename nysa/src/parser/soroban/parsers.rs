use proc_macro2::{Ident, TokenStream};
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::{
    error::ParserResult,
    model::ir::{Enum, Struct},
    parser::{
        common::{
            ContractReferenceParser, CustomElementParser, CustomImports, ErrorParser,
            ExtContractParser,
        },
        context::TypeInfo,
    },
    SorobanParser,
};

use super::code;

mod error;
mod event;
mod expr;
mod func;
mod types;

impl CustomElementParser for SorobanParser {
    fn parse_custom_struct<T: TypeInfo>(
        namespace: &Option<Ident>,
        model: &Struct,
        ctx: &T,
    ) -> ParserResult<syn::Item> {
        todo!()
    }

    fn parse_custom_enum(name: Ident, model: &Enum) -> syn::Item {
        todo!()
    }
}

impl ExtContractParser for SorobanParser {
    fn parse_ext_contract(
        mod_ident: Ident,
        contract_ident: Ident,
        items: Vec<syn::TraitItem>,
    ) -> ParserResult<syn::ItemMod> {
        todo!()
    }
}

impl ErrorParser for SorobanParser {
    fn errors_def(variants: Punctuated<TokenStream, Token![,]>) -> Option<syn::Item> {
        let attrs = code::attr::error();
        Some(parse_quote! {
            #( #attrs )*
            pub enum Error {
                #variants
            }
        })
    }
}

impl CustomImports for SorobanParser {
    fn import_items() -> Vec<syn::Item> {
        vec![]
    }
}

impl ContractReferenceParser for SorobanParser {
    fn parse_contract_ref(variable_name: &str, contract_name: &str) -> syn::Stmt {
        todo!()
    }

    fn parse_contract_ref_expr(contract_name: &str, address: syn::Expr) -> syn::Expr {
        todo!()
    }
}
