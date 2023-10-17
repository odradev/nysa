use crate::{
    model::{
        ir::{Expression, Package, Var},
        AsStringVec, ContractData,
    },
    utils, ParserError,
};
use c3_lang_parser::c3_ast::{ClassDef, PackageDef};
use proc_macro2::TokenStream;
use quote::format_ident;
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};
use syn::parse_quote;

use super::{
    context::{ContractContext, EventsRegister, GlobalContext, LocalContext},
    Parser,
};

mod custom;
mod errors;
mod event;
mod expr;
mod ext;
mod func;
mod other;
mod stmt;
mod ty;
mod var;

lazy_static::lazy_static! {
    static ref ERROR_MAP: Mutex<HashMap<String, u16>> = Mutex::new(HashMap::new());
    static ref ERRORS: Mutex<u16> = Mutex::new(0);

    static ref MSG_DATA: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    static ref SOLIDITY_ERRORS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());

    static ref DEFAULT_VARIABLES: Mutex<HashMap<Var, Expression>> = Mutex::new(HashMap::new());
}

/// Implementation of [Parser]. Generates code compatible with the Odra Framework.
pub struct OdraParser;

impl Parser for OdraParser {
    fn parse(package: Package) -> Result<TokenStream, ParserError> {
        let ctx = GlobalContext::new(
            package.events().as_string_vec(),
            package.interfaces().as_string_vec(),
            package.enums().as_string_vec(),
            package.errors().as_string_vec(),
            package.contracts().as_string_vec(),
            package.structs().as_string_vec(),
        );

        let events = event::events_def(&package, &ctx)?;
        let errors = errors::errors_def(&package);
        let enums = custom::enums_def(&package);
        let structs = custom::struct_def(&package, &ctx)?;
        let ext = ext::ext_contracts_def(&package, &ctx)?;

        let packages = parse_packages(&package, &ctx)?;

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

fn parse_packages(package: &Package, ctx: &GlobalContext) -> Result<Vec<PackageDef>, ParserError> {
    package
        .contracts()
        .iter()
        .map(|data| {
            let class_name = data.c3_class_name_def();
            let storage = data.vars();

            let mut ctx = LocalContext::new(ContractContext::new(ctx, &storage));

            let classes = vec![contract_def(&data, &mut ctx)?];

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
fn contract_def(data: &ContractData, ctx: &mut LocalContext) -> Result<ClassDef, ParserError> {
    let variables = var::variables_def(data, ctx)?;
    let functions = func::functions_def(data, ctx)?;

    let events = ctx
        .emitted_events()
        .iter()
        .map(|ev| format_ident!("{}", ev))
        .collect::<Vec<_>>();
    let struct_attrs = match events.len() {
        0 => vec![parse_quote!(#[odra::module])],
        _ => vec![parse_quote!(#[odra::module(events = [ #(#events),* ])])],
    };

    Ok(ClassDef {
        struct_attrs,
        impl_attrs: vec![parse_quote!(#[odra::module])],
        class: data.c3_class(),
        path: data.c3_path(),
        variables,
        functions,
    })
}

#[cfg(test)]
mod test;
