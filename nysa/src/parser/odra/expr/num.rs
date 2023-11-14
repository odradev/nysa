use crate::{
    formatted_invalid_expr,
    model::ir::{eval_expression_type, Expression},
    parser::{
        context::{
            ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
        },
        odra::ty,
    },
    ParserError,
};
use proc_macro2::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, Token};

macro_rules! to_uint {
    ($value:expr, $t:ty) => {
        <$t>::from_le_bytes(crate::utils::convert_to_array($value))
    };
}

pub(crate) fn to_typed_int_expr<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    value: &[u64],
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let arr = value
        .iter()
        .map(|v| quote::quote!(#v))
        .collect::<Punctuated<TokenStream, Token![,]>>();
    let ty = ctx
        .contextual_expr()
        .map(|e| eval_expression_type(e, ctx))
        .map(|t| t.map(|t| ty::parse_type_from_ty(&t, ctx).ok()))
        .flatten()
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

pub(crate) fn try_to_generic_int_expr(expr: &Expression) -> Result<syn::Expr, ParserError> {
    match expr {
        Expression::NumberLiteral(value) => to_generic_int_expr(value),
        _ => formatted_invalid_expr!("NumLiteral expected but found {:?}", expr),
    }
}

fn to_generic_int_expr(value: &[u64]) -> Result<syn::Expr, ParserError> {
    let bytes = value
        .iter()
        .map(|v| v.to_le_bytes())
        .flatten()
        .collect::<Vec<_>>();
    Ok(to_generic_lit_expr(to_uint!(&bytes[0..4], u32)))
}

fn to_generic_lit_expr<N: num_traits::Num + ToString>(num: N) -> syn::Expr {
    syn::Expr::Lit(syn::ExprLit {
        attrs: vec![],
        lit: syn::Lit::Int(syn::LitInt::new(
            &num.to_string(),
            proc_macro2::Span::call_site(),
        )),
    })
}
