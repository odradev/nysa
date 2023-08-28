use crate::model::ContractData;
use c3_lang_parser::c3_ast::FnDef;

use super::context::Context;

mod common;
mod constructor;
mod function;
mod modifier;

/// Extracts function definitions and pareses into a vector of c3 ast [FnDef].
pub fn functions_def<'a>(data: &ContractData, ctx: &mut Context) -> Vec<FnDef> {
    let names = data.functions_str();

    let result = data
        .fn_implementations()
        .iter()
        .map(|i| {
            ctx.set_current_fn(i);
            if i.is_modifier() {
                let (a, b) = modifier::def(i, ctx);
                vec![a, b]
            } else if i.is_constructor() {
                constructor::def(i, data, ctx)
            } else {
                vec![function::def(i, data, &names, ctx)]
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    ctx.clear_current_fn();
    result
}
