use convert_case::{Case, Casing};
use quote::format_ident;
use solidity_parser::pt::SourceUnitPart;

pub(crate) mod ast;

pub type SolidityAST = Vec<SourceUnitPart>;

/// Converts a &str into snake-cased Ident preserving the heading `_`
pub(crate) fn to_snake_case_ident<T: AsRef<str>>(name: T) -> proc_macro2::Ident {
    format_ident!("{}", to_snake_case(name.as_ref()))
}

/// Converts a &str into pascal-cased Ident.
pub(crate) fn to_pascal_case_ident<T: AsRef<str>>(name: T) -> proc_macro2::Ident {
    format_ident!("{}", name.as_ref().to_case(Case::Pascal))
}

/// Converts a &str into upper-snake-cased Ident preserving the heading `_`
pub(crate) fn to_upper_snake_case_ident<T: AsRef<str>>(name: T) -> proc_macro2::Ident {
    format_ident!("{}", to_upper_snake_case(name.as_ref()))
}

/// Converts a &str into snake-cased Ident starting with a `prefix ` and preserving the heading `_`
pub(crate) fn to_prefixed_snake_case_ident<T: AsRef<str>>(
    prefix: &str,
    name: T,
) -> proc_macro2::Ident {
    format_ident!("{}{}", prefix, to_snake_case(name.as_ref()))
}

/// Converts a &str into snake-cased String preserving the heading `_`
pub(crate) fn to_snake_case(input: &str) -> String {
    if input.starts_with('_') {
        // `to_case()` consumes the heading `_`
        format!("_{}", input.to_case(Case::Snake))
    } else {
        format!("{}", input.to_case(Case::Snake))
    }
}

/// Converts a &str into upper-snake-cased String preserving the heading `_`
pub(crate) fn to_upper_snake_case(input: &str) -> String {
    if input.starts_with('_') {
        // `to_case()` consumes the heading `_`
        format!("_{}", input.to_case(Case::UpperSnake))
    } else {
        format!("{}", input.to_case(Case::UpperSnake))
    }
}

/// Converts an `input` into [Ident](proc_macro2::Ident).
pub(crate) fn to_ident<T: AsRef<str>>(input: T) -> proc_macro2::Ident {
    format_ident!("{}", input.as_ref())
}

pub(crate) fn to_ref_ident<T: AsRef<str>>(input: T) -> proc_macro2::Ident {
    format_ident!("{}ContractRef", input.as_ref())
}

/// Converts a vec to to an array. The vec length must match the array length.
pub(crate) fn convert_to_array<const T: usize>(input: &[u8]) -> [u8; T] {
    let mut array: [u8; T] = [0; T];
    array.copy_from_slice(&input[..T]);
    array
}

/// Converts a vec of one type into a vec of another type.
pub(crate) fn map_collection<'a, T, R>(collection: Vec<T>) -> Vec<R>
where
    R: for<'b> From<&'b T>,
{
    collection.iter().map(R::from).collect()
}

/// A type that can be represented as a vector of strings
pub trait AsStringVec {
    fn as_string_vec(&self) -> Vec<String>;
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
