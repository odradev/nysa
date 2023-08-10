use std::vec;

use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassDef, ClassFnImpl, FnDef, PlainFnDef, VarDef};
use quote::quote;
use syn::parse_quote;

use crate::{
    model::{ir::NysaEvent, ContractData},
    utils,
};

use super::ty;

pub(crate) fn events_def(data: &ContractData) -> Vec<ClassDef> {
    data.events()
        .iter()
        .map(|ev| event_def(ev))
        .collect::<Vec<_>>()
}

fn event_def(ev: &NysaEvent) -> ClassDef {
    let class: Class = ev.name.clone().into();
    let path = vec![class.clone()];
    let variables = ev
        .fields
        .iter()
        .map(|(field_name, ty)| {
            let ident = utils::to_snake_case_ident(field_name);
            let ty = ty::parse_plain_type_from_expr(ty);
            VarDef { ident, ty }
        })
        .collect();

    let args = ev
        .fields
        .iter()
        .map(|(field_name, ty)| {
            let ident = utils::to_snake_case_ident(field_name);
            let ty = ty::parse_plain_type_from_expr(ty);
            parse_quote!(#ident: #ty)
        })
        .collect::<Vec<_>>();

    let assign = ev
        .fields
        .iter()
        .map(|(field_name, ty_)| {
            let ident = utils::to_snake_case_ident(field_name);
            quote!(#ident)
        })
        .collect::<Vec<_>>();

    ClassDef {
        struct_attrs: vec![parse_quote!(#[derive(odra::Event, PartialEq, Eq, Debug)])],
        impl_attrs: vec![],
        class,
        path: vec![],
        variables,
        functions: vec![FnDef::Plain(PlainFnDef {
            attrs: vec![],
            name: "new".into(),
            args,
            ret: parse_quote!(-> Self),
            implementation: ClassFnImpl {
                visibility: parse_quote!(pub),
                class: None,
                fun: "new".into(),
                implementation: parse_quote!({
                    Self {
                        #(#assign),*
                    }
                }),
            },
        })],
    }
}
