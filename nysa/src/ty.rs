use solidity_parser::pt;
use syn::parse_quote;

/// Parses solidity statement into a syn type.
///
/// Panics if the input is an expression of type other than [pt::Expression::Type].
pub fn parse_type_from_expr(ty: &pt::Expression) -> syn::Type {
    match ty {
        pt::Expression::Type(_, ty) => parse_type(ty),
        _ => panic!("Not a type. {:?}", ty),
    }
}

/// Parses solidity type into a syn type (plain rust type or near type).
fn parse_type(ty: &pt::Type) -> syn::Type {
    match ty {
        pt::Type::Mapping(_, key, value) => {
            let key = parse_type_from_expr(key);
            let value = parse_type_from_expr(value);
            parse_quote! {
                std::collections::HashMap<#key, #value>
            }
        }
        pt::Type::Address => parse_quote!(near_sdk::AccountId),
        pt::Type::AddressPayable => parse_quote!(near_sdk::AccountId),
        pt::Type::String => parse_quote!(String),
        pt::Type::Bool => parse_quote!(bool),
        pt::Type::Int(_) => parse_quote!(i16),
        pt::Type::Uint(_) => parse_quote!(u32),
        _ => panic!("Unsupported type."),
    }
}
