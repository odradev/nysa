use quote::ToTokens;
use syn::parse_quote;

use crate::{
    model::ir::{NumSize, NysaExpression, Op},
    parser::context::test::EmptyContext,
};

#[test]
fn assign_and_compare() {
    // sol: x = a + b <= 256
    let expr = NysaExpression::Compare {
        var_left: Some("x".to_string()),
        left: Box::new(NysaExpression::Assign {
            left: Box::new(NysaExpression::Variable {
                name: "x".to_string(),
            }),
            right: Box::new(NysaExpression::Add {
                left: Box::new(NysaExpression::Variable {
                    name: "b".to_string(),
                }),
                right: Box::new(NysaExpression::Variable {
                    name: "a".to_string(),
                }),
            }),
        }),
        var_right: None,
        right: Box::new(NysaExpression::NumberLiteral {
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
    let expr = NysaExpression::Not {
        expr: Box::new(NysaExpression::Or {
            left: Box::new(NysaExpression::Compare {
                var_left: None,
                left: Box::new(NysaExpression::Variable {
                    name: "y".to_string(),
                }),
                var_right: None,
                right: Box::new(NysaExpression::NumberLiteral {
                    ty: NumSize::U32,
                    value: vec![0, 0, 0, 0],
                }),
                op: Op::Eq,
            }),
            right: Box::new(NysaExpression::Compare {
                var_left: Some("z".to_string()),
                left: Box::new(NysaExpression::Divide {
                    left: Box::new(NysaExpression::Assign {
                        left: Box::new(NysaExpression::Variable {
                            name: "z".to_string(),
                        }),
                        right: Box::new(NysaExpression::Multiply {
                            left: Box::new(NysaExpression::Variable {
                                name: "x".to_string(),
                            }),
                            right: Box::new(NysaExpression::Variable {
                                name: "y".to_string(),
                            }),
                        }),
                    }),
                    right: Box::new(NysaExpression::Variable {
                        name: "y".to_string(),
                    }),
                }),
                var_right: None,
                right: Box::new(NysaExpression::Variable {
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
