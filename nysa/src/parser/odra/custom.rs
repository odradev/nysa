use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::{
    model::ir::Package,
    parser::{context::TypeInfo, odra::ty},
    utils, ParserError,
};

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

pub(crate) fn struct_def<T: TypeInfo>(
    package: &Package,
    t: &T,
) -> Result<Vec<syn::Item>, ParserError> {
    let structs = package.structs();

    structs
        .iter()
        .map(|s| {
            let namespace = s.namespace.as_ref().map(utils::to_snake_case_ident);
            let name = format_ident!("{}", s.name);
            let fields = s
                .fields
                .iter()
                .map(|(name, ty)| {
                    let ident = utils::to_snake_case_ident(name);
                    let ty = ty::parse_type_from_expr(ty, t)?;
                    Ok(quote!(#ident: #ty))
                })
                .collect::<Result<Punctuated<TokenStream, Token![,]>, _>>();

            match fields {
                Ok(fields) => {
                    let struct_def: syn::Item = parse_quote!(
                        #[derive(odra::OdraType, Copy, PartialEq, Eq, Debug, Default)]
                        pub struct #name { #fields }
                    );
                    Ok(match namespace {
                        Some(ns) => parse_quote!(pub mod #ns { #struct_def }),
                        None => struct_def,
                    })
                }
                Err(err) => Err(err),
            }
        })
        .collect()
}
