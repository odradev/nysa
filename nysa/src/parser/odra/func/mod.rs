use crate::{
    model::ir::FnImplementations,
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
pub fn functions_def<'a, T>(ctx: &mut T) -> Result<Vec<FnDef>, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    match ctx.current_contract().is_library() {
        true => parse_library_functions(ctx),
        false => parse_contract_functions(ctx),
    }
}

fn parse_contract_functions<'a, T>(ctx: &mut T) -> Result<Vec<FnDef>, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    in_fn_context(ctx, |i, ctx| {
        if i.is_modifier() {
            modifier::def(i, ctx).map(|(a, b)| vec![a, b])
        } else if i.is_constructor() {
            constructor::def(i, ctx)
        } else {
            function::def(i, ctx).map(|f| vec![f])
        }
    })
}

fn parse_library_functions<'a, T>(ctx: &mut T) -> Result<Vec<FnDef>, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    in_fn_context(ctx, |i, ctx| {
        if i.is_constructor() {
            Ok(vec![])
        } else {
            function::def2(i, ctx).map(|f| vec![f])
        }
    })
}

fn in_fn_context<T, F>(ctx: &mut T, f: F) -> Result<Vec<FnDef>, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
    F: Fn(&FnImplementations, &mut T) -> Result<Vec<FnDef>, ParserError>,
{
    let result = ctx
        .current_contract()
        .fn_implementations()
        .iter()
        .map(|i| {
            ctx.set_current_fn(i);
            let res = f(i, ctx);
            ctx.clear_current_fn();
            res
        })
        .collect::<Result<Vec<_>, ParserError>>()
        .map(|v: Vec<Vec<FnDef>>| v.into_iter().flatten().collect());

    result
}
