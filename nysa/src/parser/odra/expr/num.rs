use crate::{model::ir::NumSize, to_unit};
use proc_macro2::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, Token};

pub(crate) fn to_generic_int_expr(ty: &NumSize, value: &[u8]) -> syn::Expr {
    match ty {
        NumSize::U8 => to_generic_lit_expr(to_unit!(&value[0..1], u8)),
        NumSize::U16 => to_generic_lit_expr(to_unit!(&value[0..2], u16)),
        NumSize::U32 => to_generic_lit_expr(to_unit!(value, u32)),
        NumSize::U64 => to_generic_lit_expr(to_unit!(value, u64)),
        NumSize::U256 => {
            let arr = value
                .iter()
                .map(|v| quote::quote!(#v))
                .collect::<Punctuated<TokenStream, Token![,]>>();
            parse_quote!(odra::types::U256::from_big_endian(&[#arr]))
        }
        _ => panic!("unknown type"),
    }
}

pub(crate) fn to_typed_int_expr(ty: &NumSize, value: &[u8]) -> syn::Expr {
    match ty {
        NumSize::U8 => {
            let num = to_unit!(&value[0..1], u8);
            parse_quote!(#num.into())
        }
        NumSize::U16 => {
            let num = to_unit!(&value[0..2], u16);
            parse_quote!(#num.into())
        }
        NumSize::U32 => {
            let num = to_unit!(value, u32);
            parse_quote!(#num.into())
        }
        NumSize::U64 => {
            let num = to_unit!(value, u64);
            parse_quote!(#num.into())
        }
        NumSize::U256 => {
            let arr = value
                .iter()
                .map(|v| quote::quote!(#v))
                .collect::<Punctuated<TokenStream, Token![,]>>();
            parse_quote!(odra::types::U256::from_big_endian(&[#arr]))
        }
        _ => panic!("unknown type"),
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
