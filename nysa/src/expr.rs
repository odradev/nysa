use c3_lang_parser::c3_ast::VarDef;
use quote::format_ident;
use solidity_parser::pt;
use syn::parse_quote;
use quote::quote;

use crate::utils::to_snake_case_ident;

/// Parses solidity expression into a syn expression.
///
/// Todo: to handle remaining expressions.
pub fn parse_expression(expression: &pt::Expression, storage_fields: &[VarDef]) -> Result<syn::Expr, &'static str> {
    match expression {
        pt::Expression::ArraySubscript(_, array_expression, key_expression) => {
            let array = parse_expression(array_expression, storage_fields)?;

            if let Some(exp) = key_expression {
                let key = parse_expression(exp, storage_fields)?;
                // TODO: check if it is a local array or contract storage.
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
                let base_expr: syn::Expr = parse_expression(expression, storage_fields)?;
                let member: syn::Member = format_ident!("{}", id.name).into();
                Ok(parse_quote!(#base_expr.#member))
            }
        },
        pt::Expression::Assign(_, le, re) => {
            let le: &pt::Expression = le;
            let re: &pt::Expression = re;
            if let pt::Expression::ArraySubscript(_, array_expr, key_expr) = le {
                let array = parse_expression(array_expr, storage_fields)?;
                let key = parse_expression(&key_expr.clone().unwrap(), storage_fields)?;
                let value = parse_expression(re, storage_fields)?;
                Ok(parse_quote!(#array.set(&#key, #value)))
            } else {
                Err("Unsupported expr assign")
            }
        }
        pt::Expression::Variable(id) => {
            match id.name.as_str() {
                "_" => Err("Empty identifier"),
                "require" => Err("Require call"),
                ident => {
                    let ident = to_snake_case_ident(ident);
                    let fields = storage_fields.iter().map(|f| f.ident.to_string()).collect::<Vec<_>>();
                    let self_ty = fields.contains(&id.name).then(|| quote!(self.));
                    Ok(parse_quote!(#self_ty #ident))
                }
            }
        }
        pt::Expression::FunctionCall(_, name, args) => {
            match parse_expression(name, storage_fields) {
                Ok(name) => {
                    let args = args.iter().map(|e| parse_expression(e, storage_fields)).collect::<Result<Vec<syn::Expr>, _>>()?;
                    Ok(parse_quote!(self.#name(#(#args),*)))
                },
                Err(err) => match err {
                    "Require call" => {
                        // dbg!(args.first());
                        let args = args.iter().map(|e| parse_expression(e, storage_fields)).collect::<Result<Vec<syn::Expr>, _>>()?;
                        // TODO: Consider various `require` implementations
                        let check = args.get(0).expect("Should be revert condition");
                        // dbg!(check);
                        let err = args.get(1).expect("Should be the error message");
                        // TODO: find a way the enumerate errors
                        Ok(parse_quote!(if #check { return; } else { odra::contract_env::revert(odra::types::ExecutionError::new(1, #err)) }))
                    },
                    _ => Err(err)
                },
            }
        }
        pt::Expression::LessEqual(_, l, r) => {
            let l = parse_expression(l, storage_fields)?;
            let r = parse_expression(r, storage_fields)?;
            Ok(parse_quote!(#l <= #r))
        }
        solidity_parser::pt::Expression::MoreEqual(_, l, r) => {
            let l = parse_expression(l, storage_fields)?;
            let r = parse_expression(r, storage_fields)?;
            Ok(parse_quote!(#l >= #r))
        }
        pt::Expression::NumberLiteral(_, num) => {
            let (sign, digs) = num.to_u32_digits();
            let num = digs[0];
            Ok(parse_quote!(#num))
        }
        pt::Expression::Add(_, l, r) => {
            let l = parse_expression(l, storage_fields)?;
            let r = parse_expression(r, storage_fields)?;
            Ok(parse_quote!(#l + #r))
        }
        pt::Expression::Subtract(_, l, r) => {
            let l = parse_expression(l, storage_fields)?;
            let r = parse_expression(r, storage_fields)?;
            Ok(parse_quote!(#l - #r))
        }
        pt::Expression::PostIncrement(_, expression) => {
            let expr = parse_expression(expression, storage_fields)?;
            // TODO: find out if it is a member or a local variable
            Ok(parse_quote!(#expr += 1))
        }
        pt::Expression::PostDecrement(_, expression) => {
            let expr = parse_expression(expression, storage_fields)?;
            // TODO: find out if it is a member or a local variable
            Ok(parse_quote!(#expr -= 1))
        }
        solidity_parser::pt::Expression::PreIncrement(_, _) => {
            let expr = parse_expression(expression, storage_fields)?;
            Ok(parse_quote!(#expr += 1))
        }
        solidity_parser::pt::Expression::PreDecrement(_, _) => {
            let expr = parse_expression(expression, storage_fields)?;
            Ok(parse_quote!(#expr -= 1))
        }
        solidity_parser::pt::Expression::Equal(_, left , right) => {
            let left = match &**left {
                pt::Expression::Variable(id) => parse_read_variable_expression(id, storage_fields),
                _ => parse_expression(left, storage_fields)
            }?; 
            let right = match &**right {
                pt::Expression::Variable(id) => parse_read_variable_expression(id, storage_fields),
                _ => parse_expression(right, storage_fields)
            }?; 
            Ok(parse_quote!(#left == #right))
        }
        solidity_parser::pt::Expression::StringLiteral(strings) => {
            let strings = strings.iter().map(|lit| lit.string.clone()).collect::<Vec<_>>();
            let string = strings.join(",");
            Ok(parse_quote!(#string))
        }
        _ => panic!("Unsupported expression {:?}", expression),
    }
}


fn parse_read_variable_expression(id: &pt::Identifier, storage_fields: &[VarDef]) -> Result<syn::Expr, &'static str> {
    match id.name.as_str() {
        "_" => Err("Empty identifier"),
        "require" => Err("Require call"),
        ident => {
            let ident = to_snake_case_ident(ident);
            let fields = storage_fields.iter().map(|f| f.ident.to_string()).collect::<Vec<_>>();
            let self_ty = fields.contains(&id.name).then(|| quote!(self.));
            Ok(parse_quote!(#self_ty #ident.get()))
        }
    }
}