use crate::utils;
use quote::ToTokens;
use syn::parse_quote;

pub fn short_symbol<T: AsRef<str>>(name: T) -> syn::Item {
    let ty = super::ty::symbol();
    let str = utils::to_upper_snake_case(&name.as_ref().chars().take(9).collect::<String>());
    let ident = utils::to_upper_snake_case_ident(name);
    parse_quote!(const #ident: #ty = soroban_sdk::symbol_short!(#str);)
}

pub fn contract_type<T: AsRef<str>, K: ToTokens>(name: T, key: K) -> syn::Item {
    let attr = super::attr::contracttype();
    let ident = utils::to_pascal_case_ident(name);

    parse_quote!(
        #attr
        pub struct #ident(#key);
    )
}
