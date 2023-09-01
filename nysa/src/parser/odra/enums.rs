use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::model::ir::Package;

pub(crate) fn enums_def(package: &Package) -> Vec<syn::Item> {
    let enums = package.enums();

    enums
        .iter()
        .map(|e| {
            let name = format_ident!("{}", e.name);
            let variants = e
                .variants
                .iter()
                .enumerate()
                .map(|(idx, v)| {
                    let variant = format_ident!("{}", v);
                    let idx = idx as u8;
                    let attr = (idx == 0).then(|| quote!(#[default]));
                    quote!(#attr #variant = #idx)
                })
                .collect::<Punctuated<TokenStream, Token![,]>>();
            parse_quote!(
                #[derive(odra::OdraType, Copy, PartialEq, Eq, Debug, Default)]
                pub enum #name { #variants }
            )
        })
        .collect()
}
