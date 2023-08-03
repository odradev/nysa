use std::vec;

use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassDef, ClassFnImpl, FnDef, PlainFnDef, VarDef};
use quote::quote;
use solidity_parser::pt::{EventDefinition, SourceUnitPart};
use syn::parse_quote;

use crate::{ty, utils};

pub(crate) fn events_def(ast: &[SourceUnitPart]) -> Vec<ClassDef> {
    let events = utils::extract_events(ast);

    events.iter().map(|ev| event_def(ev)).collect::<Vec<_>>()
}

fn event_def(ev: &EventDefinition) -> ClassDef {
    let class: Class = ev.name.name.clone().into();
    let path = vec![class.clone()];
    let variables = ev
        .fields
        .iter()
        .map(|f| {
            let field_name = &f.name.as_ref().expect("Event field must be named").name;
            let ident = utils::to_snake_case_ident(&field_name);
            let ty = ty::parse_plain_type_from_expr(&f.ty);
            VarDef { ident, ty }
        })
        .collect();

    let args = ev
        .fields
        .iter()
        .map(|f| {
            let field_name = &f.name.as_ref().expect("Event field must be named").name;
            let ident = utils::to_snake_case_ident(&field_name);
            let ty = ty::parse_plain_type_from_expr(&f.ty);
            parse_quote!(#ident: #ty)
        })
        .collect::<Vec<_>>();

    let assign = ev
        .fields
        .iter()
        .map(|f| {
            let field_name = &f.name.as_ref().expect("Event field must be named").name;
            let ident = utils::to_snake_case_ident(&field_name);
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
