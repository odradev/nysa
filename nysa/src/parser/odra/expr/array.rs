use super::{
    parse,
    primitives::{self, get_var_or_parse},
};
use crate::{
    model::ir::Expression,
    parser::context::{
        ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
    },
    ParserError,
};
use proc_macro2::Ident;
use quote::format_ident;
use syn::parse_quote;

const PROPERTY_LENGTH: &str = "length";

pub fn read_property<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    member_name: &str,
    expr: &Expression,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let base_expr: syn::Expr = get_var_or_parse(expr, ctx)?;
    if member_name == PROPERTY_LENGTH {
        Ok(parse_quote!(#base_expr.len().into()))
    } else {
        let member: syn::Member = format_ident!("{}", member_name).into();
        Ok(parse_quote!(#base_expr.#member))
    }
}

pub fn fn_call<T>(
    array_name: &str,
    fn_ident: Ident,
    args: Vec<syn::Expr>,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let result_expr: syn::Expr = parse_quote!(result);
    let arr = primitives::get_var_or_parse(&Expression::from(array_name), ctx)?;
    let update = primitives::set_var(array_name, result_expr.clone(), ctx)?;
    Ok(parse_quote!({
        let mut #result_expr = #arr;
        result.#fn_ident(#(#args),*);
        #update;
    }))
}

pub fn replace_value<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    array_name: &str,
    index: &Expression,
    value: syn::Expr,
    ctx: &mut T,
) -> Result<syn::Expr, ParserError> {
    let result_expr: syn::Expr = parse_quote!(result);
    let index = parse(index, ctx)?;
    let arr = primitives::get_var_or_parse(&Expression::from(array_name), ctx)?;
    let update = primitives::set_var(array_name, result_expr.clone(), ctx)?;
    Ok(parse_quote!({
        let mut #result_expr = #arr;
        result[#index] = #value;
        #update;
    }))
}
