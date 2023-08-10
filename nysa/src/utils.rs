use convert_case::{Case, Casing};
use quote::format_ident;

pub mod ast;
pub mod func;

/// Converts a &str into snake-cased Ident preserving the heading `_`
pub fn to_snake_case_ident(name: &str) -> proc_macro2::Ident {
    format_ident!("{}", to_snake_case(name))
}

/// Converts a &str into snake-cased Ident starting with a `prefix ` and preserving the heading `_`
pub fn to_prefixed_snake_case_ident(prefix: &str, name: &str) -> proc_macro2::Ident {
    format_ident!("{}{}", prefix, to_snake_case(name))
}

/// Converts a &str into snake-cased String preserving the heading `_`
pub fn to_snake_case(input: &str) -> String {
    if input.starts_with('_') {
        // `to_case()` consumes the heading `_`
        format!("_{}", input.to_case(Case::Snake))
    } else {
        format!("{}", input.to_case(Case::Snake))
    }
}

/// Converts a vec to to an array. The vec length must match the array length.
pub fn convert_to_array<const T: usize>(input: &Vec<u8>) -> [u8; T] {
    let mut array: [u8; T] = [0; T];
    array.copy_from_slice(&input[..T]);
    array
}

pub fn map_collection<'a, T, R>(collection: Vec<T>) -> Vec<R>
where
    R: for<'b> From<&'b T>,
{
    collection.iter().map(|item| R::from(item)).collect()
}

#[cfg(test)]
mod t {
    use crate::utils::to_snake_case_ident;
    use proc_macro2::{Ident, Span};

    #[test]
    fn to_snake_case_ident_works() {
        assert_eq!(ident("my_value"), to_snake_case_ident("MyValue"));
        assert_eq!(ident("value"), to_snake_case_ident("value"));
        assert_eq!(ident("value"), to_snake_case_ident("Value"));
        assert_eq!(ident("_value"), to_snake_case_ident("_value"));
    }

    fn ident(string: &str) -> Ident {
        Ident::new(string, Span::call_site())
    }
}
