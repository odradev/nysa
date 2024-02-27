use super::common::{self, func};
use super::{context::GlobalContext, Parser};
use crate::error::ParserResult;
use crate::parser::context::{ContractContext, ContractInfo, LocalContext};
use crate::{model::ir::Package, utils, ParserError};
use c3_lang_parser::c3_ast::{ClassDef, PackageDef};
use proc_macro2::TokenStream;

pub(crate) mod code;
mod parsers;
mod symbol;
#[cfg(test)]
mod test;

/// Implementation of [Parser]. Generates code compatible with the Soroban Framework.
pub struct SorobanParser;

impl Parser for SorobanParser {
    fn parse(package: Package) -> Result<TokenStream, ParserError> {
        // register all metadata in the global context.
        let mut ctx: GlobalContext = (&package).into();

        let events = common::event::events_def::<_, Self>(&package, &ctx)?;
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
            }

            pub mod events {
                #(#events)*
            }

            pub mod enums {
            }

            pub mod structs {
            }

            #contracts
        })
    }

    type EventEmitParser = Self;
    type ContractReferenceParser = Self;
    type ContractErrorParser = Self;
    type ExpressionParser = Self;
    type FnParser = Self;
    type TypeParser = Self;
    type ElementsParser = Self;
}

fn parse_packages(package: &Package, ctx: &mut GlobalContext) -> ParserResult<Vec<PackageDef>> {
    package
        .contracts()
        .iter()
        .map(|data| {
            let mut ctx = LocalContext::new(ContractContext::new(ctx, data.clone()));
            let class_name = data.c3_class_name_def();
            // let contract_def = contract_def(&mut ctx)?;

            let default_imports = common::other::imports_code::<_, SorobanParser>(&ctx);
            let storage_items = symbol::symbols_def(&mut ctx)
                .into_iter()
                .map(From::from)
                .collect();
            Ok(PackageDef {
                no_std: true,
                attrs: code::attr::module_attrs(),
                other_code: [default_imports, storage_items].concat(),
                class_name,
                classes: vec![contract_def(&mut ctx)?],
            })
        })
        .collect::<ParserResult<_>>()
}

fn contract_def(ctx: &mut LocalContext) -> ParserResult<ClassDef> {
    // let constants = var::const_def(ctx)?;
    let functions = func::functions_def::<_, SorobanParser>(ctx)?;

    Ok(ClassDef {
        struct_attrs: vec![code::attr::contract()],
        impl_attrs: vec![code::attr::contractimpl()],
        class: ctx.current_contract().c3_class(),
        path: ctx.current_contract().c3_path(),
        variables: vec![],
        functions,
        other_items: vec![],
    })
}
