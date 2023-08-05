use quote::format_ident;
use syn::parse_quote;

use crate::{
    expr,
    model::{NysaExpression, NysaStmt, StorageField},
    utils,
};

/// Parses solidity statement into a syn statement.
///
/// Todo: to handle remaining statements.
pub fn parse_statement(
    stmt: &NysaStmt,
    storage_fields: &[StorageField],
) -> Result<syn::Stmt, &'static str> {
    match stmt {
        NysaStmt::Expression { expr } => {
            let expr = expr::parse(expr, storage_fields)?;
            Ok(parse_quote!(#expr;))
        }
        NysaStmt::VarDefinition { declaration, init } => {
            let name = utils::to_snake_case_ident(declaration);
            let pat: syn::Pat = parse_quote! { #name };
            let expr: syn::Expr = expr::parse(init, storage_fields)?;
            Ok(parse_quote!(let #pat = #expr;))
        }
        NysaStmt::VarDeclaration { declaration } => {
            let name = utils::to_snake_case_ident(declaration);
            let pat: syn::Pat = parse_quote! { #name };
            Ok(parse_quote!(let #pat;))
        }
        NysaStmt::Return { expr } => {
            let ret = match expr {
                NysaExpression::Variable { name } => {
                    expr::primitives::parse_variable(name, None, storage_fields)
                }
                expr => expr::parse(expr, storage_fields),
            }?;
            Ok(parse_quote!(return #ret;))
        }
        NysaStmt::ReturnVoid => Ok(parse_quote!(return;)),
        NysaStmt::If { assertion, if_body } => {
            let assertion = expr::parse(assertion, storage_fields)?;
            let if_body = parse_statement(if_body, storage_fields)?;
            let result: syn::Stmt = parse_quote!(if #assertion #if_body);
            Ok(result)
        }
        NysaStmt::IfElse {
            assertion,
            if_body,
            else_body,
        } => {
            let assertion = expr::parse(assertion, storage_fields)?;
            let if_body = parse_statement(if_body, storage_fields)?;
            let else_body = parse_statement(else_body, storage_fields)?;
            let result: syn::Stmt = parse_quote!(if #assertion #if_body else #else_body);
            Ok(result)
        }
        NysaStmt::Block { stmts } => {
            let res = stmts
                .iter()
                .map(|stmt| parse_statement(stmt, storage_fields))
                .collect::<Result<Vec<syn::Stmt>, _>>()?;

            Ok(parse_quote!({ #(#res);* }))
        }
        NysaStmt::Emit { expr } => match expr {
            NysaExpression::Func { name, args } => {
                let event_ident = match &**name {
                    NysaExpression::Variable { name } => format_ident!("{}", name),
                    _ => panic!("Invalid Emit statement"),
                };
                let args = args
                    .iter()
                    .map(|e| expr::parse(e, storage_fields))
                    .collect::<Result<Vec<syn::Expr>, _>>()?;
                Ok(parse_quote!(
                    <#event_ident as odra::types::event::OdraEvent>::emit(
                        #event_ident::new(#(#args),*)
                    );
                ))
            }
            _ => panic!("Invalid Emit statement"),
        },
        _ => panic!("Unsupported statement {:?}", stmt),
    }
}
