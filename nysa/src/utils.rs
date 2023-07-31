use convert_case::{Case, Casing};
use quote::format_ident;

pub fn to_snake_case_ident(name: &str) -> proc_macro2::Ident {
    format_ident!("{}", to_snake_case(name))
}

pub fn to_snake_case(input: &str) -> String {
    if input.starts_with('_') {
        // `to_case()` consumes the heading `_`
        format!("_{}", input.to_case(Case::Snake))
    } else {
        format!("{}", input.to_case(Case::Snake))
    }
}

#[cfg(test)]
mod t {
    use proc_macro2::{Ident, Span};
    use crate::utils::to_snake_case_ident;

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