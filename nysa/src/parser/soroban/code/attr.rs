pub fn contract() -> syn::Attribute {
    syn::parse_quote!(#[soroban_sdk::contract])
}

pub fn contractimpl() -> syn::Attribute {
    syn::parse_quote!(#[soroban_sdk::contractimpl])
}

pub fn contracttype() -> syn::Attribute {
    syn::parse_quote!(#[soroban_sdk::contracttype])
}

pub(crate) fn module_attrs() -> Vec<syn::Attribute> {
    vec![
        syn::parse_quote!(#![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports, unused_variables)]),
    ]
}

pub fn error() -> Vec<syn::Attribute> {
    vec![
        syn::parse_quote!(#[soroban_sdk::contracterror]),
        syn::parse_quote!(#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]),
        syn::parse_quote!(#[repr(u32)]),
    ]
}
