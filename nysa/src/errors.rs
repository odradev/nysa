use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse_quote;

use crate::{model::ContractData, ERRORS};

pub(crate) fn errors_def(data: &ContractData) -> Option<syn::Item> {
    let errors = data.c3_errors();

    let mut error_count = ERRORS.lock().unwrap();
    *error_count = errors.len() as u16;

    let execution_error_body = errors
        .iter()
        .enumerate()
        .map(|(idx, e)| {
            let name = format_ident!("{}", e.name.name);
            let idx = idx as u16;
            quote!(#name => #idx,)
        })
        .collect::<TokenStream>();
    if execution_error_body.is_empty() {
        return None;
    }
    Some(parse_quote! {
        odra::execution_error! {
            pub enum Error {
                #execution_error_body
            }
        }
    })
}
