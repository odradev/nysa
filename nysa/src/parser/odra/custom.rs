use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::{
    model::Named,
    parser::{
        common::{ty::parse_type_from_expr, CustomElementParser},
        context::TypeInfo,
        odra::syn_utils::attr,
    },
    utils, OdraParser,
};

impl CustomElementParser for OdraParser {
    fn parse_custom_struct<T: TypeInfo>(
        namespace: &Option<proc_macro2::Ident>,
        model: &crate::model::ir::Struct,
        ctx: &T,
    ) -> crate::error::ParserResult<syn::Item> {
        let derive_attr = attr::derive_odra_ty();
        let name = utils::to_ident(model.name());
        let fields = model
            .fields
            .iter()
            .map(|(name, ty)| {
                let ident = utils::to_snake_case_ident(name);
                let ty = parse_type_from_expr::<_, OdraParser>(ty, ctx)?;
                Ok(quote!(pub #ident: #ty))
            })
            .collect::<Result<Punctuated<TokenStream, Token![,]>, _>>()?;
        let struct_def: syn::Item = parse_quote!(
            #derive_attr
            pub struct #name { #fields }
        );
        Ok(struct_def)
    }

    fn parse_custom_enum(name: proc_macro2::Ident, model: &crate::model::ir::Enum) -> syn::Item {
        let variants = model
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
    }
}
