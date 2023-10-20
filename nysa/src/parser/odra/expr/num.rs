use crate::{
    parser::{
        context::{FnContext, TypeInfo},
        odra::ty,
    },
    ParserError,
};
use proc_macro2::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, Token};

pub(crate) fn to_typed_int_expr<T: TypeInfo + FnContext>(
    value: &[u64],
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let ty = ctx.expected_type();
    let arr = value
        .iter()
        .map(|v| quote::quote!(#v))
        .collect::<Punctuated<TokenStream, Token![,]>>();
    let ty = ty
        .map(|t| ty::parse_type_from_ty(&t, ctx).ok())
        .flatten()
        .unwrap_or(parse_quote!(nysa_types::U256));
    if value.is_empty() {
        Ok(parse_quote!(#ty::ZERO))
    } else if value.len() == 1 && value[0] == 1 {
        Ok(parse_quote!(#ty::ONE))
    } else {
        Ok(parse_quote!(#ty::from_limbs_slice(&[#arr])))
    }
}
