use proc_macro2::{Ident, TokenStream};
use syn::{parse_quote, BinOp};

use super::op;
use crate::{
    error::ParserResult,
    formatted_invalid_expr,
    model::ir::{Expression, TupleItem},
    parser::common::StatementParserContext,
    utils, Parser,
};
use quote::{format_ident, quote};

pub(super) fn parse_update<T, O, P>(
    left: &[TupleItem],
    right: &Expression,
    operator: Option<O>,
    ctx: &mut T,
) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    O: Into<BinOp> + Clone,
    P: Parser,
{
    // a tuple that defines local variables
    // sol: (uint a, uint b) = (1, 1);
    if left
        .iter()
        .all(|i| matches!(i, TupleItem::Declaration(_, _)))
    {
        let items = left
            .iter()
            .filter_map(|i| match i {
                TupleItem::Declaration(ty, name) => Some((ty, name)),
                _ => None,
            })
            .map(|(e, n)| {
                let name = utils::to_snake_case_ident(n);
                let ty = TryFrom::try_from(e).unwrap();
                ctx.register_local_var(n, &ty);
                quote!(mut #name)
            })
            .collect::<syn::punctuated::Punctuated<TokenStream, syn::Token![,]>>();
        let values = super::parse::<_, P>(right, ctx)?;

        return Ok(parse_quote!(let (#items) = #values));
    } else {
        // a tuple that defines update a tuple - may be multiple local/state variables or mix of both.

        if let Expression::Tuple(values) = right {
            // The lvalue is a tuple
            // sol: (a, b) = (1, 1);
            // rs: {
            //   a = 1;
            //   b = 2;
            // }
            // However the syntax (a, b) = (1, 1) is correct in rust, if a variable is a state variable
            // Odra uses `set()` function not the `=` operator
            let items: Vec<syn::Stmt> =
                parse_tuple_statements::<_, _, P>(left, values, operator, ctx)?;
            return Ok(parse_quote!( { #(#items)* } ));
        } else {
            // The lvalue is an expression that returns a tuple.
            // sol: (a, b) = func_call();
            // rs: {
            //   let (_0, _1) = func_call();
            //   a = _0;
            //   b = _1;
            // }
            // Due to the same reason as above a more verbose syntax is required.
            let names = (0..left.len())
                .map(|idx| format_ident!("_{}", idx))
                .collect::<syn::punctuated::Punctuated<Ident, syn::Token![,]>>();
            let values = super::parse::<_, P>(right, ctx)?;

            let tmp_items = (0..left.len())
                .map(|idx| TupleItem::Expr(Expression::Variable(format!("_{}", idx))))
                .collect::<Vec<_>>();

            let assignment: Vec<syn::Stmt> =
                parse_tuple_statements::<_, _, P>(left, &tmp_items, operator, ctx)?;

            return Ok(parse_quote!({
                let (#names) = #values;
                #(#assignment)*
            }));
        }
    }
}

fn parse_tuple_statements<T, O, P>(
    left: &[TupleItem],
    right: &[TupleItem],
    operator: Option<O>,
    ctx: &mut T,
) -> ParserResult<Vec<syn::Stmt>>
where
    T: StatementParserContext,
    O: Into<BinOp> + Clone,
    P: Parser,
{
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| {
            if let TupleItem::Expr(r) = r {
                match l {
                    TupleItem::Expr(l) => op::assign::<_, _, P>(l, Some(r), operator.clone(), ctx)
                        .map(|e| parse_quote!(#e;)),
                    TupleItem::Wildcard => {
                        let value = super::parse::<_, P>(r, ctx)?;
                        Ok(parse_quote!(let _ =  #value;))
                    }
                    TupleItem::Declaration(_, _) => formatted_invalid_expr!("invalid tuple item"),
                }
            } else {
                formatted_invalid_expr!("invalid tuple item")
            }
        })
        .collect::<ParserResult<_>>()
}
