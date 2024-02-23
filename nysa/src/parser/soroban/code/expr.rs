use syn::parse_quote;

use crate::parser::soroban::code::ty;

pub fn storage_instance() -> syn::Expr {
    parse_quote!(env.storage().persistent())
}

pub fn auth_caller() -> syn::Expr {
    parse_quote!(caller.expect("`caller` must not be `None`").require_auth())
}

pub fn cloned_env() -> syn::Expr {
    parse_quote!(env.clone())
}

pub fn cloned_caller() -> syn::Expr {
    parse_quote!(caller.clone())
}

pub fn string_from(string: &str) -> syn::Expr {
    let ty = ty::string();
    parse_quote!(#ty::from_str(&env, #string))
}

pub fn default_u256() -> syn::Expr {
    let ty = ty::u256();
    parse_quote!(#ty::from_u32(&env, 0))
}

pub mod fn_arg {
    use syn::parse_quote;

    use crate::parser::soroban::code::ty;

    pub fn env() -> syn::FnArg {
        let ty = ty::env();
        parse_quote!(env: soroban_sdk::Env)
    }

    pub fn caller() -> syn::FnArg {
        let ty = ty::option(ty::address());
        parse_quote!(caller: #ty)
    }
}
