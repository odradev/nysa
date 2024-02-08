use proc_macro2::Ident;
use syn::parse_quote;

pub fn fn_arg(name: Ident, ty: syn::Type) -> syn::FnArg {
    parse_quote!( #name: #ty )
}

pub fn self_arg(is_mut: bool) -> syn::FnArg {
    let mut_mod = is_mut.then(|| quote::quote!(mut));
    parse_quote!( &#mut_mod self )
}
