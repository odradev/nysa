use proc_macro2::Ident;
use quote::format_ident;

pub fn env() -> Ident {
    format_ident!("env")
}
