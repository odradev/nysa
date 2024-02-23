use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassFnImpl, FnDef, PlainFnDef};
use syn::parse_quote;

use crate::{
    error::ParserResult,
    model::ir::{contains_sender_expr, FnImplementations},
    parser::common::{FunctionParser, StatementParserContext},
    Parser, ParserError,
};

use super::common;

/// Splits a solidity modifier into two functions:
/// 1. modifier_before_{{modifier_name}}
/// 2. modifier_after_{{modifier_name}}
///
/// Both functions have the same definition, except the implementation:
/// the  first function takes statements before the `_`, and the second
/// take the remaining statements.
pub(super) fn def<T, P>(impls: &FnImplementations, ctx: &mut T) -> ParserResult<(FnDef, FnDef)>
where
    T: StatementParserContext,
    P: Parser,
{
    let modifiers = impls.as_modifiers();

    if modifiers.len() != 1 {
        return Err(ParserError::InvalidModifier(impls.name.to_owned()));
    }

    let (_, def) = modifiers[0];
    let before_stmts = common::parse_statements::<_, P>(&def.before_stmts, ctx);
    let after_stmts = common::parse_statements::<_, P>(&def.after_stmts, ctx);

    let params = common::parse_params::<_, P>(&def.params, ctx)?;
    common::register_local_vars(&def.params, ctx);

    let uses_sender =
        contains_sender_expr(&def.after_stmts) || contains_sender_expr(&def.before_stmts);
    let args =
        <P::FnParser as FunctionParser>::parse_modifier_args(params, def.is_mutable, uses_sender)?;
    let before_fn: Class = format!("modifier_before_{}", def.base_name).into();
    let after_fn: Class = format!("modifier_after_{}", def.base_name).into();
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
