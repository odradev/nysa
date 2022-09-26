use solidity_parser::pt;
use syn::parse_quote;

pub fn parse_type_from_expr(ty: &pt::Expression) -> syn::Type {
    match ty {
        pt::Expression::Type(_, ty) => parse_type(ty),
        _ => panic!("Not a type. {:?}", ty),
    }
}

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
