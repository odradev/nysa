use syn::parse_quote;

pub fn caller() -> syn::Type {
    parse_quote!(soroban_sdk::Address)
}

pub mod fn_arg {
    use proc_macro2::Ident;
    use syn::parse_quote;

    pub fn env() -> syn::FnArg {
        parse_quote!(env: soroban_sdk::Env)
    }

    pub fn caller() -> syn::FnArg {
        parse_quote!(caller: soroban_sdk::Address)
    }

    pub fn custom(ident: Ident, ty: syn::Type) -> syn::FnArg {
        parse_quote!(#ident: #ty)
    }
}
