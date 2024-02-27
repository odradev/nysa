use proc_macro2::Ident;
use syn::BinOp;

use super::var::{self, parse_or_default};
use crate::{
    error::ParserResult,
    formatted_invalid_expr,
    model::ir::{eval_expression_type, Expression, Type},
    parser::{
        common::{ExpressionParser, NumberParser, StatementParserContext},
        context::ItemType,
    },
    utils::to_snake_case_ident,
    Parser, ParserError,
};

pub fn parse<T, P>(
    name: &str,
    keys: &[Expression],
    value: Option<syn::Expr>,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    P: Parser,
{
    let ident = to_snake_case_ident(name);

    let item_type = ctx
        .type_from_string(name)
        .ok_or(ParserError::InvalidExpression(
            "unknown item type".to_string(),
        ))?;
    match &item_type {
        ItemType::Storage(v) => parse_storage::<_, P>(ident, keys, value, &v.ty, ctx),
        ItemType::Local(v) => parse_local::<_, P>(ident, keys, value, &v.ty, ctx),
        _ => formatted_invalid_expr!("unknown collection {:?}", item_type),
    }
}

fn parse_local<T, P>(
    var_ident: Ident,
    keys_expr: &[Expression],
    value_expr: Option<syn::Expr>,
    ty: &Type,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    P: Parser,
{
    if keys_expr.len() == 0 {
        return Err(ParserError::InvalidCollection);
    }

    let keys = if let Type::Mapping(_, _) = ty {
        keys_expr
            .iter()
            .map(|k| parse_storage_key::<_, P>(&k, ctx))
            .collect::<ParserResult<_>>()
    } else {
        keys_expr
            .iter()
            .map(|k| parse_local_key::<_, P>(&k, ctx))
            .collect::<ParserResult<_>>()
    }?;

    <P::ExpressionParser as ExpressionParser>::parse_local_collection(
        var_ident, keys, value_expr, ty, ctx,
    )
}

fn parse_storage<T, P>(
    var_ident: Ident,
    keys_expr: &[Expression],
    value_expr: Option<syn::Expr>,
    ty: &Type,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    P: Parser,
{
    if keys_expr.len() == 0 {
        return Err(ParserError::InvalidCollection);
    }

    let keys = keys_expr
        .iter()
        .map(|k| parse_storage_key::<_, P>(&k, ctx))
        .collect::<ParserResult<Vec<syn::Expr>>>()?;

    <P::ExpressionParser as ExpressionParser>::parse_storage_collection(
        var_ident, keys, value_expr, ty, ctx,
    )
}

pub(crate) fn parse_storage_key<T, P>(key: &Expression, ctx: &mut T) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    P: Parser,
{
    match key {
        Expression::NumberLiteral(v) => {
            <P::ExpressionParser as NumberParser>::parse_typed_number(v, ctx)
        }
        _ => parse_or_default::<_, P>(key, ctx),
    }
}

fn parse_local_key<T, P>(key: &Expression, ctx: &mut T) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    P: Parser,
{
    match <P::ExpressionParser as NumberParser>::parse_generic_number(key) {
        Ok(e) => Ok(e),
        Err(_) => var::parse_or_default::<_, P>(key, ctx),
    }
}

pub(super) fn parse_update<T, O, P>(
    name: &str,
    keys: &[Expression],
    right: &Expression,
    operator: Option<O>,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    O: Into<BinOp>,
    P: Parser,
{
    if operator.is_none() {
        let value = parse_or_default::<_, P>(right, ctx)?;
        parse::<_, P>(name, &keys, Some(value), ctx)
    } else {
        let op = operator.map(Into::<BinOp>::into).unwrap();
        let value_expr = parse_or_default::<_, P>(right, ctx)?;
        let current_value_expr = parse::<_, P>(name, &keys, None, ctx)?;
        let ty = eval_expression_type(right, ctx).expect("failed to evaluate expression type");

        let new_value = <P::ExpressionParser as ExpressionParser>::parse_update_value_expression(
            current_value_expr,
            value_expr,
            op,
            ty,
            ctx,
        );
        parse::<_, P>(name, &keys, Some(new_value), ctx)
    }
}
