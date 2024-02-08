use super::{parse, parse_many, primitives};
use crate::{
    error::ParserResult,
    model::ir::Expression,
    parser::context::{
        ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
        TypeInfo,
    },
    utils,
};
use proc_macro2::Ident;
use syn::parse_quote;

const PROPERTY_LENGTH: &str = "length";

/// Parses an expression reading a property from an array into a `syn::Expr`.
///
/// # Solidity Example
/// ```ignore
/// arr.length;
/// ````
pub fn read_property<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
>(
    property_name: &str,
    expr: &Expression,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let array = primitives::get_var_or_parse(expr, ctx)?;
    if property_name == PROPERTY_LENGTH {
        Ok(parse_quote!(#array.len().into()))
    } else {
        let property = utils::to_ident(property_name);
        Ok(parse_quote!(#array.#property))
    }
}

/// Parses an expression calling a function on an array into a `syn::Expr`.
///
/// # Solidity Example
/// ```ignore
/// arr.push(i);
/// ```
pub fn fn_call<T>(
    array_name: &str,
    fn_ident: Ident,
    args: &[Expression],
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let result_expr: syn::Expr = parse_quote!(result);
    let array = primitives::get_var_or_parse(&Expression::from(array_name), ctx)?;
    let args = parse_many(args, ctx)?;
    let update_array = primitives::set_var(array_name, result_expr.clone(), ctx)?;
    Ok(parse_quote!({
        let mut #result_expr = #array;
        #result_expr.#fn_ident(#(#args),*);
        #update_array;
    }))
}

/// Parses an expression replacing a value in an array into a `syn::Expr`.
///
/// # Solidity Example
/// ```ignore
/// // uint[] memory a = new uint[](5);
/// a[1] = 123;
/// ```
pub fn replace_value<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
>(
    array_name: &str,
    index: &Expression,
    value: syn::Expr,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let result_expr: syn::Expr = parse_quote!(result);
    let index = parse(index, ctx)?;
    let array = primitives::get_var_or_parse(&Expression::from(array_name), ctx)?;
    let update_array = primitives::set_var(array_name, result_expr.clone(), ctx)?;
    Ok(parse_quote!({
        let mut #result_expr = #array;
        #result_expr[#index] = #value;
        #update_array;
    }))
}
