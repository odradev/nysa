use quote::ToTokens;
use syn::parse_quote;

use crate::{
    model::ir::{Expression, NumSize, Op},
    parser::context::test::EmptyContext,
};

#[test]
fn assign_and_compare() {
    // sol: x = a + b <= 256
    let expr = Expression::Compare {
        var_left: Some("x".to_string()),
        left: Box::new(Expression::Assign {
            left: Box::new(Expression::Variable {
                name: "x".to_string(),
            }),
            right: Box::new(Expression::Add {
                left: Box::new(Expression::Variable {
                    name: "b".to_string(),
                }),
                right: Box::new(Expression::Variable {
                    name: "a".to_string(),
                }),
            }),
        }),
        var_right: None,
        right: Box::new(Expression::NumberLiteral {
            ty: NumSize::U32,
            value: vec![0, 1, 0, 0],
        }),
        op: Op::LessEq,
    };
    let result = super::parse(&expr, &mut EmptyContext).unwrap();
    let expected: syn::Expr = parse_quote!(
        {
            x = (b + a);
            x
        } <= 256u32.into()
    );

    assert(result, expected);
}

#[test]
fn complex_stmt() {
    // sol: !(y == 0u8.into() || (z = (x * y) / y) == x)
    let expr = Expression::Not {
        expr: Box::new(Expression::Or {
            left: Box::new(Expression::Compare {
                var_left: None,
                left: Box::new(Expression::Variable {
                    name: "y".to_string(),
                }),
                var_right: None,
                right: Box::new(Expression::NumberLiteral {
                    ty: NumSize::U32,
                    value: vec![0, 0, 0, 0],
                }),
                op: Op::Eq,
            }),
            right: Box::new(Expression::Compare {
                var_left: Some("z".to_string()),
                left: Box::new(Expression::Divide {
                    left: Box::new(Expression::Assign {
                        left: Box::new(Expression::Variable {
                            name: "z".to_string(),
                        }),
                        right: Box::new(Expression::Multiply {
                            left: Box::new(Expression::Variable {
                                name: "x".to_string(),
                            }),
                            right: Box::new(Expression::Variable {
                                name: "y".to_string(),
                            }),
                        }),
                    }),
                    right: Box::new(Expression::Variable {
                        name: "y".to_string(),
                    }),
                }),
                var_right: None,
                right: Box::new(Expression::Variable {
                    name: "x".to_string(),
                }),
                op: Op::Eq,
            }),
        }),
    };

    assert(
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
