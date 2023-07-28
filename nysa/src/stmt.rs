use c3_lang_parser::c3_ast::VarDef;
use quote::format_ident;
use solidity_parser::pt;
use syn::parse_quote;

use super::expr::parse_expression;

/// Parses solidity statement into a syn statement.
///
/// Todo: to handle remaining statements.
pub fn parse_statement(stmt: &pt::Statement, storage_fields: &[VarDef]) -> Result<syn::Stmt, &'static str> {
    match stmt {
        pt::Statement::Expression(loc, expression) => {
            let expr = parse_expression(expression, storage_fields)?;
            Ok(parse_quote!(#expr;))
        }
        pt::Statement::VariableDefinition(_, declaration, expression) => {
            let name = format_ident!("{}", &declaration.name.name);
            let pat: syn::Pat = parse_quote! { #name };
            let expr: syn::Expr = parse_expression(expression.as_ref().unwrap(), storage_fields)?;

            Ok(parse_quote!(let #pat = #expr;))
        }
        pt::Statement::Return(_, expression) => {
            let ret = parse_expression(expression.as_ref().unwrap(), storage_fields)?;
            Ok(parse_quote!(return #ret;))
        }
        pt::Statement::If(_, assertion, if_body, else_body) => {
            let assertion = parse_expression(assertion, storage_fields)?;
            let if_body = parse_statement(if_body, storage_fields)?;
            let else_body = else_body.clone().unwrap();
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
        _ => panic!("Unsupported statement {:?}", stmt),
    }
}
