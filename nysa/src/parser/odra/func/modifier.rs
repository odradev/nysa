use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassFnImpl, FnDef, PlainFnDef};
use syn::parse_quote;

use crate::{model::ir::FnImplementations, parser::odra::context::Context, ParserError};

use super::common;

/// Splits a solidity modifier into two functions:
/// 1. modifier_before_{{modifier_name}}
/// 2. modifier_after_{{modifier_name}}
///
/// Both functions have the same definition, except the implementation:
/// the  first function takes statements before the `_`, and the second
/// take the remaining statements.
pub(super) fn def(impls: &FnImplementations, ctx: &mut Context) -> Result<(FnDef, FnDef), ParserError> {
    let modifiers = impls.as_modifiers();

    if modifiers.len() != 1 {
        return Err(ParserError::InvalidModifier(impls.name.to_owned()))
    }

    let (_, def) = modifiers.first().unwrap();
    let before_stmts = common::parse_statements(&def.before_stmts, ctx);
    let after_stmts = common::parse_statements(&def.after_stmts, ctx);

    let before_fn: Class = format!("modifier_before_{}", def.base_name).into();
    let after_fn: Class = format!("modifier_after_{}", def.base_name).into();
    let args = common::args(&def.params, def.is_mutable)?;
    Ok((
        FnDef::Plain(PlainFnDef {
            attrs: vec![],
            name: before_fn.clone(),
            args: args.clone(),
            ret: parse_quote!(),
            implementation: ClassFnImpl {
                visibility: parse_quote!(),
                class: None,
                fun: before_fn,
                implementation: parse_quote!({ #(#before_stmts)* }),
            },
        }),
        FnDef::Plain(PlainFnDef {
            attrs: vec![],
            name: after_fn.clone(),
            args,
            ret: parse_quote!(),
            implementation: ClassFnImpl {
                visibility: parse_quote!(),
                class: None,
                fun: after_fn,
                implementation: parse_quote!({ #(#after_stmts)* }),
            },
        }),
    ))
}
