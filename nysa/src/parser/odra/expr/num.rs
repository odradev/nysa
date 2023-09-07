use crate::{
    model::ir::{NumSize, NysaExpression},
    to_unit, ParserError,
};
use proc_macro2::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, Token};

pub(crate) fn to_generic_int_expr(ty: &NumSize, value: &[u8]) -> Result<syn::Expr, ParserError> {
    match ty {
        NumSize::U8 => Ok(to_generic_lit_expr(to_unit!(&value[0..1], u8))),
        NumSize::U16 => Ok(to_generic_lit_expr(to_unit!(&value[0..2], u16))),
        NumSize::U32 => Ok(to_generic_lit_expr(to_unit!(value, u32))),
        NumSize::U64 => Ok(to_generic_lit_expr(to_unit!(value, u64))),
        NumSize::U256 => {
            let arr = value
                .iter()
                .map(|v| quote::quote!(#v))
                .collect::<Punctuated<TokenStream, Token![,]>>();
            Ok(parse_quote!(odra::types::U256::from_big_endian(&[#arr])))
        }
        s => Err(ParserError::UnsupportedUnit(s.clone())),
    }
}

pub(crate) fn to_typed_int_expr(ty: &NumSize, value: &[u8]) -> Result<syn::Expr, ParserError> {
    match ty {
        NumSize::U8 => {
            let num = if value.is_empty() {
                0
            } else {
                to_unit!(&value[0..1], u8)
            };

            Ok(parse_quote!(#num.into()))
        }
        NumSize::U16 => {
            let num = to_unit!(&value[0..2], u16);
            Ok(parse_quote!(#num.into()))
        }
        NumSize::U32 => {
            let num = to_unit!(value, u32);
            Ok(parse_quote!(#num.into()))
        }
        NumSize::U64 => {
            let num = to_unit!(value, u64);
            Ok(parse_quote!(#num.into()))
        }
        NumSize::U256 => {
            let arr = value
                .iter()
                .map(|v| quote::quote!(#v))
                .collect::<Punctuated<TokenStream, Token![,]>>();
            Ok(parse_quote!(odra::types::U256::from_big_endian(&[#arr])))
        }
        s => Err(ParserError::UnsupportedUnit(s.clone())),
    }
}

pub(crate) fn to_generic_lit_expr<N: num_traits::Num + ToString>(num: N) -> syn::Expr {
    syn::Expr::Lit(syn::ExprLit {
        attrs: vec![],
        lit: syn::Lit::Int(syn::LitInt::new(
            &num.to_string(),
            proc_macro2::Span::call_site(),
        )),
    })
}

pub(crate) fn try_to_generic_int_expr(expr: &NysaExpression) -> Result<syn::Expr, ParserError> {
    match expr {
        NysaExpression::NumberLiteral { ty, value } => to_generic_int_expr(ty, value),
        _ => Err(ParserError::InvalidExpression),
    }
}
