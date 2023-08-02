use std::vec;

use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassDef, VarDef};
use solidity_parser::pt::{ContractDefinition, ContractPart, EventDefinition};
use syn::parse_quote;

use crate::{class, utils, ty};

pub(crate) fn events_def(contract: &ContractDefinition) -> Vec<ClassDef> {
    let class: Class = class(contract);

    contract
        .parts
        .iter()
        .filter_map(|part| match part {
            ContractPart::EventDefinition(ev) => Some(ev),
            _ => None,
        })
        .map(|ev| event_def(ev, class.clone()))
        .collect::<Vec<_>>()
}

fn event_def(ev: &EventDefinition, contract: Class) -> ClassDef {
    let variables = ev.fields.iter().map(|f| {
        let field_name = &f.name.as_ref().expect("Event field must be named").name;
        let ident = utils::to_snake_case_ident(&field_name);
        let ty = ty::parse_plain_type_from_expr(&f.ty);
        VarDef { ident, ty }
    }).collect();
    ClassDef { 
        struct_attrs: vec![parse_quote!(#[derive(odra::Event, PartialEq, Eq, Debug)])], 
        impl_attrs: vec![], 
        class: ev.name.name.clone().into(), 
        path: vec![contract], 
        variables, 
        functions: vec![]
    }
}