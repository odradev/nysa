use std::vec;

use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassDef, ClassFnImpl, FnDef, PlainFnDef, VarDef};
use quote::quote;
use syn::parse_quote;

use crate::{
    model::ir::{Event, Package},
    parser::context::TypeInfo,
    utils, ParserError,
};

use super::{syn_utils::attr, ty};

pub(crate) fn events_def<T: TypeInfo>(
    package: &Package,
    ctx: &T,
) -> Result<Vec<ClassDef>, ParserError> {
    package
        .events()
        .iter()
        .map(|ev| event_def(ev, ctx))
        .collect()
}

fn event_def<T: TypeInfo>(ev: &Event, ctx: &T) -> Result<ClassDef, ParserError> {
    let class: Class = ev.name.clone().into();
    let path = vec![class.clone()];
    let variables = ev
        .fields
        .iter()
        .map(|(field_name, ty)| {
            let ident = utils::to_snake_case_ident(field_name);
            let ty = ty::parse_type_from_expr(ty, ctx)?;
            Ok(VarDef { ident, ty })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let args = ev
        .fields
        .iter()
        .map(|(field_name, ty)| {
            let ident = utils::to_snake_case_ident(field_name);
            let ty = ty::parse_type_from_expr(ty, ctx)?;
            Ok(parse_quote!(#ident: #ty))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let assign = ev
        .fields
        .iter()
        .map(|(field_name, ty_)| {
            let ident = utils::to_snake_case_ident(field_name);
            quote!(#ident)
        })
        .collect::<Vec<_>>();

    Ok(ClassDef {
        struct_attrs: vec![attr::derive_odra_event()],
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
