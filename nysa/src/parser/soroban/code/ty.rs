use syn::parse_quote;

pub fn address() -> syn::Type {
    parse_quote!(soroban_sdk::Address)
}

pub fn option(ty: syn::Type) -> syn::Type {
    parse_quote!(Option<#ty>)
}

pub fn symbol() -> syn::Type {
    parse_quote!(soroban_sdk::Symbol)
}

pub fn string() -> syn::Type {
    parse_quote!(soroban_sdk::String)
}

pub fn env() -> syn::Type {
    parse_quote!(soroban_sdk::Env)
}
