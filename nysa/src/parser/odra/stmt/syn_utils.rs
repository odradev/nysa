use proc_macro2::Ident;
use quote::ToTokens;
use syn::parse_quote;

use crate::parser::odra::expr;

pub fn definition(ident: Ident, expr: syn::Expr) -> syn::Stmt {
    parse_quote!(let mut #ident = #expr;)
}

pub fn ret(expr: Option<syn::Expr>) -> syn::Stmt {
    parse_quote!(return #expr;)
}

pub fn contract_ref(var_name: &str, contract_name: &str) -> syn::Stmt {
    let ident = crate::utils::to_ident(var_name);
    let contract_ref = expr::syn_utils::contract_ref(contract_name, &ident);
    definition(ident, contract_ref)
}

pub fn emit_event<T: ToTokens>(event_ident: Ident, args: &[T]) -> syn::Stmt {
    parse_quote!(self.env().emit_event(#event_ident::new(#(#args),*));)
}

pub fn while_loop<T: ToTokens>(assertion: syn::Expr, block: T) -> syn::Stmt {
    parse_quote!(while #assertion #block)
}

pub fn if_stmt<T: ToTokens>(assertion: syn::Expr, body: T) -> syn::Stmt {
    parse_quote!(if #assertion #body)
}

pub fn if_else_stmt<T: ToTokens, F: ToTokens>(
    assertion: syn::Expr,
    if_body: T,
    else_body: F,
) -> syn::Stmt {
    parse_quote!(if #assertion #if_body else #else_body)
}
