use quote::ToTokens;
use syn::parse_quote;

use crate::{
    model::ir::Expression,
    parser::context::{
        ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
        TypeInfo,
    },
};

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
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
    F: FnOnce(&mut T) -> R,
{
    ctx.push_contextual_expr(ctx_expr.clone());
    let result = f(ctx);
    ctx.drop_contextual_expr();
    result
}

pub mod ty {
    use quote::ToTokens;
    use syn::parse_quote;

    pub fn var<T: ToTokens>(ty: T) -> syn::Type {
        parse_quote!(odra::Var<#ty>)
    }

    pub fn map<T: ToTokens, U: ToTokens>(key: T, value: U) -> syn::Type {
        parse_quote!(odra::Mapping<#key, #value>)
    }

    pub fn option<T: ToTokens>(ty: T) -> syn::Type {
        parse_quote!(Option<#ty>)
    }

    pub fn address() -> syn::Type {
        parse_quote!(odra::Address)
    }

    pub fn string() -> syn::Type {
        parse_quote!(odra::prelude::string::String)
    }

    pub fn bool() -> syn::Type {
        parse_quote!(bool)
    }

    pub fn vec<T: ToTokens>(ty: T) -> syn::Type {
        parse_quote!(odra::prelude::vec::Vec<#ty>)
    }

    pub fn fixed_bytes(size: usize) -> syn::Type {
        parse_quote!(nysa_types::FixedBytes<#size>)
    }

    pub fn u256() -> syn::Type {
        parse_quote!(nysa_types::U256)
    }
}

pub mod attr {
    use quote::ToTokens;

    pub fn module() -> syn::Attribute {
        syn::parse_quote!(#[odra::module])
    }

    pub fn module_with_events<T: ToTokens>(events: Vec<T>) -> syn::Attribute {
        syn::parse_quote!(#[odra::module(events = [ #(#events),* ])])
    }

    pub fn derive_odra_ty() -> syn::Attribute {
        syn::parse_quote!(#[derive(odra::OdraType, PartialEq, Eq, Debug, Default)])
    }

    pub fn derive_odra_err() -> syn::Attribute {
        syn::parse_quote!(#[derive(odra::OdraError, PartialEq, Eq, Debug)])
    }

    pub fn derive_odra_event() -> syn::Attribute {
        syn::parse_quote!(#[derive(odra::Event, PartialEq, Eq, Debug)])
    }

    pub fn default() -> syn::Attribute {
        syn::parse_quote!(#[default])
    }

    pub fn payable() -> syn::Attribute {
        syn::parse_quote!(#[odra(payable)])
    }
}
