use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::{
    model::{ir::Package, Named},
    parser::{
        context::TypeInfo,
        odra::{syn_utils::attr, ty},
    },
    utils, ParserError,
};

pub(crate) fn enums_def(package: &Package) -> Vec<syn::Item> {
    let enums = package.enums();

    enums
        .iter()
        .map(|e| {
            let name = utils::to_ident(e.name());
            let variants = e
                .variants
                .iter()
                .enumerate()
                .map(|(idx, v)| {
                    let attr = (idx == 0).then(|| attr::default());
                    let variant = utils::to_ident(v);
                    let idx = idx as u8;
                    quote!(#attr #variant = #idx)
                })
                .collect::<Punctuated<TokenStream, Token![,]>>();
            let derive_attr = attr::derive_odra_ty();
            parse_quote!(
                #derive_attr
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
                let derive_attr = attr::derive_odra_ty();
                let name = utils::to_ident(s.name());
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
                    #derive_attr
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
