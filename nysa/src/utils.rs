use convert_case::{Case, Casing};
use quote::format_ident;

pub fn to_snake_case_ident(name: &str) -> proc_macro2::Ident {
    format_ident!("{}", name.to_case(Case::Snake))
}
