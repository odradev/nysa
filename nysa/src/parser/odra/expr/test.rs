use quote::{quote, ToTokens};
use solidity_parser::pt::{SourceUnitPart, Statement};

use crate::model::ir::Type;
use crate::parser::context::{with_context, FnContext, LocalContext};
use crate::parser::odra::test::assert_tokens_eq;

#[test]
fn assign_and_compare() {
    with_context(|ctx| {
        ctx.register_local_var(&"x".to_string(), &Type::Uint(32));
        ctx.register_local_var(&"a".to_string(), &Type::Uint(32));
        ctx.register_local_var(&"b".to_string(), &Type::Uint(32));

        let solidity_expr = "(x = a + b) <= 256;";
        let expected_rust_code = quote!(
            {
                x = (a + b);
                x
            } <= nysa_types::U32::from_limbs_slice(&[256u64])
        );

        assert_expression(solidity_expr, expected_rust_code, ctx);
    });
}

#[test]
fn complex_stmt() {
    with_context(|ctx| {
        ctx.register_local_var(&"y".to_string(), &Type::Uint(32));

        let solidity_expr = "!(y == 0 || (z = x * y) / y == x);";
        let expected_rust_code = quote!(
            !(y == nysa_types::U32::ZERO
                || ({
                    z = (x * y);
                    z
                } / y)
                    == x)
        );

        assert_expression(solidity_expr, expected_rust_code, ctx);
    })
}

fn assert_expression<T: AsRef<str>, R: ToTokens>(
    solidity_expr: T,
    expected: R,
    ctx: &mut LocalContext,
) {
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
            if let Some(Statement::Expression(_, e)) = statements.first() {
                let expr = e.into();
                let expr = super::parse(&expr, ctx).unwrap();
                assert_tokens_eq(expr, expected);
                return;
            }
        }
    }
    panic!("Could not find an expression")
}
