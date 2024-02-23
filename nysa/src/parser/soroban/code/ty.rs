use syn::parse_quote;

pub fn address() -> syn::Type {
    parse_quote!(soroban_sdk::Address)
}

pub fn option(ty: syn::Type) -> syn::Type {
    parse_quote!(Option<#ty>)
}

pub fn bool() -> syn::Type {
    parse_quote!(bool)
}

pub fn symbol() -> syn::Type {
    parse_quote!(soroban_sdk::Symbol)
}

pub fn string() -> syn::Type {
    parse_quote!(soroban_sdk::String)
}

pub fn env() -> syn::Type {
    parse_quote!(soroban_sdk::Env)
}

pub fn uint(size: u16) -> syn::Type {
    match round_up_num_size(size) {
        256 => parse_quote!(soroban_sdk::U256),
        128 => parse_quote!(u128),
        64 => parse_quote!(u64),
        32 => parse_quote!(u32),
        16 => parse_quote!(u32),
        8 => parse_quote!(u32),
        _ => panic!("Invalid int size: {}", size),
    }
}

pub fn int(size: u16) -> syn::Type {
    match round_up_num_size(size) {
        256 => parse_quote!(soroban_sdk::I256),
        128 => parse_quote!(i128),
        64 => parse_quote!(i64),
        32 => parse_quote!(i32),
        16 => parse_quote!(i32),
        8 => parse_quote!(i32),
        _ => panic!("Invalid int size: {}", size),
    }
}

pub fn u256() -> syn::Type {
    parse_quote!(soroban_sdk::U256)
}

pub fn bytes() -> syn::Type {
    parse_quote!(soroban_sdk::Bytes)
}

fn round_up_num_size(size: u16) -> u16 {
    let mut num = 8;
    while num < size && num < 256 {
        num *= 2;
    }
    num
}
