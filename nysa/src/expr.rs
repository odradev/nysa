use std::ops::Deref;

use quote::format_ident;
use quote::quote;
use solidity_parser::pt;
use syn::parse_quote;

use crate::{utils::to_snake_case_ident, var::IsField};

/// Parses solidity expression into a syn expression.
///
/// Todo: to handle remaining expressions.
pub fn parse(
    expression: &pt::Expression,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Expr, &'static str> {
    match expression {
        pt::Expression::ArraySubscript(_, array_expression, key_expression) => {
            let array = parse(array_expression, storage_fields)?;

            if let Some(exp) = key_expression {
                let key = parse(exp, storage_fields)?;
                // TODO: check if it is a local array or contract storage.
                // TODO: what if the type does not implement Default trait.
                Ok(parse_quote!(#array.get(&#key).unwrap_or_default()))
            } else {
                Err("Unspecified key")
            }
        }
        pt::Expression::MemberAccess(_, expression, id) => match expression.as_ref() {
            pt::Expression::Variable(var) => {
                if &var.name == "msg" && &id.name == "sender" {
                    Ok(parse_quote!(odra::contract_env::caller()))
                } else {
                    Err("Unknown variable")
                }
            }
            _ => {
                let base_expr: syn::Expr = parse(expression, storage_fields)?;
                let member: syn::Member = format_ident!("{}", id.name).into();
                Ok(parse_quote!(#base_expr.#member))
            }
        },
        pt::Expression::Assign(_, le, re) => {
            let le: &pt::Expression = le;
            let re: &pt::Expression = re;
            if let pt::Expression::ArraySubscript(_, array_expr, key_expr) = le {
                let array = parse(array_expr, storage_fields)?;
                let key = parse(&key_expr.clone().unwrap(), storage_fields)?;
                let value = match re {
                    pt::Expression::Variable(id) => parse_variable(id, None, storage_fields),
                    _ => parse(re, storage_fields),
                }?;
                Ok(parse_quote!(#array.set(&#key, #value)))
            } else if let pt::Expression::Variable(id) = le {
                let re = parse(re, storage_fields)?;
                parse_variable(id, Some(re), storage_fields)
            } else {
                Err("Unsupported expr assign")
            }
        }
        pt::Expression::Variable(id) => match id.name.as_str() {
            "_" => Err("Empty identifier"),
            "require" => Err("Require call"),
            ident => {
                let ident = to_snake_case_ident(ident);
                let self_ty = id.is_field(storage_fields).then(|| quote!(self.));
                Ok(parse_quote!(#self_ty #ident))
            }
        },
        pt::Expression::FunctionCall(_, name, args) => {
            match parse(name, storage_fields) {
                Ok(name) => {
                    let args = parse_many(args, storage_fields)?;
                    Ok(parse_quote!(self.#name(#(#args),*)))
                }
                Err(err) => match err {
                    "Require call" => {
                        let args = args
                            .iter()
                            .map(|e| parse(e, storage_fields))
                            .collect::<Result<Vec<syn::Expr>, _>>()?;
                        // TODO: Consider various `require` implementations
                        let check = args.get(0).expect("Should be revert condition");
                        let err = args.get(1).expect("Should be the error message");
                        // TODO: find a way the enumerate errors
                        Ok(
                            parse_quote!(if #check { return; } else { odra::contract_env::revert(odra::types::ExecutionError::new(1, #err)) }),
                        )
                    }
                    _ => Err(err),
                },
            }
        }
        pt::Expression::LessEqual(_, l, r) => {
            let l = parse(l, storage_fields)?;
            let r = parse(r, storage_fields)?;
            Ok(parse_quote!(#l <= #r))
        }
        solidity_parser::pt::Expression::MoreEqual(_, l, r) => {
            let l = parse(l, storage_fields)?;
            let r = parse(r, storage_fields)?;
            Ok(parse_quote!(#l >= #r))
        }
        pt::Expression::NumberLiteral(_, num) => {
            let (sign, digs) = num.to_u32_digits();
            let num = digs[0];
            Ok(parse_quote!(#num))
        }
        pt::Expression::Add(_, l, r) => {
            let l = parse(l, storage_fields)?;
            let r = parse(r, storage_fields)?;
            Ok(parse_quote!(#l + #r))
        }
        pt::Expression::Subtract(_, l, r) => {
            let l = parse(l, storage_fields)?;
            let r = parse(r, storage_fields)?;
            Ok(parse_quote!(#l - #r))
        }
        pt::Expression::PostIncrement(_, expression) => {
            let expr = parse(expression, storage_fields)?;
            // TODO: find out if it is a member or a local variable
            Ok(parse_quote!(#expr += 1))
        }
        pt::Expression::PostDecrement(_, expression) => {
            let expr = parse(expression, storage_fields)?;
            // TODO: find out if it is a member or a local variable
            Ok(parse_quote!(#expr -= 1))
        }
        solidity_parser::pt::Expression::PreIncrement(_, _) => {
            let expr = parse(expression, storage_fields)?;
            Ok(parse_quote!(#expr += 1))
        }
        solidity_parser::pt::Expression::PreDecrement(_, _) => {
            let expr = parse(expression, storage_fields)?;
            Ok(parse_quote!(#expr -= 1))
        }
        solidity_parser::pt::Expression::Equal(_, left, right) => {
            let left = match &**left {
                pt::Expression::Variable(id) => parse_variable(id, None, storage_fields),
                _ => parse(left, storage_fields),
            }?;
            let right = match &**right {
                pt::Expression::Variable(id) => parse_variable(id, None, storage_fields),
                _ => parse(right, storage_fields),
            }?;
            Ok(parse_quote!(#left == #right))
        }
        solidity_parser::pt::Expression::StringLiteral(strings) => {
            let strings = strings
                .iter()
                .map(|lit| lit.string.clone())
                .collect::<Vec<_>>();
            let string = strings.join(",");
            Ok(parse_quote!(#string))
        }
        solidity_parser::pt::Expression::AssignSubtract(_, left, right) => {
            let expr = match &**left {
                pt::Expression::ArraySubscript(id, array_expr, key_expr) => {
                    let key_expr = key_expr.clone().map(|boxed| boxed.deref().clone());
                    let value_expr = parse(&right, storage_fields)?;
                    let current_value_expr =
                        parse_mapping(array_expr, key_expr.clone(), None, storage_fields)?;
                    let new_value: syn::Expr = parse_quote!(#current_value_expr - #value_expr);
                    parse_mapping(array_expr, key_expr, Some(new_value), storage_fields)
                }
                _ => parse(left, storage_fields),
            }?;
            Ok(expr)
        }
        solidity_parser::pt::Expression::AssignAdd(_, left, right) => {
            let expr = match &**left {
                pt::Expression::ArraySubscript(id, array_expr, key_expr) => {
                    let key_expr = key_expr.clone().map(|boxed| boxed.deref().clone());
                    let value_expr = parse(&right, storage_fields)?;
                    let current_value_expr =
                        parse_mapping(array_expr, key_expr.clone(), None, storage_fields)?;
                    let new_value: syn::Expr = parse_quote!(#current_value_expr + #value_expr);
                    parse_mapping(array_expr, key_expr, Some(new_value), storage_fields)
                }
                pt::Expression::Variable(id) => {
                    let current_value_expr = parse_variable(id, None, storage_fields)?;
                    let value_expr = parse(&right, storage_fields)?;
                    let new_value: syn::Expr = parse_quote!(#current_value_expr + #value_expr);
                    parse_variable(id, Some(new_value), storage_fields)
                }
                _ => parse(left, storage_fields),
            }?;
            Ok(expr)
        }
        _ => panic!("Unsupported expression {:?}", expression),
    }
}

pub fn parse_variable(
    id: &pt::Identifier,
    value_expr: Option<syn::Expr>,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Expr, &'static str> {
    let is_field = id.is_field(storage_fields);
    match id.name.as_str() {
        "_" => Err("Empty identifier"),
        "require" => Err("Require call"),
        ident => {
            let ident = to_snake_case_ident(ident);
            let self_ty = is_field.then(|| quote!(self.));
            if let Some(value) = value_expr {
                match is_field {
                    // Variable update must use the `set` function
                    true => Ok(parse_quote!(#self_ty #ident.set(#value))),
                    // regular, local value
                    false => Ok(parse_quote!(#ident = #value)),
                }
            } else {
                match is_field {
                    true => Ok(parse_quote!(odra::UnwrapOrRevert::unwrap_or_revert(#self_ty #ident.get()))),
                    false => Ok(parse_quote!(#self_ty #ident)),
                }
            }
        }
    }
}

pub fn parse_mapping(
    array_expr: &pt::Expression,
    key_expr: Option<pt::Expression>,
    value_expr: Option<syn::Expr>,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Expr, &'static str> {
    let array = parse(array_expr, storage_fields)?;

    if let Some(expr) = key_expr {
        let key = parse(&expr, storage_fields)?;
        // TODO: check if it is a local array or contract storage.
        // TODO: what if the type does not implement Default trait.
        if let Some(value) = value_expr {
            Ok(parse_quote!(#array.set(&#key, #value)))
        } else {
            Ok(parse_quote!(#array.get(&#key).unwrap_or_default()))
        }
    } else {
        Err("Unspecified key")
    }
}

pub fn parse_many(
    expressions: &[pt::Expression],
    storage_fields: &[&pt::VariableDefinition],
) -> Result<Vec<syn::Expr>, &'static str> {
    expressions
        .iter()
        .map(|e| parse(e, storage_fields))
        .collect::<Result<Vec<syn::Expr>, _>>()
}
