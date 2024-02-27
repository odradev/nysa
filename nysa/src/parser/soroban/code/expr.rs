use syn::parse_quote;

use crate::parser::soroban::code::ty;

pub fn storage_instance() -> syn::Expr {
    parse_quote!(env.storage().instance())
}

pub fn auth_caller() -> syn::Expr {
    parse_quote!(caller.expect("`caller` must not be `None`").require_auth())
}

pub fn cloned_env() -> syn::Expr {
    parse_quote!(env.clone())
}

pub fn cloned_caller() -> syn::Expr {
    parse_quote!(caller.clone())
}

pub fn string_from(string: &str) -> syn::Expr {
    let ty = ty::string();
    parse_quote!(#ty::from_str(&env, #string))
}

pub fn u256_from_le_bytes(values: &[u64]) -> syn::Expr {
    let mut _values = [0u64; 4];
    for (i, v) in values.iter().enumerate() {
        _values[i] = *v;
    }
    let hi_hi = _values[3];
    let hi_lo = _values[2];
    let lo_hi = _values[1];
    let lo_lo = _values[0];
    parse_quote!(soroban_sdk::U256::from_parts(&env, #hi_hi, #hi_lo, #lo_hi, #lo_lo))
}

pub fn from_le_bytes(ty: syn::Type, size: u16, values: &[u64]) -> syn::Expr {
    let bytes = to_array(values, size);
    let arr = bytes
        .iter()
        .map(|v| quote::quote!(#v))
        .collect::<syn::punctuated::Punctuated<_, syn::Token![,]>>();
    parse_quote!(#ty::from_le_bytes([#arr]))
}

pub fn default_u256() -> syn::Expr {
    let ty = ty::u256();
    parse_quote!(#ty::from_u32(&env, 0))
}

pub fn ret(expr: Option<syn::Expr>) -> syn::Stmt {
    match expr {
        Some(expr) => parse_quote!(return Ok(#expr);),
        None => {
            let expr: syn::Expr = parse_quote!(Ok(()));
            syn::Stmt::Expr(expr)
        }
    }
}

pub fn from_contract_error(error_num: u32) -> syn::Expr {
    parse_quote!(return Err(soroban_sdk::Error::from_contract_error(#error_num)))
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

pub mod fn_arg {
    use syn::parse_quote;

    use crate::parser::soroban::code::ty;

    pub fn env() -> syn::FnArg {
        let ty = ty::env();
        parse_quote!(env: soroban_sdk::Env)
    }

    pub fn caller() -> syn::FnArg {
        let ty = ty::option(ty::address());
        parse_quote!(caller: #ty)
    }
}
