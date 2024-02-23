#![cfg(test)]

use crate::{
    model::ir::{Stmt, Type},
    parser::context::{test::EmptyContext, with_context, FnContext, LocalContext},
    OdraParser, ParserError,
};
use quote::ToTokens;
use solidity_parser::pt::{SourceUnitPart, Statement};

use super::common::stmt::parse_statement;

pub type TestParser = OdraParser;

pub(crate) fn assert_tokens_eq<L, R>(left: L, right: R)
where
    L: ToTokens,
    R: ToTokens,
{
    assert_eq!(
        left.into_token_stream().to_string(),
        right.into_token_stream().to_string()
    )
}

#[test]
#[should_panic]
fn fail() {
    let _ = parse_with_empty_context(Stmt::Unknown);
}

pub(super) fn unsafe_parse_with_empty_context(stmt: Stmt) -> syn::Stmt {
    parse_with_empty_context(stmt).expect("Couldn't parse statement")
}

pub(super) fn parse_with_empty_context(stmt: Stmt) -> Result<syn::Stmt, ParserError> {
    parse_statement::<_, TestParser>(&stmt, true, &mut EmptyContext)
}

#[test]
fn test_no_block_if() {
    with_context(|ctx| {
        ctx.register_local_var(&"x".to_string(), &Type::Uint(32));
        ctx.register_local_var(&"y".to_string(), &Type::Uint(32));

        let solidity_expr = "if (x != 0) x = y;";
        let expected_rust_code = quote::quote!(if x != nysa_types::U32::ZERO {
            x = y;
        });

        assert_stmt(solidity_expr, expected_rust_code, ctx);
    })
}

fn assert_stmt<T: AsRef<str>, R: ToTokens>(solidity_expr: T, expected: R, ctx: &mut LocalContext) {
    // dummy function to successfully parse an expression.
    let src = r#"
    function foo() {
        {{STMT}}
    }
    "#;
    let src = src.replace("{{STMT}}", solidity_expr.as_ref());

    // parse code
    let (actual_parse_tree, _) = solidity_parser::parse(&src, 0).unwrap();
    let ast = actual_parse_tree.0.first().unwrap();

    // extract the expression from the AST
    if let SourceUnitPart::FunctionDefinition(box f) = ast {
        if let Some(Statement::Block {
            loc,
            unchecked,
            statements,
        }) = &f.body
        {
            if let Some(s) = statements.first() {
                let stmt = s.into();
                let expr = parse_statement::<_, TestParser>(&stmt, true, ctx)
                    .expect("Should be a valid statement");
                assert_tokens_eq(expr, expected);
                return;
            }
        }
    }
    panic!("Could not find an expression")
}
