use crate::{
    error::ParserResult,
    model::ir::FnImplementations,
    parser::{
        common::StatementParserContext,
        context::{
            ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
        },
    },
    Parser,
};
use c3_lang_parser::c3_ast::FnDef;

mod common;
mod constructor;
mod function;
pub(crate) mod interface;
mod modifier;

/// Parses currently processed function from the context into a vector of c3 ast [FnDef].
pub fn functions_def<'a, T, P>(ctx: &mut T) -> ParserResult<Vec<FnDef>>
where
    T: StatementParserContext,
    P: Parser,
{
    match ctx.current_contract().is_library() {
        true => parse_library_functions::<_, P>(ctx),
        false => parse_contract_functions::<_, P>(ctx),
    }
}

fn parse_contract_functions<'a, T, P>(ctx: &mut T) -> ParserResult<Vec<FnDef>>
where
    T: StatementParserContext,
    P: Parser,
{
    in_fn_context(ctx, |i, ctx| {
        if i.is_modifier() {
            modifier::def::<_, P>(i, ctx).map(|(before, after)| vec![before, after])
        } else if i.is_constructor() {
            constructor::def::<_, P>(i, ctx)
        } else {
            function::def::<_, P>(i, ctx).map(|f| vec![f])
        }
    })
}

fn parse_library_functions<'a, T, P>(ctx: &mut T) -> ParserResult<Vec<FnDef>>
where
    T: StatementParserContext,
    P: Parser,
{
    in_fn_context(ctx, |i, ctx| {
        if i.is_constructor() {
            Ok(vec![])
        } else {
            function::library_def::<_, P>(i, ctx).map(|f| vec![f])
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
