use syn::parse_quote;

use crate::model::ir::{Expression, LogicalOp};
use crate::model::ir::{MathOp, Type};
use crate::parser::context::{ContractContext, FnContext, GlobalContext, LocalContext};
use crate::parser::odra::test::assert_tokens_eq;

#[test]
fn assign_and_compare() {
    let storage = vec![];
    let ctx = GlobalContext::new(vec![], vec![], vec![], vec![], vec![]);
    let ctx = ContractContext::new(&ctx, &storage);
    let mut ctx = LocalContext::new(ctx);
    ctx.register_local_var(&"x".to_string(), &Type::Uint(32));
    ctx.register_local_var(&"a".to_string(), &Type::Uint(32));
    ctx.register_local_var(&"b".to_string(), &Type::Uint(32));

    // sol: x = a + b <= 256
    let expr = Expression::LogicalOp(
        Box::new(Expression::Assign(
            Box::new(Expression::Variable("x".to_string())),
            Some(Box::new(Expression::MathOp(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::Variable("a".to_string())),
                MathOp::Add,
            ))),
        )),
        Box::new(Expression::NumberLiteral(vec![0, 1, 0, 0])),
        LogicalOp::LessEq,
    );
    let result = super::parse(&expr, &mut ctx).unwrap();
    let expected: syn::Expr = parse_quote!(
        {
            x = (b + a);
            x
        } <= nysa_types::U32::from_limbs_slice(&[0u64, 1u64, 0u64, 0u64])
    );

    assert_tokens_eq(result, expected);
}

#[test]
fn complex_stmt() {
    let storage = vec![];
    let ctx = GlobalContext::new(vec![], vec![], vec![], vec![], vec![]);
    let ctx = ContractContext::new(&ctx, &storage);
    let mut ctx = LocalContext::new(ctx);
    ctx.register_local_var(&"y".to_string(), &Type::Uint(32));

    // sol: !(y == 0u8.into() || (z = (x * y) / y) == x)
    let or_left = Expression::LogicalOp(
        Box::new(Expression::Variable("y".to_string())),
        Box::new(Expression::NumberLiteral(vec![0u64])),
        LogicalOp::Eq,
    );

    let or_right = Expression::LogicalOp(
        Box::new(Expression::MathOp(
            Box::new(Expression::Assign(
                Box::new(Expression::Variable("z".to_string())),
                Some(Box::new(Expression::MathOp(
                    Box::new(Expression::Variable("x".to_string())),
                    Box::new(Expression::Variable("y".to_string())),
                    MathOp::Mul,
                ))),
            )),
            Box::new(Expression::Variable("y".to_string())),
            MathOp::Div,
        )),
        Box::new(Expression::Variable("x".to_string())),
        LogicalOp::Eq,
    );

    let expr = Expression::Not(Box::new(Expression::LogicalOp(
        Box::new(or_left),
        Box::new(or_right),
        LogicalOp::Or,
    )));

    assert_tokens_eq(
        super::parse(&expr, &mut ctx).unwrap(),
        quote::quote!(
            !(y == nysa_types::U32::from_limbs_slice(&[0u64])
                || ({
                    z = (x * y);
                    z
                } / y)
                    == x)
        ),
    );
}
