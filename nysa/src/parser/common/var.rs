use c3_lang_parser::c3_ast::VarDef;
use syn::parse_quote;

use crate::{
    error::ParserResult,
    model::ir::{Expression, Type, Var},
    parser::{
        context::{ContractInfo, TypeInfo},
        syn_utils::AsType,
    },
    utils, ParserError,
};

use super::{expr, NumberParser, TypeParser};

/// Pareses mutable [Var]s into a vector of c3 ast [VarDef].
pub fn variables_def<T: TypeInfo + ContractInfo, P: TypeParser>(
    t: &mut T,
) -> ParserResult<Vec<VarDef>> {
    t.current_contract()
        .vars()
        .iter()
        .filter(|v| !v.is_immutable)
        .map(|v| variable_def::<_, P>(v, t))
        .collect()
}

/// Pareses immutable [Var]s into a vector of c3 ast [VarDef].
pub fn const_def<T: TypeInfo + ContractInfo, P: TypeParser + NumberParser>(
    ctx: &mut T,
) -> ParserResult<Vec<syn::Item>> {
    ctx.current_contract()
        .vars()
        .iter()
        .filter(|v| v.is_immutable)
        .map(|v| {
            let const_ident = utils::to_ident(&v.name);

            let ty = P::parse_ty(&v.ty, ctx)?;
            let expr = v
                .initializer
                .as_ref()
                .expect("A const must be initialized.");
            match expr {
                Expression::BoolLiteral(v) => Ok(parse_quote!(pub const #const_ident: bool = #v;)),
                Expression::StringLiteral(s) => {
                    Ok(parse_quote!(pub const #const_ident: &str = #s;))
                }
                Expression::NumberLiteral(n) => {
                    if let Type::Uint(size) | Type::Int(size) = v.ty {
                        let words = to_sized_u64_words(n, size.div_ceil(64) as usize);
                        let num = P::words_to_number(words, &ty);
                        Ok(parse_quote!(pub const #const_ident: #ty = #num;))
                    } else {
                        Err(ParserError::InvalidType)
                    }
                }
                Expression::BytesLiteral(bytes) => {
                    if let Type::Uint(size) | Type::Int(size) = v.ty {
                        let bytes = bytes.iter().rev().map(|u| *u).collect::<Vec<_>>();
                        let words = to_bytes_u64_words(&bytes, size.div_ceil(64) as usize);
                        let num = P::words_to_number(words, &ty);
                        Ok(parse_quote!(pub const #const_ident: #ty = #num;))
                    } else if let Type::Bytes(b) = v.ty {
                        let value = expr::parse_bytes_lit(bytes)?;
                        Ok(parse_quote!(pub const #const_ident: #ty = #value;))
                    } else {
                        Err(ParserError::InvalidType)
                    }
                }
                Expression::ArrayLiteral(_) => todo!(),
                _ => todo!(),
            }
        })
        .collect()
}

/// Transforms [Var] into a c3 ast [VarDef].
fn variable_def<T: TypeInfo, P: TypeParser>(v: &Var, t: &T) -> ParserResult<VarDef> {
    let ident = utils::to_snake_case_ident(&v.name);
    let ty = P::parse_state_ty(&v.ty, t)?.as_type();
    Ok(VarDef { ident, ty })
}

fn to_bytes_u64_words(input: &[u8], size: usize) -> Vec<u64> {
    let mut output = vec![0; size];
    let mut idx = 0;
    let mut start = 0;

    while start < input.len() {
        let end = std::cmp::min(start + 8, input.len());

        let mut bytes = [0u8; 8];
        for i in start..end {
            bytes[i % 8] = input[i];
        }
        output[idx] = u64::from_le_bytes(bytes);

        start = end;
        idx += 1;
    }
    output
}

fn to_sized_u64_words(input: &[u64], size: usize) -> Vec<u64> {
    let mut output = vec![0; size];
    for i in 0..input.len() {
        output[i] = input[i];
    }
    output
}
