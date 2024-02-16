use super::{parse, parse_many, var};
use crate::{
    error::ParserResult, model::ir::Expression, parser::common::StatementParserContext, utils,
    Parser,
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
pub fn read_property<T: StatementParserContext, P: Parser>(
    property_name: &str,
    expr: &Expression,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let array = var::parse_or_default::<_, P>(expr, ctx)?;
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
pub fn fn_call<T, P>(
    array_name: &str,
    fn_ident: Ident,
    args: &[Expression],
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    P: Parser,
{
    let result_expr: syn::Expr = parse_quote!(result);
    let array = var::parse_or_default::<_, P>(&Expression::from(array_name), ctx)?;
    let args = parse_many::<_, P>(args, ctx)?;
    let update_array = var::parse_set::<_, P>(array_name, result_expr.clone(), ctx)?;
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
pub fn replace_value<T: StatementParserContext, P: Parser>(
    array_name: &str,
    index: &Expression,
    value: syn::Expr,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let result_expr: syn::Expr = parse_quote!(result);
    let index = parse::<_, P>(index, ctx)?;
    let array = var::parse_or_default::<_, P>(&Expression::from(array_name), ctx)?;
    let update_array = var::parse_set::<_, P>(array_name, result_expr.clone(), ctx)?;
    Ok(parse_quote!({
        let mut #result_expr = #array;
        #result_expr[#index] = #value;
        #update_array;
    }))
}
