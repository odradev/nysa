pub fn contract() -> syn::Attribute {
    syn::parse_quote!(#[soroban_sdk::contract])
}

pub fn contractimpl() -> syn::Attribute {
    syn::parse_quote!(#[soroban_sdk::contractimpl])
}

pub fn contracttype() -> syn::Attribute {
    syn::parse_quote!(#[soroban_sdk::contracttype])
}
