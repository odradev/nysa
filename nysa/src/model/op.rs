#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum LogicalOp {
    Less,
    LessEq,
    More,
    MoreEq,
    Eq,
    NotEq,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum BitwiseOp {
    And,
    Or,
    ShiftLeft,
    ShiftRight,
    Xor,
    Not
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum UnaryOp {
    Not,
    Plus,
    Minus
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Op {
    Bitwise(BitwiseOp),
    Unary(UnaryOp),
    Math(MathOp),
    Logical(LogicalOp)
}