use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::{
    model::{ir::Package, Named},
    utils,
};

use super::syn_utils::attr;

pub(crate) fn errors_def(package: &Package) -> Option<syn::Item> {
    let execution_error_body = package
        .errors()
        .iter()
        .enumerate()
        .map(|(idx, e)| {
            let name = utils::to_ident(e.name());
            let idx = idx as isize;
            quote!(#name = #idx,)
        })
        .collect::<TokenStream>();
    if execution_error_body.is_empty() {
        return None;
    }
    let derive_attr = attr::derive_odra_err();
    Some(parse_quote! {
        #derive_attr
        pub enum Error {
            #execution_error_body
        }
    })
}
