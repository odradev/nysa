use syn::{parse_quote, BinOp};

use crate::{
    error::ParserResult,
    model::ir::{BitwiseOp, Expression, LogicalOp, MathOp, Op, UnaryOp},
    parser::{
        context::{
            ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
            TypeInfo,
        },
        odra::expr::math,
    },
};

pub(crate) fn bin_op<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
    O: Into<BinOp>,
>(
    left: &Expression,
    right: &Expression,
    op: O,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let left_expr = math::eval_in_context(left, right, ctx)?;
    let right_expr = math::eval_in_context(right, left, ctx)?;

    let op: BinOp = op.into();

    Ok(parse_quote!(#left_expr #op #right_expr))
}

pub(crate) fn unary_op<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
>(
    expr: &Expression,
    op: &UnaryOp,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let expr = math::eval(expr, ctx)?;

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
