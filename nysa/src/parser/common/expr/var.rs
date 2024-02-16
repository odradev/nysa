use syn::{parse_quote, BinOp};

use super::parse;
use crate::{
    error::ParserResult,
    formatted_invalid_expr,
    model::ir::Expression,
    parser::{
        common::{ExpressionParser, StatementParserContext},
        context::{FnContext, ItemType, StorageInfo, TypeInfo},
        syn_utils::{AsExpression, AsSelfField},
    },
    utils::to_snake_case_ident,
    Parser,
};

/// Parses Expression::Variable into a syn::Expr. If the passed expression is not a variable,
/// it will be parsed as a regular expression.
pub fn parse_or_default<T: StatementParserContext, P: Parser>(
    expr: &Expression,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    match expr {
        Expression::Variable(name) => parse_get::<_, P>(name, ctx),
        _ => parse::<_, P>(expr, ctx),
    }
}

/// Parses a single set value interaction.
///
/// In solidity referring to a contract storage value and a local variable is the same.
/// When interacting with an Odra value, we need to know more context if we use a module field or a local value
///
/// # Arguments
///
/// * `id` - A variable identifier.
/// * `value_expr` - am expression that writes to a var
/// * `ctx` - Parser context
///
/// # Returns
///
/// A parsed syn expression.
pub(super) fn parse_set<T: StorageInfo + TypeInfo + FnContext, P: Parser>(
    id: &str,
    value_expr: syn::Expr,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let item_type = ctx.type_from_string(id);
    let var = var(&item_type, id)?;

    <P::ExpressionParser as ExpressionParser>::parse_set_var_expression(var, value_expr, item_type)
}

/// Parses a single get value interactions.
///
/// In solidity referring to a contract storage value and a local variable is the same.
/// When interacting with an Odra value, we need to know more context if we use a module field or a local value
///
/// # Arguments
///
/// * `id` - A variable identifier.
/// * `ctx` - A slice containing all the contract storage fields.
///
/// # Returns
///
/// A parsed syn expression.
pub(super) fn parse_get<T: StorageInfo + TypeInfo + FnContext, P: Parser>(
    id: &str,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let item_type = ctx.type_from_string(id);
    let var = var(&item_type, id)?;

    match item_type {
        Some(ItemType::Storage(v)) => Ok(
            <P::ExpressionParser as ExpressionParser>::parse_read_values_expression(
                var, None, &v.ty, ctx,
            ),
        ),
        _ => Ok(var),
    }
}

pub(super) fn parse_update<T, O, P>(
    name: &str,
    right: &Expression,
    operator: Option<O>,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    O: Into<BinOp>,
    P: Parser,
{
    // a regular assign expression
    if operator.is_none() {
        let right = parse_or_default::<_, P>(right, ctx)?;
        parse_set::<_, P>(&name, right, ctx)
    } else {
        // calculate the new value by reading the current value and applying the operator,
        // then set the new value
        let op = operator.map(Into::<BinOp>::into);
        let current_value_expr = parse_get::<_, P>(&name, ctx)?;
        let value_expr = parse_or_default::<_, P>(right, ctx)?;
        let new_value: syn::Expr = parse_quote!(#current_value_expr #op #value_expr);
        parse_set::<_, P>(&name, new_value, ctx)
    }
}

fn var(item_type: &Option<ItemType>, id: &str) -> ParserResult<syn::Expr> {
    let ident = to_snake_case_ident(id);

    match item_type {
        // Variable update must use the `set` function
        Some(ItemType::Storage(_)) => Ok(ident.as_self_field()),
        // regular, local value
        Some(ItemType::Local(_)) => Ok(ident.as_expression()),
        None => Ok(ident.as_expression()),
        _ => formatted_invalid_expr!("unknown variable {:?}", item_type),
    }
}
