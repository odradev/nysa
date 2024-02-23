use proc_macro2::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::{
    error::ParserResult,
    model::ir::{eval_expression_type, Expression, Type},
    parser::{
        common::{NumberParser, StatementParserContext, StringParser, TypeParser},
        context::TypeInfo,
        soroban::code,
    },
    ParserError, SorobanParser,
};

impl NumberParser for SorobanParser {
    fn parse_typed_number<T: StatementParserContext>(
        values: &[u64],
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let ty = ctx
            .contextual_expr()
            .map(|e| eval_expression_type(e, ctx))
            .flatten()
            .unwrap_or(Type::Uint(256));

        let syn_ty = Self::parse_ty(&ty, ctx).unwrap_or(code::ty::u256());

        if let Type::Int(size) | Type::Uint(size) = ty {
            if size > 128 {
                let mut _values = [0u64; 4];
                for (i, v) in values.iter().enumerate() {
                    _values[i] = *v;
                }
                let hi_hi = _values[0];
                let hi_lo = _values[1];
                let lo_hi = _values[2];
                let lo_lo = _values[3];
                Ok(
                    parse_quote!(soroban_sdk::U256::from_parts(&env, #hi_hi, #hi_lo, #lo_hi, #lo_lo)),
                )
            } else {
                let bytes = to_array(values, size);
                let arr = bytes
                    .iter()
                    .map(|v| quote::quote!(#v))
                    .collect::<Punctuated<TokenStream, Token![,]>>();
                Ok(parse_quote!(#syn_ty::from_le_bytes(&[#arr])))
            }
        } else {
            panic!("Invalid type for number literal")
        }
    }

    fn parse_generic_number(expr: &Expression) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn unsigned_one() -> syn::Expr {
        todo!()
    }

    fn words_to_number(words: Vec<u64>, ty: &syn::Type) -> syn::Expr {
        todo!()
    }
}

impl StringParser for SorobanParser {
    fn parse_string(string: &str) -> ParserResult<syn::Expr> {
        Ok(code::expr::string_from(string))
    }
}

impl TypeParser for SorobanParser {
    fn parse_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> ParserResult<syn::Type> {
        match ty {
            Type::Address => Ok(code::ty::option(code::ty::address())),
            Type::Bool => Ok(code::ty::bool()),
            Type::String => Ok(code::ty::string()),
            Type::Int(size) => Ok(code::ty::int(*size)),
            Type::Uint(size) => Ok(code::ty::uint(*size)),
            Type::Bytes(_) => Ok(code::ty::bytes()),
            Type::Mapping(_, _) => Err(ParserError::InvalidType),
            Type::Custom(_) => todo!(),
            Type::Array(_) => todo!(),
            Type::Unknown => todo!(),
        }
    }

    fn parse_fixed_bytes(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn parse_serialize(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn parse_state_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> ParserResult<syn::Type> {
        todo!()
    }
}

fn to_array(values: &[u64], size: u16) -> Vec<u8> {
    let size = round_up_num_size(size) / 8;

    let mut vec = Vec::<u8>::with_capacity(size as usize);
    for (i, v) in values.iter().enumerate() {
        v.to_le_bytes().iter().enumerate().for_each(|(j, v)| {
            let idx = i * 8 + j;
            if idx >= vec.capacity() {
                return;
            }
            vec.insert(i * 8 + j, *v);
        });
    }
    vec
}

fn round_up_num_size(size: u16) -> u16 {
    let mut num = 32;
    while num < size && num < 256 {
        num *= 2;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_array() {
        let values = [1000];
        let expected: [u8; 4] = [232, 3, 0, 0];
        assert_eq!(to_array(&values, 17), expected.to_vec());
    }
}
