use syn::parse_quote;

use crate::model::ir::MathOp;
use crate::parser::odra::test::assert_tokens_eq;
use crate::{
    model::ir::{Expression, LogicalOp, NumSize},
    parser::context::test::EmptyContext,
};

#[test]
fn assign_and_compare() {
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
        Box::new(Expression::NumberLiteral(NumSize::U32, vec![0, 1, 0, 0])),
        LogicalOp::LessEq,
    );
    let result = super::parse(&expr, &mut EmptyContext).unwrap();
    let expected: syn::Expr = parse_quote!(
        {
            x = (b + a);
            x
        } <= 256u32.into()
    );

    assert_tokens_eq(result, expected);
}

#[test]
fn complex_stmt() {
    // sol: !(y == 0u8.into() || (z = (x * y) / y) == x)
    let or_left = Expression::LogicalOp(
        Box::new(Expression::Variable("y".to_string())),
        Box::new(Expression::NumberLiteral(NumSize::U32, vec![0, 0, 0, 0])),
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
        super::parse(&expr, &mut EmptyContext).unwrap(),
        quote::quote!(
            !(y == 0u32.into()
                || ({
                    z = (x * y);
                    z
                } / y)
                    == x)
        ),
    );
}
