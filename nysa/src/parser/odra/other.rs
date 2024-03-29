use syn::parse_quote;

use crate::{
    parser::context::{ExternalCallsRegister, TypeInfo},
    utils,
};

/// File level attributes to mute error while compiling a contract.
/// Generating code from Solidity may result in some unusual naming conventions
/// and syntax that linter does not like.
pub(super) fn attrs() -> Vec<syn::Attribute> {
    vec![
        parse_quote!(#![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]),
    ]
}

/// Generates code that is not a direct derivative of Solidity code.
pub(super) fn other_code() -> Vec<syn::Item> {
    path_stack_default_impl()
}

/// Generates code that is not a direct derivative of Solidity code.
pub(super) fn imports_code<T: ExternalCallsRegister + TypeInfo>(ctx: &T) -> Vec<syn::Item> {
    ctx.get_external_calls()
        .iter()
        .map(|class| {
            let ident = utils::to_snake_case_ident(class);
            parse_quote!(use super::#ident::*;)
        })
        .chain(if ctx.has_enums() {
            vec![parse_quote!(
                use super::enums::*;
            )]
        } else {
            vec![]
        })
        .chain(vec![
            parse_quote!(
                use super::errors::*;
            ),
            parse_quote!(
                use super::events::*;
            ),
            parse_quote!(
                use super::structs::*;
            ),
            parse_quote!(
                use odra::prelude::*;
            ),
        ])
        .collect()
}

/// Generates Odra-specific implementations for PathStack.
pub(super) fn path_stack_default_impl() -> Vec<syn::Item> {
    vec![
        parse_quote! {
            impl odra::module::ModuleComponent for PathStack {
                fn instance(_env: Rc<odra::ContractEnv>, _index: u8) -> Self {
                    Self::default()
                }
            }
        },
        parse_quote! {
            impl odra::module::ModulePrimitive for PathStack {
            }
        },
    ]
}
