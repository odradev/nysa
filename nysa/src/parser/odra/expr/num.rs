use crate::{
    error::ParserResult,
    formatted_invalid_expr,
    model::ir::{eval_expression_type, Expression},
    parser::{
        common::{self, NumberParser, StatementParserContext},
        odra::{syn_utils::ty::u256, ty},
    },
    OdraParser,
};
use proc_macro2::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, Token};

use super::syn_utils;

impl NumberParser for OdraParser {
    fn parse_typed_number<T: StatementParserContext>(
        values: &[u64],
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let arr = values
            .iter()
            .map(|v| quote::quote!(#v))
            .collect::<Punctuated<TokenStream, Token![,]>>();
        let ty = ctx
            .contextual_expr()
            .map(|e| eval_expression_type(e, ctx))
            .map(|t| t.map(|t| ty::parse_type_from_ty(&t, ctx).ok()))
            .flatten()
            .flatten()
            .unwrap_or(u256());

        if values.is_empty() {
            Ok(parse_quote!(#ty::ZERO))
        } else if values.len() == 1 && values[0] == 1 {
            Ok(parse_quote!(#ty::ONE))
        } else {
            Ok(parse_quote!(#ty::from_limbs_slice(&[#arr])))
        }
    }

    fn parse_generic_number(expr: &Expression) -> ParserResult<syn::Expr> {
        match expr {
            Expression::NumberLiteral(value) => to_generic_int_expr(value),
            _ => formatted_invalid_expr!("NumLiteral expected but found {:?}", expr),
        }
    }

    fn unsigned_one() -> syn::Expr {
        syn_utils::unsigned_one()
    }

    fn words_to_number(words: Vec<u64>, ty: &syn::Type) -> syn::Expr {
        let arr = words
            .iter()
            .map(|v| quote::quote!(#v))
            .collect::<Punctuated<TokenStream, Token![,]>>();
        parse_quote!(#ty::from_limbs([#arr]))
    }
}

macro_rules! to_uint {
    ($value:expr, $t:ty) => {
        <$t>::from_le_bytes(crate::utils::convert_to_array($value))
    };
}

fn to_generic_int_expr(value: &[u64]) -> ParserResult<syn::Expr> {
    let bytes = value
        .iter()
        .map(|v| v.to_le_bytes())
        .flatten()
        .collect::<Vec<_>>();
    Ok(common::expr::num::to_generic_lit_expr(to_uint!(
        &bytes[0..4],
        u32
    )))
}
