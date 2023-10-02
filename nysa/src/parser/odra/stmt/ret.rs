use syn::parse_quote;

use crate::model::ir::{Expression, Func, Function};
use crate::parser::context::{
    ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
};
use crate::parser::odra::expr;
use crate::ParserError;

pub(super) fn ret<T>(expr: &Expression, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let ret_ty = ctx
        .current_fn()
        .implementations
        .first()
        .map(|(_, func)| match &func {
            Function::Function(Func { ret, .. }) => Some(ret[0].1.clone()),
            _ => None,
        })
        .map(|e| match e {
            Some(Expression::Type(ty)) => Some(ty),
            _ => None,
        })
        .flatten();

    let pushed = ctx.push_expected_type(ret_ty);
    let ret = expr::primitives::get_var_or_parse(expr, ctx)?;
    if pushed {
        ctx.drop_expected_type();
    }
    Ok(parse_quote!(return #ret;))
}

pub(super) fn ret_unit() -> Result<syn::Stmt, ParserError> {
    Ok(parse_quote!(return;))
}
