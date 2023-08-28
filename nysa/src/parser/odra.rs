use crate::model::{
    ir::{NysaExpression, NysaVar},
    ContractData,
};
use c3_lang_parser::c3_ast::{ClassDef, PackageDef};
use quote::format_ident;
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};
use syn::parse_quote;

use self::context::Context;

use super::Parser;

mod context;
mod errors;
mod event;
mod expr;
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

    static ref DEFAULT_VARIABLES: Mutex<HashMap<NysaVar, NysaExpression>> = Mutex::new(HashMap::new());
}

pub struct OdraParser;

impl Parser for OdraParser {
    fn parse(data: ContractData) -> PackageDef {
        let class_name = data.c3_class_name_def();
        let storage = data.vars();

        let mut ctx = Context::default();
        ctx.set_storage(&storage);
        ctx.set_classes(data.contract_names().to_vec());

        let mut classes = vec![];
        classes.extend(event::events_def(&data));
        classes.push(contract_def(&data, &mut ctx));
        PackageDef {
            attrs: other::attrs(),
            other_code: other::other_code(&data),
            class_name,
            classes,
        }
    }
}

/// Builds a c3 contract class definition
fn contract_def(data: &ContractData, ctx: &mut Context) -> ClassDef {
    let variables = var::variables_def(data, ctx);
    let functions = func::functions_def(data, ctx);

    let events = data
        .events()
        .iter()
        .map(|ev| format_ident!("{}", ev.name))
        .collect::<Vec<_>>();
    let struct_attrs = match events.len() {
        0 => vec![parse_quote!(#[odra::module])],
        _ => vec![parse_quote!(#[odra::module(events = [ #(#events),* ])])],
    };

    ClassDef {
        struct_attrs,
        impl_attrs: vec![parse_quote!(#[odra::module])],
        class: data.c3_class(),
        path: data.c3_path(),
        variables,
        functions,
    }
}

#[cfg(test)]
mod test;