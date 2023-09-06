use crate::{
    model::ContractData,
    parser::context::{
        ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
    },
    ParserError,
};
use c3_lang_parser::c3_ast::FnDef;

mod common;
mod constructor;
mod function;
pub(super) mod interface;
mod modifier;

/// Extracts function definitions and pareses into a vector of c3 ast [FnDef].
pub fn functions_def<'a, T>(data: &ContractData, ctx: &mut T) -> Result<Vec<FnDef>, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let names = data.functions_str();

    let result = data
        .fn_implementations()
        .iter()
        .map(|i| {
            ctx.set_current_fn(i);
            if i.is_modifier() {
                modifier::def(i, ctx).map(|(a, b)| vec![a, b])
            } else if i.is_constructor() {
                constructor::def(i, data, ctx)
            } else {
                function::def(i, data, &names, ctx).map(|f| vec![f])
            }
        })
        .collect::<Result<Vec<_>, ParserError>>()
        .map(|v: Vec<Vec<FnDef>>| v.into_iter().flatten().collect());

    ctx.clear_current_fn();
    result
}
