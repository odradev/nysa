use itertools::Itertools;
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
                #[derive(odra::OdraType, PartialEq, Eq, Debug, Default)]
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

    let mut result: Vec<syn::Item> = vec![];

    for (key, group) in &structs.into_iter().group_by(|s| s.namespace.clone()) {
        let namespace = key.as_ref().map(utils::to_snake_case_ident);

        let items = group
            .map(|s| {
                let name = format_ident!("{}", s.name);
                let fields = s
                    .fields
                    .iter()
                    .map(|(name, ty)| {
                        let ident = utils::to_snake_case_ident(name);
                        let ty = ty::parse_type_from_expr(ty, t)?;
                        Ok(quote!(pub #ident: #ty))
                    })
                    .collect::<Result<Punctuated<TokenStream, Token![,]>, _>>()?;
                let struct_def: syn::Item = parse_quote!(
                    #[derive(odra::OdraType, PartialEq, Eq, Debug, Default)]
                    pub struct #name { #fields }
                );
                Ok(struct_def)
            })
            .collect::<Result<Vec<syn::Item>, ParserError>>()?;

        if let Some(ns) = namespace {
            result.push(parse_quote!(pub mod #ns { #(#items)* }));
        } else {
            result.extend(items);
        }
    }
    Ok(result)
}
