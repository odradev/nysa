use quote::{format_ident, ToTokens};
use syn::parse_quote;

use crate::{
    model::ir::{NysaExpression, NysaStmt},
    parser::context::{
        ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
    },
    utils, ParserError,
};

use super::expr;

/// Parses solidity statement into a syn statement.
///
/// Todo: to handle remaining statements.
pub fn parse_statement<T>(stmt: &NysaStmt, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    // dbg!(stmt);
    match stmt {
        NysaStmt::Expression { expr } => expr::parse(expr, ctx).map(|e| parse_quote!(#e;)),
        NysaStmt::VarDefinition {
            declaration,
            ty,
            init,
        } => {
            let name = utils::to_snake_case_ident(declaration);
            let pat: syn::Pat = parse_quote! { #name };
            if let NysaExpression::Func { name, args } = init {
                if let Some(class_name) = ctx.as_contract_name(name) {
                    let args = expr::parse_many(&args, ctx)?;
                    let addr = args.get(0);
                    return Ok(parse_ext_contract_stmt(&class_name, pat, addr, ctx));
                }
            };
            let expr: syn::Expr = expr::primitives::read_variable_or_parse(init, ctx)?;
            ctx.register_local_var(declaration, ty);
            Ok(parse_quote!(let mut #pat = #expr;))
        }
        NysaStmt::VarDeclaration { declaration, ty } => {
            let name = utils::to_snake_case_ident(declaration);
            let pat: syn::Pat = parse_quote!(#name);
            ctx.register_local_var(declaration, ty);
            Ok(parse_quote!(let #pat;))
        }
        NysaStmt::Return { expr } => {
            let ret = match expr {
                NysaExpression::Variable { name } => {
                    expr::primitives::parse_variable(name, None, ctx)
                }
                expr => expr::parse(expr, ctx),
            }?;
            Ok(parse_quote!(return #ret;))
        }
        NysaStmt::ReturnVoid => Ok(parse_quote!(return;)),
        NysaStmt::If { assertion, if_body } => {
            let assertion = expr::parse(assertion, ctx)?;
            let if_body = parse_statement(if_body, ctx)?;
            let result: syn::Stmt = parse_quote!(if #assertion #if_body);
            Ok(result)
        }
        NysaStmt::IfElse {
            assertion,
            if_body,
            else_body,
        } => {
            let assertion = expr::parse(assertion, ctx)?;
            let if_body = parse_statement(if_body, ctx)?;
            let else_body = parse_statement(else_body, ctx)?;
            Ok(parse_quote!(if #assertion #if_body else #else_body))
        }
        NysaStmt::Block { stmts } => {
            let res = stmts
                .iter()
                .map(|stmt| parse_statement(stmt, ctx))
                .collect::<Result<Vec<syn::Stmt>, _>>()?;

            Ok(parse_quote!({ #(#res);* }))
        }
        NysaStmt::Emit { expr } => parse_emit_event(expr, ctx),
        NysaStmt::Revert { msg } => {
            if let Some(error) = msg {
                let expr = expr::error::revert(None, error, ctx)?;
                Ok(parse_quote!(#expr;))
            } else {
                let expr = expr::error::revert_with_str(None, "", ctx)?;
                Ok(parse_quote!(#expr;))
            }
        }
        NysaStmt::RevertWithError { error } => {
            let expr = expr::error::revert_with_err(error)?;
            Ok(parse_quote!(#expr;))
        }
        _ => panic!("Unsupported statement {:?}", stmt),
    }
}

pub fn parse_ext_contract_stmt<S: ToTokens, T: ToTokens, R>(
    contract_name: &str,
    ident: S,
    addr: T,
    ctx: &mut R,
) -> syn::Stmt
where
    R: ExternalCallsRegister,
{
    ctx.register_external_call(contract_name);

    let ref_ident = format_ident!("{}Ref", contract_name);
    parse_quote!(let mut #ident = #ref_ident::at(&odra::UnwrapOrRevert::unwrap_or_revert(#addr));)
}

fn parse_emit_event<T>(expr: &NysaExpression, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    match expr {
        NysaExpression::Func { name, args } => {
            let event_ident = TryInto::<String>::try_into(*name.to_owned())
                .map(|name| format_ident!("{}", name))?;
            let args = expr::parse_many(args, ctx)?;

            ctx.register_event(event_ident.to_string().as_str());

            Ok(parse_quote!(
                <#event_ident as odra::types::event::OdraEvent>::emit(
                    #event_ident::new(#(#args),*)
                );
            ))
        }
        _ => panic!("Invalid Emit statement"),
    }
}

#[cfg(test)]
mod test;
