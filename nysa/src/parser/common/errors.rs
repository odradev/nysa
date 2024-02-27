use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Token};

use crate::{
    model::{ir::Package, Named},
    utils,
};

use super::{expr::num::to_generic_lit_expr, ErrorParser};

pub(crate) fn errors_def<P: ErrorParser>(package: &Package) -> Option<syn::Item> {
    let execution_error_body = package
        .errors()
        .iter()
        .enumerate()
        .map(|(idx, e)| {
            let name = utils::to_ident(e.name());
            let idx = to_generic_lit_expr(idx);
            quote!(#name = #idx)
        })
        .collect::<Punctuated<TokenStream, Token![,]>>();
    if execution_error_body.is_empty() {
        return None;
    }

    P::errors_def(execution_error_body)
}
