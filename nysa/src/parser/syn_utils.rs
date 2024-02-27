use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse_quote;

use crate::model::ir::Expression;

use super::common::StatementParserContext;

pub fn fn_arg(name: Ident, ty: syn::Type) -> syn::FnArg {
    parse_quote!( #name: #ty )
}

pub fn self_arg(is_mut: bool) -> syn::FnArg {
    let mut_mod = is_mut.then(|| quote::quote!(mut));
    parse_quote!( &#mut_mod self )
}

pub fn default() -> syn::Expr {
    parse_quote!(Default::default())
}

pub fn none() -> syn::Expr {
    parse_quote!(None)
}

pub fn definition(ident: Ident, expr: syn::Expr) -> syn::Stmt {
    parse_quote!(let mut #ident = #expr;)
}

pub fn ret(expr: Option<syn::Expr>) -> syn::Stmt {
    match expr {
        Some(expr) => parse_quote!(return #expr;),
        None => {
            let e: syn::Expr = syn::Expr::Verbatim(TokenStream::new());
            syn::Stmt::Expr(e)
        }
    }
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

pub fn if_not<T: ToTokens, F: ToTokens>(condition: T, expr: F) -> syn::Expr {
    parse_quote!(if !(#condition) { #expr })
}

pub fn default_value(ident: Ident) -> TokenStream {
    quote::quote!(let mut #ident = Default::default();)
}

pub fn while_loop<T: ToTokens>(assertion: syn::Expr, block: T) -> syn::Stmt {
    parse_quote!(while #assertion #block)
}

pub fn as_ref(expr: Option<syn::Expr>) -> Option<syn::Expr> {
    expr.map(|k| parse_quote!(&#k))
}

pub trait AsSelfField {
    fn as_self_field(self) -> syn::Expr;
}

impl AsSelfField for syn::Ident {
    fn as_self_field(self) -> syn::Expr {
        parse_quote!(self.#self)
    }
}

pub trait AsExpression {
    fn as_expression(self) -> syn::Expr;
}

pub trait AsType {
    fn as_type(self) -> syn::Type;
}

pub trait AsStatement {
    fn as_statement(self) -> syn::Stmt;
}

impl<T: ToTokens> AsExpression for T {
    fn as_expression(self) -> syn::Expr {
        syn::parse_quote!(#self)
    }
}

impl<T: ToTokens> AsType for T {
    fn as_type(self) -> syn::Type {
        syn::parse_quote!(#self)
    }
}

impl AsStatement for syn::Expr {
    fn as_statement(self) -> syn::Stmt {
        syn::parse_quote!(#self;)
    }
}

pub fn in_context<T, F, R>(ctx_expr: &Expression, ctx: &mut T, f: F) -> R
where
    T: StatementParserContext,
    F: FnOnce(&mut T) -> R,
{
    ctx.push_contextual_expr(ctx_expr.clone());
    let result = f(ctx);
    ctx.drop_contextual_expr();
    result
}
