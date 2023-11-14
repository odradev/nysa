use quote::{quote, ToTokens};
use solidity_parser::pt;

use crate::model::ir::{eval_expression_type, Expression, Type, Var};
use crate::model::ContractData;
use crate::parser::context::*;
use crate::parser::odra::test::assert_tokens_eq;

#[test]
fn assign_and_compare() {
    with_context(|ctx| {
        ctx.register_local_var(&"x".to_string(), &Type::Uint(32));
        ctx.register_local_var(&"a".to_string(), &Type::Uint(32));
        ctx.register_local_var(&"b".to_string(), &Type::Uint(32));

        let solidity_expr = "(x = a + b) <= 256";
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

        let solidity_expr = "!(y == 0 || (z = x * y) / y == x)";
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

#[test]
fn eval_complex_stmt() {
    with_context(|ctx: &mut LocalContext<'_>| {
        ctx.register_local_var(&"y".to_string(), &Type::Uint(32));

        let solidity_expr = "!(y == 0 || (z = x * y) / y == x)";

        let e = parse_expression(solidity_expr);
        let ty = eval_expression_type(&e, ctx);

        assert_eq!(ty, Some(Type::Bool));
    })
}

#[test]
fn eval_variables() {
    let mut global_ctx = GlobalContext::default();
    let storage = vec![Var {
        name: "my_var".to_string(),
        ty: Type::Bool,
        initializer: None,
        is_immutable: false,
    }];
    let data = ContractData::with_storage("test", storage);
    let contract_ctx = ContractContext::new(&mut global_ctx, data);
    let mut ctx = LocalContext::new(contract_ctx);
    ctx.register_local_var(&"y".to_string(), &Type::Uint(32));

    assert_expression_type("y", Some(Type::Uint(32)), &mut ctx);
    assert_expression_type("my_var", Some(Type::Bool), &mut ctx);
}

#[test]
fn eval_math() {
    with_context(|ctx| {
        ctx.register_local_var(&"y".to_string(), &Type::Uint(32));
        ctx.register_local_var(&"x".to_string(), &Type::Uint(256));
        ctx.register_local_var(&"z".to_string(), &Type::Uint(192));
        ctx.register_local_var(&"b".to_string(), &Type::Bool);

        assert_expression_type("y + y", Some(Type::Uint(32)), ctx);
        assert_expression_type("10 + x", Some(Type::Uint(256)), ctx);
        assert_expression_type("y + 10 + z", Some(Type::Uint(192)), ctx);
        assert_expression_type("y + x + z", Some(Type::Uint(256)), ctx);
        assert_expression_type("y + z", Some(Type::Uint(192)), ctx);
        assert_expression_type("z + y", Some(Type::Uint(192)), ctx);
        assert_expression_type("z * y", Some(Type::Uint(192)), ctx);
        assert_expression_type("z - y", Some(Type::Uint(192)), ctx);
        assert_expression_type("z / y", Some(Type::Uint(192)), ctx);
        assert_expression_type("x + b", None, ctx);
    })
}

fn assert_expression<T: AsRef<str>, R: ToTokens>(
    solidity_expr: T,
    expected: R,
    ctx: &mut LocalContext,
) {
    let expr = parse_expression(solidity_expr);
    let expr = super::parse(&expr, ctx).unwrap();
    assert_tokens_eq(expr, expected);
}

fn parse_expression<T: AsRef<str>>(solidity_expr: T) -> Expression {
    // dummy function to successfully parse an expression.
    let src = r#"
    function foo() {
        {{STMT}};
    }
    "#;
    let src = src.replace("{{STMT}}", solidity_expr.as_ref());

    // parse code
    let (actual_parse_tree, _) = solidity_parser::parse(&src, 0).unwrap();
    let ast = actual_parse_tree.0.first().unwrap();

    // extract the expression from the AST
    if let pt::SourceUnitPart::FunctionDefinition(box f) = ast {
        if let Some(pt::Statement::Block {
            loc,
            unchecked,
            statements,
        }) = &f.body
        {
            if let Some(pt::Statement::Expression(_, e)) = statements.first() {
                return e.into();
            }
        }
    }
    panic!("Could not find an expression")
}

fn assert_expression_type<T: AsRef<str>>(
    solidity_expr: T,
    expected_ty: Option<Type>,
    ctx: &mut LocalContext<'_>,
) {
    let e = parse_expression(solidity_expr);
    let ty = eval_expression_type(&e, ctx);
    assert_eq!(expected_ty, ty);
}
