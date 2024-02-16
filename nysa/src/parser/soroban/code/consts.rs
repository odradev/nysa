use syn::parse_quote;

use crate::utils;

pub fn short_symbol<T: AsRef<str>>(name: T) -> syn::ItemConst {
    let ty = super::ty::symbol();
    let str = utils::to_upper_snake_case(name.as_ref());
    let ident = utils::to_upper_snake_case_ident(name);
    parse_quote!(const #ident: #ty = soroban_sdk::symbol_short!(#str);)
}
