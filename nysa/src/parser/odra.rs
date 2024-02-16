use crate::{model::ir::Package, utils, ParserError};
use c3_lang_parser::c3_ast::{ClassDef, PackageDef};
use proc_macro2::TokenStream;

use self::syn_utils::attr;

use super::{
    common::ContractReferenceParser,
    context::{ContractContext, ContractInfo, EventsRegister, GlobalContext, LocalContext},
    Parser,
};

mod custom;
mod errors;
mod event;
mod expr;
mod ext;
mod func;
mod other;
pub(crate) mod stmt;
mod syn_utils;
mod ty;
mod var;

/// Implementation of [Parser]. Generates code compatible with the Odra Framework.
pub struct OdraParser;

impl Parser for OdraParser {
    type EventEmitParser = Self;
    type ContractReferenceParser = Self;
    type ContractErrorParser = Self;
    type ExpressionParser = Self;

    fn parse(package: Package) -> Result<TokenStream, ParserError> {
        // register all metadata in the global context.
        let mut ctx: GlobalContext = (&package).into();

        let events = event::events_def(&package, &ctx)?;
        let errors = errors::errors_def(&package);
        let enums = custom::enums_def(&package);
        let structs = custom::struct_def(&package, &ctx)?;
        let ext = ext::ext_contracts_def(&package, &ctx)?;

        let packages = parse_packages(&package, &mut ctx)?;

        let contracts = packages
            .iter()
            .map(|def| {
                let name = def.classes.first().as_ref().unwrap().class.to_string();
                let mod_name = utils::to_snake_case_ident(&name);
                quote::quote! {
                    pub mod #mod_name {
                        #def
                    }
                }
            })
            .collect::<TokenStream>();

        Ok(quote::quote! {
            pub mod errors {
                #errors
            }

            pub mod events {
                use odra::prelude::*;
                #(#events)*
            }

            pub mod enums {
                #(#enums)*
            }

            pub mod structs {
                #(#structs)*
            }

            #(#ext)*

            #contracts
        })
    }
}

fn parse_packages(
    package: &Package,
    ctx: &mut GlobalContext,
) -> Result<Vec<PackageDef>, ParserError> {
    package
        .contracts()
        .iter()
        .map(|data| {
            let class_name = data.c3_class_name_def();
            let storage = data
                .vars()
                .into_iter()
                .filter(|v| !v.is_immutable)
                .collect::<Vec<_>>();

            let mut ctx = LocalContext::new(ContractContext::new(ctx, data.clone()));

            let classes = vec![contract_def(&mut ctx)?];

            let mut other_code = vec![];
            other_code.extend(other::imports_code(&ctx));
            other_code.extend(other::other_code());

            Ok(PackageDef {
                no_std: true,
                attrs: other::attrs(),
                other_code,
                class_name,
                classes,
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

/// Builds a c3 contract class definition
fn contract_def(ctx: &mut LocalContext) -> Result<ClassDef, ParserError> {
    let variables = var::variables_def(ctx)?;
    let constants = var::const_def(ctx)?;
    let functions = func::functions_def(ctx)?;

    let events = ctx
        .emitted_events()
        .iter()
        .map(utils::to_ident)
        .collect::<Vec<_>>();
    let struct_attrs = match events.len() {
        0 => vec![attr::module()],
        _ => vec![attr::module_with_events(events)],
    };

    Ok(ClassDef {
        struct_attrs,
        impl_attrs: vec![attr::module()],
        class: ctx.current_contract().c3_class(),
        path: ctx.current_contract().c3_path(),
        variables,
        functions,
        other_items: constants,
    })
}

#[cfg(test)]
mod test;

impl ContractReferenceParser for OdraParser {
    fn parse_contract_ref(variable_name: &str, contract_name: &str) -> syn::Stmt {
        syn_utils::stmt::contract_ref(variable_name, contract_name)
    }

    fn parse_contract_ref_expr(contract_name: &str, address: syn::Expr) -> syn::Expr {
        expr::syn_utils::contract_ref(contract_name, address)
    }
}
