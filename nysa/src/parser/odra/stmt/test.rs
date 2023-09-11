use quote::ToTokens;
use syn::parse_quote;

use super::parse_statement;
use crate::{
    model::ir::{NysaExpression, NysaStmt},
    parser::context::test::EmptyContext,
};

#[test]
fn revert_with_no_msg() {
    let stmt = NysaStmt::Revert { msg: None };
    let result = parse_statement(&stmt, true, &mut EmptyContext).unwrap();
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
    let result = parse_statement(&stmt, true, &mut EmptyContext).unwrap();
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
    let result = parse_statement(&stmt, true, &mut EmptyContext).unwrap();
    let expected: syn::Stmt = parse_quote!(odra::contract_env::revert(Error::MyError););

    assert(result, expected)
}

#[test]
fn invalid_revert_stmt() {
    let error_msg = "An error occurred";
    let stmt = NysaStmt::Revert {
        msg: Some(NysaExpression::Placeholder),
    };
    let result = parse_statement(&stmt, true, &mut EmptyContext);

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
