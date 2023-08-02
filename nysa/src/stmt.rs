use quote::format_ident;
use solidity_parser::pt;
use syn::parse_quote;

use crate::expr;

/// Parses solidity statement into a syn statement.
///
/// Todo: to handle remaining statements.
pub fn parse_statement(
    stmt: &pt::Statement,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Stmt, &'static str> {
    match stmt {
        pt::Statement::Expression(loc, expression) => {
            let expr = expr::parse(expression, storage_fields)?;
            Ok(parse_quote!(#expr;))
        }
        pt::Statement::VariableDefinition(_, declaration, expression) => {
            let name = format_ident!("{}", &declaration.name.name);
            let pat: syn::Pat = parse_quote! { #name };
            let expr: syn::Expr = expr::parse(expression.as_ref().unwrap(), storage_fields)?;

            Ok(parse_quote!(let #pat = #expr;))
        }
        pt::Statement::Return(_, expression) => {
            let ret = match expression.as_ref().unwrap() {
                pt::Expression::Variable(id) => expr::parse_variable(id, None, storage_fields),
                expr => expr::parse(expr, storage_fields),
            }?;
            Ok(parse_quote!(return #ret;))
        }
        pt::Statement::If(_, assertion, if_body, else_body) => {
            let assertion = expr::parse(assertion, storage_fields)?;
            let if_body = parse_statement(if_body, storage_fields)?;
            let else_body = else_body.clone().expect("Else body not found");
            let else_body = parse_statement(&else_body, storage_fields)?;
            let result: syn::Stmt = parse_quote!(if #assertion #if_body else #else_body);
            Ok(result)
        }
        pt::Statement::Block {
            loc: _,
            unchecked: _,
            statements,
        } => {
            let res = statements
                .iter()
                .map(|stmt| parse_statement(stmt, storage_fields))
                .collect::<Result<Vec<syn::Stmt>, _>>()?;

            Ok(parse_quote!({ #(#res);* }))
        }
        pt::Statement::Emit(_, expr) => match expr {
            pt::Expression::FunctionCall(_, name, args) => {
                let event_ident = match &**name {
                    pt::Expression::Variable(id) => format_ident!("{}", id.name),
                    _ => panic!("Invalid Emit statement"),
                };
                let args = args
                    .iter()
                    .map(|e| expr::parse(e, storage_fields))
                    .collect::<Result<Vec<syn::Expr>, _>>()?;
                Ok(parse_quote!(
                    <#event_ident as odra::types::event::OdraEvent>::emit(
                        #event_ident { #(#args),* }
                    );
                ))
            }
            _ => panic!("Invalid Emit statement"),
        },
        _ => panic!("Unsupported statement {:?}", stmt),
    }
}
