use proc_macro2::TokenStream;
use syn::parse_quote;
use quote::{quote, format_ident};

use crate::{model::ContractData, ERRORS};

pub(crate) fn errors_def(data: &ContractData) -> syn::Item {
    let errors = data.c3_errors();

    let mut error_count = ERRORS.lock().unwrap();
    *error_count = errors.len() as u16;

    let execution_error_body = errors.iter().enumerate().map(|(idx, e)| {
        let name = format_ident!("{}", e.name.name);
        let idx = idx as u16;
        quote!(#name => #idx,)
    }).collect::<TokenStream>();

    parse_quote! {
        odra::execution_error! {
            pub enum Error {
                #execution_error_body
            }
        }
    }
}
