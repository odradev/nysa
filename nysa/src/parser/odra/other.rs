use syn::parse_quote;

use crate::{parser::common::CustomImports, OdraParser};

impl CustomImports for OdraParser {
    fn import_items() -> Vec<syn::Item> {
        vec![parse_quote!(
            use odra::prelude::*;
        )]
    }
}
