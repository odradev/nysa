use syn::{parse_quote, BinOp};

use crate::{
    error::ParserResult,
    model::ir::{BitwiseOp, Expression, LogicalOp, MathOp, Op, Type, UnaryOp, Var},
    parser::{self, common::StatementParserContext, context::ItemType, syn_utils::in_context},
    Parser, ParserError,
};

use super::{array, collection, math, tuple, var};

/// Parses an assign expression (=, +=, -=, *=, /=).
///
/// In solidity there is left-hand and right-hand statement
/// Eg.
/// right         left
///
/// totalSupply = balanceOf[msg.sender]
///
/// In Odra, if we update Variable or Mapping, there is a single expression.
/// Eg.
/// self.total_supply.set(self.balance_of.get(self.env().caller())).
///
/// To parse any kind of an assign statement we need to treat it a single statement
/// and parse both sides at once.
///
/// # Arguments
///
/// * `left` - Left-hand side solidity expression.
/// * `right` - Right-hand side solidity expression.
/// * `value_expr` - An optional operator (eg. +, -)
/// * `ctx` - parser Context.
pub fn assign<T: StatementParserContext, O: Into<BinOp> + Clone, P: Parser>(
    left: &Expression,
    right: Option<&Expression>,
    operator: Option<O>,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    in_context(left, ctx, |ctx| match right {
        Some(right) => match left {
            Expression::Collection(name, keys) => {
                collection::parse_update::<_, _, P>(name, keys, right, operator, ctx)
            }
            Expression::Variable(name) => var::parse_update::<_, _, P>(name, right, operator, ctx),
            Expression::Tuple(left_items) => {
                tuple::parse_update::<_, _, P>(left_items, right, operator, ctx)
            }
            Expression::MemberAccess(field, var) => {
                let l = super::parse::<_, P>(left, ctx)?;
                let r = super::parse::<_, P>(right, ctx)?;
                Ok(parse_quote!(#l = #r))
            }
            _ => todo!(),
        },
        None => assign_default::<_, P>(left, ctx),
    })
}

pub(crate) fn bin_op<T: StatementParserContext, O: Into<BinOp>, P: Parser>(
    left: &Expression,
    right: &Expression,
    op: O,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let left_expr = math::eval_in_context::<_, P>(left, right, ctx)?;
    let right_expr = math::eval_in_context::<_, P>(right, left, ctx)?;

    let op: BinOp = op.into();

    Ok(parse_quote!(#left_expr #op #right_expr))
}

pub(crate) fn unary_op<T: StatementParserContext, P: Parser>(
    expr: &Expression,
    op: &UnaryOp,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let expr = math::eval::<_, P>(expr, ctx)?;

    Ok(match op {
        UnaryOp::Not => parse_quote!(!#expr),
        UnaryOp::Plus => parse_quote!(#expr),
        UnaryOp::Minus => parse_quote!(-#expr),
    })
}

impl Into<BinOp> for &LogicalOp {
    fn into(self) -> BinOp {
        match self {
            LogicalOp::Less => parse_quote!(<),
            LogicalOp::LessEq => parse_quote!(<=),
            LogicalOp::More => parse_quote!(>),
            LogicalOp::MoreEq => parse_quote!(>=),
            LogicalOp::Eq => parse_quote!(==),
            LogicalOp::NotEq => parse_quote!(!=),
            LogicalOp::And => parse_quote!(&&),
            LogicalOp::Or => parse_quote!(||),
        }
    }
}

impl Into<BinOp> for &BitwiseOp {
    fn into(self) -> BinOp {
        match self {
            BitwiseOp::And => parse_quote!(&),
            BitwiseOp::Or => parse_quote!(|),
            BitwiseOp::ShiftLeft => parse_quote!(<<),
            BitwiseOp::ShiftRight => parse_quote!(>>),
            BitwiseOp::Xor => parse_quote!(^),
            BitwiseOp::Not => parse_quote!(!),
        }
    }
}

impl Into<BinOp> for &MathOp {
    fn into(self) -> BinOp {
        match self {
            MathOp::Add => parse_quote!(+),
            MathOp::Sub => parse_quote!(-),
            MathOp::Div => parse_quote!(/),
            MathOp::Modulo => parse_quote!(%),
            MathOp::Mul => parse_quote!(*),
            MathOp::Pow => panic!("Cannot parse to BinOp"),
        }
    }
}

impl Into<BinOp> for &Op {
    fn into(self) -> BinOp {
        match self {
            Op::Bitwise(bo) => bo.into(),
            Op::Math(mo) => mo.into(),
            Op::Unary(_) => todo!(),
            Op::Logical(o) => o.into(),
        }
    }
}

fn assign_default<T: StatementParserContext, P: Parser>(
    left: &Expression,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let err = || ParserError::UnexpectedExpression("Expression::Variable", left.clone());
    let default_expr = parser::syn_utils::default();

    match left {
        Expression::Variable(name) => var::parse_set::<_, P>(&name, default_expr, ctx),
        Expression::Collection(name, keys) => match ctx.type_from_string(name) {
            Some(ItemType::Storage(Var {
                ty: Type::Array(_), ..
            })) => array::replace_value::<_, P>(name, &keys[0], default_expr, ctx),
            _ => Err(err()),
        },
        _ => Err(err()),
    }
}
