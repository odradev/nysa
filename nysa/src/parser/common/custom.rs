use itertools::Itertools;
use syn::parse_quote;

use crate::{
    error::ParserResult,
    model::{ir::Package, Named},
    parser::context::TypeInfo,
    utils, Parser,
};

use super::CustomElementParser;

pub(crate) fn enums_def<P: CustomElementParser>(package: &Package) -> Vec<syn::Item> {
    let enums = package.enums();

    enums
        .iter()
        .map(|e| {
            let name = utils::to_ident(e.name());
            P::parse_custom_enum(name, e)
        })
        .collect()
}

pub(crate) fn struct_def<T: TypeInfo, P: Parser>(
    package: &Package,
    ctx: &T,
) -> ParserResult<Vec<syn::Item>> {
    let structs = package.structs();

    let mut result: Vec<syn::Item> = vec![];

    for (key, group) in &structs.into_iter().group_by(|s| s.namespace.clone()) {
        let namespace = key.as_ref().map(utils::to_snake_case_ident);

        let items = group
            .map(|s| {
                <P::ElementsParser as CustomElementParser>::parse_custom_struct(&namespace, s, ctx)
            })
            .collect::<ParserResult<Vec<syn::Item>>>()?;

        if let Some(ns) = namespace {
            result.push(parse_quote!(pub mod #ns { #(#items)* }));
        } else {
            result.extend(items);
        }
    }
    Ok(result)
}
