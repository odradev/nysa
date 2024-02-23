use crate::{
    error::ParserResult,
    parser::odra::{expr, syn_utils::ty},
};
use quote::ToTokens;
use syn::parse_quote;

pub fn unwrap_or_revert<T: ToTokens>(expr: T) -> syn::Expr {
    parse_quote!(odra::UnwrapOrRevert::unwrap_or_revert(#expr, &self.env()))
}

pub fn string_from<T: ToTokens>(expr: T) -> syn::Expr {
    let ty = ty::string();
    parse_quote!(#ty::from(#expr))
}

pub fn try_fixed_bytes<T: ToTokens>(args: &[T]) -> syn::Expr {
    parse_quote!(nysa_types::FixedBytes::try_from(&self.env().hash(#(#args),*)).unwrap_or_default())
}

pub fn unsigned_one() -> syn::Expr {
    parse_quote!(nysa_types::Unsigned::ONE)
}

pub fn revert_user_error(num: u16) -> syn::Expr {
    if cfg!(test) {
        parse_quote!(self.env().revert(odra::ExecutionError::User(1u16)))
    } else {
        parse_quote!(self.env().revert(odra::ExecutionError::User(#num)))
    }
}

pub fn revert<T: ToTokens>(error: T) -> syn::Expr {
    parse_quote!(self.env().revert(Error::#error))
}

pub fn serialize<T: ToTokens>(args: &[T]) -> syn::Expr {
    let to_bytes: syn::Type = parse_quote!(odra::casper_types::bytesrepr::ToBytes);
    let unwrap_or_revert: syn::Type = parse_quote!(odra::UnwrapOrRevert);
    parse_quote!({
        let mut result = Vec::new();
        #(result.extend(#unwrap_or_revert::unwrap_or_revert(#to_bytes::to_bytes(&#args), &self.env()));)*
        result
    })
}

pub fn contract_ref<T: ToTokens>(contract_name: &str, address_var: T) -> syn::Expr {
    let ref_ident = crate::utils::to_ref_ident(contract_name);
    let address = expr::syn_utils::unwrap_or_revert(address_var);

    parse_quote!(#ref_ident::new(self.env(), #address))
}

pub trait ReadValue {
    fn expr<F: ToTokens, K: ToTokens>(field: F, key: K) -> ParserResult<syn::Expr>;
}

pub struct DefaultValue;

impl ReadValue for DefaultValue {
    fn expr<F: ToTokens, K: ToTokens>(field: F, key: K) -> ParserResult<syn::Expr> {
        Ok(parse_quote!(#field.get_or_default(#key)))
    }
}

pub struct UnwrapOrNone;

impl ReadValue for UnwrapOrNone {
    fn expr<F: ToTokens, K: ToTokens>(field: F, key: K) -> ParserResult<syn::Expr> {
        Ok(parse_quote!(#field.get(#key).unwrap_or(None)))
    }
}

pub struct UnwrapOrRevert;

impl ReadValue for UnwrapOrRevert {
    fn expr<F: ToTokens, K: ToTokens>(field: F, key: K) -> ParserResult<syn::Expr> {
        let get_expr = quote::quote!(#field.get(#key));
        Ok(unwrap_or_revert(get_expr))
    }
}

pub struct ArrayReader;

impl ReadValue for ArrayReader {
    fn expr<F: ToTokens, K: ToTokens>(field: F, key: K) -> ParserResult<syn::Expr> {
        Ok(parse_quote!(#field.get_or_default()#key))
    }
}
