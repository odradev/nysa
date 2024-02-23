use proc_macro2::Ident;
use syn::parse_quote;

use crate::{error::ParserResult, parser::common::ExtContractParser, OdraParser};

use super::syn_utils;

impl ExtContractParser for OdraParser {
    fn parse_ext_contract(
        mod_ident: Ident,
        contract_ident: Ident,
        items: Vec<syn::TraitItem>,
    ) -> ParserResult<syn::ItemMod> {
        let derive_attr = syn_utils::attr::derive_ext_contract();
        Ok(parse_quote!(
            pub mod #mod_ident {
                #![allow(unused_imports)]
                use odra::prelude::*;

                #derive_attr
                pub trait #contract_ident {
                    #(#items)*
                }
            }
        ))
    }
}
