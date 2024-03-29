use crate::{
    error::ParserResult,
    model::ir::FnImplementations,
    parser::context::{
        ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
        TypeInfo,
    },
};
use c3_lang_parser::c3_ast::FnDef;

mod common;
mod constructor;
mod function;
pub(super) mod interface;
mod modifier;
mod syn_utils;

/// Parses currently processed function from the context into a vector of c3 ast [FnDef].
pub fn functions_def<'a, T>(ctx: &mut T) -> ParserResult<Vec<FnDef>>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    match ctx.current_contract().is_library() {
        true => parse_library_functions(ctx),
        false => parse_contract_functions(ctx),
    }
}

fn parse_contract_functions<'a, T>(ctx: &mut T) -> ParserResult<Vec<FnDef>>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    in_fn_context(ctx, |i, ctx| {
        if i.is_modifier() {
            modifier::def(i, ctx).map(|(before, after)| vec![before, after])
        } else if i.is_constructor() {
            constructor::def(i, ctx)
        } else {
            function::def(i, ctx).map(|f| vec![f])
        }
    })
}

fn parse_library_functions<'a, T>(ctx: &mut T) -> ParserResult<Vec<FnDef>>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    in_fn_context(ctx, |i, ctx| {
        if i.is_constructor() {
            Ok(vec![])
        } else {
            function::library_def(i, ctx).map(|f| vec![f])
        }
    })
}

fn in_fn_context<T, F>(ctx: &mut T, f: F) -> ParserResult<Vec<FnDef>>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
    F: Fn(&FnImplementations, &mut T) -> ParserResult<Vec<FnDef>>,
{
    ctx.current_contract()
        .fn_implementations()
        .iter()
        .map(|i| {
            ctx.set_current_fn(i);
            let res = f(i, ctx);
            ctx.clear_current_fn();
            res
        })
        .collect::<ParserResult<Vec<_>>>()
        .map(|v: Vec<Vec<FnDef>>| v.into_iter().flatten().collect())
}
