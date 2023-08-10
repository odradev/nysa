use solidity_parser::pt;
use syn::parse_quote;

/// Parses solidity statement into a syn type.
///
/// Panics if the input is an expression of type other than [pt::Expression::Type].
pub fn parse_type_from_expr(ty: &pt::Expression) -> syn::Type {
    match ty {
        pt::Expression::Type(_, ty) => parse_odra_type(ty),
        _ => panic!("Not a type. {:?}", ty),
    }
}

/// Parses solidity type into a syn type (plain rust type or near type).
fn parse_odra_type(ty: &pt::Type) -> syn::Type {
    match ty {
        pt::Type::Mapping(_, key, value) => {
            let key = parse_plain_type_from_expr(key);
            let value = parse_plain_type_from_expr(value);
            parse_quote!(odra::Mapping<#key, #value>)
        }
        pt::Type::Address => parse_quote!(odra::Variable<Option<odra::types::Address>>),
        pt::Type::AddressPayable => parse_quote!(odra::Variable<Option<odra::types::Address>>),
        pt::Type::String => parse_quote!(odra::Variable<String>),
        pt::Type::Bool => parse_quote!(odra::Variable<bool>),
        pt::Type::Int(_) => parse_quote!(odra::Variable<i16>),
        pt::Type::Uint(size) => match size {
            8 => parse_quote!(odra::Variable<u8>),
            16 => parse_quote!(odra::Variable<u16>),
            32 => parse_quote!(odra::Variable<u32>),
            64 => parse_quote!(odra::Variable<u64>),
            128 => parse_quote!(odra::Variable<odra::types::U128>),
            256 => parse_quote!(odra::Variable<odra::types::U256>),
            512 => parse_quote!(odra::Variable<odra::types::U512>),
            _ => panic!("Unsupported unit {}.", size),
        },
        _ => panic!("Unsupported type."),
    }
}

pub fn parse_plain_type_from_expr(expr: &pt::Expression) -> syn::Type {
    match expr {
        pt::Expression::Type(_, ty) => parse_plain_type_from_ty(ty),
        _ => panic!("Not a type. {:?}", expr),
    }
}

pub fn parse_plain_type_from_ty(ty: &pt::Type) -> syn::Type {
    match ty {
        pt::Type::Address => parse_quote!(Option<odra::types::Address>),
        pt::Type::AddressPayable => parse_quote!(Option<odra::types::Address>),
        pt::Type::String => parse_quote!(String),
        pt::Type::Bool => parse_quote!(bool),
        pt::Type::Int(_) => parse_quote!(i16),
        pt::Type::Uint(size) => match size {
            8 => parse_quote!(u8),
            16 => parse_quote!(u16),
            32 => parse_quote!(u32),
            64 => parse_quote!(u64),
            128 => parse_quote!(odra::types::U128),
            256 => parse_quote!(odra::types::U256),
            512 => parse_quote!(odra::types::U512),
            _ => panic!("Unsupported unit {}.", size),
        },
        pt::Type::Mapping(_, key, value) => {
            let key = parse_plain_type_from_expr(key);
            let value = parse_plain_type_from_expr(value);
            parse_quote!(odra::Mapping<#key, #value>)
        }
        _ => panic!("Unsupported type {:?}.", ty),
    }
}
