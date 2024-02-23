use proc_macro2::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::{parser::common::ErrorParser, OdraParser};

use super::syn_utils::attr;

impl ErrorParser for OdraParser {
    fn errors_def(variants: Punctuated<TokenStream, Token![,]>) -> Option<syn::Item> {
        let derive_attr = attr::derive_odra_err();
        Some(parse_quote! {
            #derive_attr
            pub enum Error {
                #variants
            }
        })
    }
}
