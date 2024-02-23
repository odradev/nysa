use std::vec;

use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassDef, ClassFnImpl, FnDef, PlainFnDef, VarDef};
use syn::parse_quote;

use crate::{
    error::ParserResult,
    model::ir::{Event, Package},
    parser::{common::ty::parse_type_from_expr, context::TypeInfo, syn_utils},
    utils,
};

use super::{EventParser, TypeParser};

pub(crate) fn events_def<T: TypeInfo, P: TypeParser + EventParser>(
    package: &Package,
    ctx: &T,
) -> ParserResult<Vec<ClassDef>> {
    package
        .events()
        .iter()
        .map(|ev| event_def::<_, P>(ev, ctx))
        .collect()
}

fn event_def<T: TypeInfo, P: TypeParser + EventParser>(
    ev: &Event,
    ctx: &T,
) -> ParserResult<ClassDef> {
    let class: Class = ev.name.clone().into();
    let path = vec![class.clone()];
    let variables = ev
        .fields
        .iter()
        .map(|(field_name, ty)| {
            let ident = utils::to_snake_case_ident(field_name);
            let ty = parse_type_from_expr::<_, P>(ty, ctx)?;
            Ok(VarDef { ident, ty })
        })
        .collect::<ParserResult<Vec<_>>>()?;

    let args = ev
        .fields
        .iter()
        .map(|(field_name, ty)| {
            let ident = utils::to_snake_case_ident(field_name);
            let ty = parse_type_from_expr::<_, P>(ty, ctx)?;
            Ok(syn_utils::fn_arg(ident, ty))
        })
        .collect::<ParserResult<Vec<_>>>()?;

    let assign = ev
        .fields
        .iter()
        .map(|(field_name, ty_)| utils::to_snake_case_ident(field_name))
        .collect::<Vec<_>>();

    Ok(ClassDef {
        struct_attrs: P::derive_attrs(),
        impl_attrs: vec![],
        class,
        path: vec![],
        variables,
        other_items: vec![],
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
    })
}
