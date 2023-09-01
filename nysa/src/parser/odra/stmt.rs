use quote::{format_ident, ToTokens};
use syn::parse_quote;

use crate::{
    model::ir::{NysaExpression, NysaStmt},
    parser::context::Context,
    utils, ParserError,
};

use super::expr;

/// Parses solidity statement into a syn statement.
///
/// Todo: to handle remaining statements.
pub fn parse_statement(stmt: &NysaStmt, ctx: &mut Context) -> Result<syn::Stmt, ParserError> {
    match stmt {
        NysaStmt::Expression { expr } => {
            let expr = expr::parse(expr, ctx)?;
            Ok(parse_quote!(#expr;))
        }
        NysaStmt::VarDefinition { declaration, init } => {
            let name = utils::to_snake_case_ident(declaration);
            let pat: syn::Pat = parse_quote! { #name };
            if let NysaExpression::Func { name, args } = init {
                if let Some(class_name) = ctx.class(name) {
                    let args = expr::parse_many(&args, ctx)?;
                    let addr = args.get(0);
                    return Ok(parse_ext_contract_stmt(&class_name, pat, addr, ctx));
                }
            };
            let expr: syn::Expr = expr::primitives::read_variable_or_parse(init, ctx)?;
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
            let result: syn::Stmt = parse_quote!(if #assertion #if_body else #else_body);
            Ok(result)
        }
        NysaStmt::Block { stmts } => {
            let res = stmts
                .iter()
                .map(|stmt| parse_statement(stmt, ctx))
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
                    .map(|e| expr::parse(e, ctx))
                    .collect::<Result<Vec<syn::Expr>, _>>()?;
                ctx.register_event(event_ident.to_string().as_str());
                Ok(parse_quote!(
                    <#event_ident as odra::types::event::OdraEvent>::emit(
                        #event_ident::new(#(#args),*)
                    );
                ))
            }
            _ => panic!("Invalid Emit statement"),
        },
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

pub fn parse_ext_contract_stmt<S: ToTokens, T: ToTokens>(
    contract_name: &str,
    ident: S,
    addr: T,
    ctx: &mut Context,
) -> syn::Stmt {
    ctx.register_external_call(contract_name);

    let ref_ident = format_ident!("{}Ref", contract_name);
    parse_quote!(let mut #ident = #ref_ident::at(&odra::UnwrapOrRevert::unwrap_or_revert(#addr));)
}

#[cfg(test)]
mod t {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::parse_statement;
    use crate::{
        model::ir::{NysaExpression, NysaStmt},
        parser::context::Context,
    };

    #[test]
    fn revert_with_no_msg() {
        let stmt = NysaStmt::Revert { msg: None };
        let result = parse_statement(&stmt, &mut Context::default()).unwrap();
        let expected: syn::Stmt =
            parse_quote!(odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "")););

        assert(result, expected);
    }

    #[test]
    fn revert_with_msg() {
        let error_msg = "An error occurred";
        let stmt = NysaStmt::Revert {
            msg: Some(NysaExpression::StringLiteral(error_msg.to_string())),
        };
        let result = parse_statement(&stmt, &mut Context::default()).unwrap();
        let expected: syn::Stmt = parse_quote!(
            odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "An error occurred"));
        );

        assert(result, expected)
    }

    #[test]
    fn revert_with_error() {
        let error_msg = "MyError";
        let stmt = NysaStmt::RevertWithError {
            error: error_msg.to_string(),
        };
        let result = parse_statement(&stmt, &mut Context::default()).unwrap();
        let expected: syn::Stmt = parse_quote!(odra::contract_env::revert(Error::MyError););

        assert(result, expected)
    }

    #[test]
    fn invalid_revert_stmt() {
        let error_msg = "An error occurred";
        let stmt = NysaStmt::Revert {
            msg: Some(NysaExpression::Placeholder),
        };
        let result = parse_statement(&stmt, &mut Context::default());

        assert!(result.is_err());
    }

    fn assert<L, R>(left: L, right: R)
    where
        L: ToTokens,
        R: ToTokens,
    {
        assert_eq!(
            left.into_token_stream().to_string(),
            right.into_token_stream().to_string()
        )
    }
}
