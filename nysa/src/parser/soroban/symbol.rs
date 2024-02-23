use proc_macro2::TokenStream;
use quote::quote;

use crate::error::ParserResult;
use crate::model::ir::{Expression, Type};
use crate::parser::common::ty::parse_type_from_expr;
use crate::parser::context::{ContractInfo, TypeInfo};
use crate::parser::soroban::code;
use crate::SorobanParser;

pub fn symbols_def<T: TypeInfo + ContractInfo>(ctx: &mut T) -> Vec<syn::Item> {
    ctx.current_contract()
        .vars()
        .iter()
        .filter(|v| !v.is_immutable)
        .map(|v| match &v.ty {
            Type::Mapping(key, value) => {
                let key = parse_type_from_expr::<_, SorobanParser>(key, ctx).unwrap();
                let (key, _) = compose_key(vec![key], value, ctx).unwrap();
                let key = key.iter().map(|k| quote!(pub #k,)).collect::<TokenStream>();
                code::consts::contract_type(&v.name, key)
            }
            _ => code::consts::short_symbol(&v.name),
        })
        .collect()
}

fn compose_key<T: TypeInfo>(
    parts: Vec<syn::Type>,
    value: &Expression,
    ctx: &T,
) -> ParserResult<(Vec<syn::Type>, syn::Type)> {
    if let Expression::Type(Type::Mapping(key, value)) = value {
        let key = parse_type_from_expr::<_, SorobanParser>(key, ctx)?;
        compose_key(parts.into_iter().chain(vec![key]).collect(), value, ctx)
    } else {
        Ok((
            parts.to_vec(),
            parse_type_from_expr::<_, SorobanParser>(value, ctx)?,
        ))
    }
}
