use quote::{format_ident, ToTokens};
use syn::parse_quote;

use crate::{
    model::ir::{Expression, Stmt},
    parser::context::{
        ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
    },
    utils, ParserError,
};

use super::expr;

/// Parses solidity statement into a syn statement.
///
/// Todo: to handle remaining statements.
pub fn parse_statement<T>(stmt: &Stmt, is_semi: bool, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    match stmt {
        Stmt::Expression { expr } => {
            let expr = expr::parse(expr, ctx)?;
            if !is_semi {
                Ok(syn::Stmt::Expr(expr))
            } else {
                Ok(syn::Stmt::Semi(expr, Default::default()))
            }
        }
        Stmt::VarDefinition {
            declaration,
            ty,
            init,
        } => {
            let name = utils::to_snake_case_ident(declaration);
            let pat: syn::Pat = parse_quote! { #name };
            if let Expression::Func { name, args } = init {
                if let Some(class_name) = ctx.as_contract_name(name) {
                    let args = expr::parse_many(&args, ctx)?;
                    let addr = args.get(0);
                    return Ok(parse_ext_contract_stmt(&class_name, pat, addr, ctx));
                }
            };
            let expr: syn::Expr = expr::primitives::get_var_or_parse(init, ctx)?;
            ctx.register_local_var(declaration, ty);
            Ok(parse_quote!(let mut #pat = #expr;))
        }
        Stmt::VarDeclaration { declaration, ty } => {
            let name = utils::to_snake_case_ident(declaration);
            let pat: syn::Pat = parse_quote!(#name);
            ctx.register_local_var(declaration, ty);
            Ok(parse_quote!(let #pat;))
        }
        Stmt::Return { expr } => {
            let ret = match expr {
                Expression::Variable { name } => expr::primitives::get_var(name, ctx),
                expr => expr::parse(expr, ctx),
            }?;
            Ok(parse_quote!(return #ret;))
        }
        Stmt::ReturnVoid => Ok(parse_quote!(return;)),
        Stmt::If { assertion, if_body } => {
            let assertion = expr::parse(assertion, ctx)?;
            let if_body = parse_statement(if_body, false, ctx)?;
            let result: syn::Stmt = parse_quote!(if #assertion #if_body);
            Ok(result)
        }
        Stmt::IfElse {
            assertion,
            if_body,
            else_body,
        } => {
            let assertion = expr::parse(assertion, ctx)?;
            let if_body = parse_statement(if_body, false, ctx)?;
            let else_body = parse_statement(else_body, false, ctx)?;

            Ok(parse_quote!(if #assertion #if_body else #else_body))
        }
        Stmt::Block { stmts } => {
            let res = stmts
                .iter()
                .map(|stmt| parse_statement(stmt, false, ctx))
                .collect::<Result<Vec<syn::Stmt>, _>>()?;

            Ok(parse_quote!({ #(#res);* }))
        }
        Stmt::Emit { expr } => parse_emit_event(expr, ctx),
        Stmt::Revert { msg } => {
            if let Some(error) = msg {
                let expr = expr::error::revert(None, error, ctx)?;
                Ok(parse_quote!(#expr;))
            } else {
                let expr = expr::error::revert_with_str(None, "", ctx)?;
                Ok(parse_quote!(#expr;))
            }
        }
        Stmt::RevertWithError { error } => {
            let expr = expr::error::revert_with_err(error)?;
            Ok(parse_quote!(#expr;))
        }
        Stmt::While { assertion, block } => {
            let assertion = expr::parse(assertion, ctx)?;
            let block = parse_statement(block, false, ctx)?;
            dbg!(assertion.to_token_stream().to_string());
            dbg!(block.to_token_stream().to_string());
            Ok(parse_quote!(while #assertion #block))
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

fn parse_emit_event<T>(expr: &Expression, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    match expr {
        Expression::Func { name, args } => {
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

// pub fn parse_expr<T>(expr: &Expression, is_semi: bool, c) -> Result<syn::Stmt, ParserError>
// where
//     T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
// {
//     expr::parse(expr, ctx).map(|e| parse_quote!(#e;))
// }

#[cfg(test)]
mod test;
