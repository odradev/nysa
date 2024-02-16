pub mod ty {
    use quote::ToTokens;
    use syn::parse_quote;

    pub fn var<T: ToTokens>(ty: T) -> syn::Type {
        parse_quote!(odra::Var<#ty>)
    }

    pub fn map<T: ToTokens, U: ToTokens>(key: T, value: U) -> syn::Type {
        parse_quote!(odra::Mapping<#key, #value>)
    }

    pub fn option<T: ToTokens>(ty: T) -> syn::Type {
        parse_quote!(Option<#ty>)
    }

    pub fn address() -> syn::Type {
        parse_quote!(odra::Address)
    }

    pub fn string() -> syn::Type {
        parse_quote!(odra::prelude::string::String)
    }

    pub fn bool() -> syn::Type {
        parse_quote!(bool)
    }

    pub fn vec<T: ToTokens>(ty: T) -> syn::Type {
        parse_quote!(odra::prelude::vec::Vec<#ty>)
    }

    pub fn fixed_bytes(size: usize) -> syn::Type {
        parse_quote!(nysa_types::FixedBytes<#size>)
    }

    pub fn u256() -> syn::Type {
        parse_quote!(nysa_types::U256)
    }
}

pub mod attr {
    use quote::ToTokens;

    pub fn module() -> syn::Attribute {
        syn::parse_quote!(#[odra::module])
    }

    pub fn module_with_events<T: ToTokens>(events: Vec<T>) -> syn::Attribute {
        syn::parse_quote!(#[odra::module(events = [ #(#events),* ])])
    }

    pub fn derive_odra_ty() -> syn::Attribute {
        syn::parse_quote!(#[derive(odra::OdraType, PartialEq, Eq, Debug, Default)])
    }

    pub fn derive_odra_err() -> syn::Attribute {
        syn::parse_quote!(#[derive(odra::OdraError, PartialEq, Eq, Debug)])
    }

    pub fn derive_odra_event() -> syn::Attribute {
        syn::parse_quote!(#[derive(odra::Event, PartialEq, Eq, Debug)])
    }

    pub fn default() -> syn::Attribute {
        syn::parse_quote!(#[default])
    }

    pub fn payable() -> syn::Attribute {
        syn::parse_quote!(#[odra(payable)])
    }
}

pub mod stmt {
    use proc_macro2::Ident;
    use quote::ToTokens;
    use syn::parse_quote;

    use crate::parser::{self, odra::expr};

    pub fn contract_ref(var_name: &str, contract_name: &str) -> syn::Stmt {
        let ident = crate::utils::to_ident(var_name);
        let contract_ref = expr::syn_utils::contract_ref(contract_name, &ident);
        parser::syn_utils::definition(ident, contract_ref)
    }

    pub fn emit_event<T: ToTokens>(event_ident: Ident, args: &[T]) -> syn::Stmt {
        parse_quote!(self.env().emit_event(#event_ident::new(#(#args),*));)
    }
}
