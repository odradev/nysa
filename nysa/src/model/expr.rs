use solidity_parser::pt::{self, Parameter};
use syn::parse_quote;

use crate::ParserError;

use super::{
    misc::Type,
    op::{BitwiseOp, LogicalOp, MathOp, Op, UnaryOp},
    stmt::Stmt,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Expression {
    Require(Box<Expression>, Box<Expression>),
    Placeholder,
    ZeroAddress,
    Message(Message),
    Collection(String, Vec<Expression>),
    Variable(String),
    BoolLiteral(bool),
    StringLiteral(String),
    Assign(Box<Expression>, Option<Box<Expression>>),
    LogicalOp(Box<Expression>, Box<Expression>, LogicalOp),
    MathOp(Box<Expression>, Box<Expression>, MathOp),
    AssignAnd(Box<Expression>, Box<Expression>, Op),
    Increment(Box<Expression>),
    Decrement(Box<Expression>),
    MemberAccess(String, Box<Expression>),
    NumberLiteral(Vec<u64>),
    Func(Box<Expression>, Vec<Expression>),
    SuperCall(String, Vec<Expression>),
    ExternalCall(String, String, Vec<Expression>),
    TypeInfo(Box<Expression>, String),
    Type(Type),
    Not(Box<Expression>),
    BytesLiteral(Vec<u8>),
    ArrayLiteral(Vec<Expression>),
    Initializer(Box<Expression>),
    Statement(Box<Stmt>),
    BitwiseOp(Box<Expression>, Box<Expression>, BitwiseOp),
    UnaryOp(Box<Expression>, UnaryOp),
    Tuple(Vec<TupleItem>),
    #[cfg(test)]
    /// To fail fast in tests
    Fail,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TupleItem {
    Expr(Expression),
    Declaration(Expression, String),
    Wildcard,
}

pub fn to_expr(solidity_expressions: Vec<pt::Expression>) -> Vec<Expression> {
    solidity_expressions.iter().map(From::from).collect()
}

impl From<&pt::Expression> for Expression {
    fn from(value: &pt::Expression) -> Self {
        parse_expr(value)
    }
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        Expression::Variable(value.to_string())
    }
}

impl TryInto<String> for Expression {
    type Error = ParserError;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Self::Variable(name) => Ok(name.clone()),
            _ => Err(ParserError::InvalidExpression),
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[allow(dead_code)]
pub enum Message {
    Sender,
    Value,
    Data,
}

impl TryInto<syn::Expr> for &Message {
    type Error = ParserError;

    fn try_into(self) -> Result<syn::Expr, Self::Error> {
        match self {
            Message::Sender => Ok(parse_quote!(Some(odra::contract_env::caller()))),
            Message::Value => todo!(),
            Message::Data => todo!(),
        }
    }
}

fn try_to_zero_address(name: &pt::Expression) -> Option<Expression> {
    if let pt::Expression::Type(_, ty) = name {
        if *ty == pt::Type::Address || *ty == pt::Type::AddressPayable {
            return Some(Expression::ZeroAddress);
        }
    }
    None
}

fn try_to_require(name: &pt::Expression, args: &[pt::Expression]) -> Option<Expression> {
    if let pt::Expression::Variable(ref id) = name {
        if id.name.as_str() == "require" {
            let condition = args.get(0).expect("Should be revert condition").into();
            let error = args.get(1).expect("Should be the error message").into();
            return Some(Expression::Require(Box::new(condition), Box::new(error)));
        }
    }
    None
}

fn try_to_super_call(name: &pt::Expression, args: &[pt::Expression]) -> Option<Expression> {
    if let pt::Expression::MemberAccess(_, box pt::Expression::Variable(var), fn_id) = name {
        if &var.name == "super" {
            return Some(Expression::SuperCall(
                fn_id.name.to_owned(),
                args.iter().map(From::from).collect(),
            ));
        }
    }
    None
}

fn try_to_variable_name(name: &pt::Expression) -> Option<String> {
    if let pt::Expression::Variable(id) = name {
        return Some(id.name.to_owned());
    }
    None
}

fn try_to_ext_contract_call(name: &pt::Expression, args: &[pt::Expression]) -> Option<Expression> {
    if let pt::Expression::MemberAccess(_, box pt::Expression::Variable(var), fn_id) = name {
        return Some(Expression::ExternalCall(
            var.name.to_owned(),
            fn_id.name.to_owned(),
            args.iter().map(From::from).collect(),
        ));
    }
    None
}

fn parse_expr(e: &pt::Expression) -> Expression {
    match e {
        pt::Expression::ArraySubscript(_, arr, key) => {
            // Eg uint[]
            if key.is_none() {
                let expr = Expression::from(&**arr);
                let ty = Expression::Type(Type::Array(Box::new(Type::try_from(&expr).unwrap())));
                return ty;
            }

            let key_expr = key
                .clone()
                .map(|key_expr| key_expr.as_ref().into())
                .expect("Unspecfied key");

            if let pt::Expression::ArraySubscript(_, arr2, key2) = &**arr {
                let key_expr2 = key2
                    .clone()
                    .map(|key_expr| key_expr.as_ref().into())
                    .expect("Unspecified key");
                let name = try_to_variable_name(arr2).expect("Collection name expected");
                Expression::Collection(name, vec![key_expr2, key_expr])
            } else {
                let name = try_to_variable_name(arr).expect("Collection name expected");
                Expression::Collection(name, vec![key_expr])
            }
        }
        pt::Expression::Assign(_, l, r) => {
            Expression::Assign(to_boxed_expr(l), Some(to_boxed_expr(r)))
        }
        pt::Expression::StringLiteral(strings) => {
            let strings = strings
                .iter()
                .map(|lit| lit.string.clone())
                .collect::<Vec<_>>();
            Expression::StringLiteral(strings.join(","))
        }
        pt::Expression::FunctionCall(_, name, args) => {
            let to_func =
                || Expression::Func(to_boxed_expr(name), args.iter().map(From::from).collect());

            try_to_zero_address(name)
                .or(try_to_super_call(name, args))
                .or(try_to_require(name, args))
                .or(try_to_ext_contract_call(name, args))
                .unwrap_or_else(to_func)
        }
        pt::Expression::Variable(id) => match id.name.as_str() {
            "_" => Expression::Placeholder,
            name => Expression::Variable(name.to_string()),
        },
        pt::Expression::MemberAccess(_, expression, id) => match expression.as_ref() {
            pt::Expression::Variable(var) => {
                if &var.name == "msg" && &id.name == "sender" {
                    Expression::Message(Message::Sender)
                } else {
                    Expression::MemberAccess(id.name.to_owned(), to_boxed_expr(expression))
                }
            }
            pt::Expression::FunctionCall(_, name, args) => {
                let expr = match &**name {
                    // expr like type(unit256).min https://docs.soliditylang.org/en/latest/units-and-global-variables.html#meta-type
                    pt::Expression::Variable(v) => {
                        if &v.name == "type" {
                            let ty = args.first().unwrap();
                            Some(Expression::TypeInfo(to_boxed_expr(ty), id.name.to_owned()))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                expr.unwrap_or(Expression::MemberAccess(
                    id.name.to_owned(),
                    to_boxed_expr(expression),
                ))
            }
            _ => Expression::MemberAccess(id.name.to_owned(), to_boxed_expr(expression)),
        },
        pt::Expression::LessEqual(_, l, r) => to_logical_expr(l, r, LogicalOp::LessEq),
        pt::Expression::Less(_, l, r) => to_logical_expr(l, r, LogicalOp::Less),
        pt::Expression::MoreEqual(_, l, r) => to_logical_expr(l, r, LogicalOp::MoreEq),
        pt::Expression::More(_, l, r) => to_logical_expr(l, r, LogicalOp::More),
        pt::Expression::NumberLiteral(_, num) => {
            let (_, u64_digits) = num.to_u64_digits();
            Expression::NumberLiteral(u64_digits)
        }
        pt::Expression::Add(_, l, r) => {
            Expression::MathOp(to_boxed_expr(l), to_boxed_expr(r), MathOp::Add)
        }
        pt::Expression::Subtract(_, l, r) => {
            Expression::MathOp(to_boxed_expr(l), to_boxed_expr(r), MathOp::Sub)
        }
        pt::Expression::Multiply(_, l, r) => {
            Expression::MathOp(to_boxed_expr(l), to_boxed_expr(r), MathOp::Mul)
        }
        pt::Expression::Divide(_, l, r) => {
            Expression::MathOp(to_boxed_expr(l), to_boxed_expr(r), MathOp::Div)
        }
        pt::Expression::Modulo(_, l, r) => {
            Expression::MathOp(to_boxed_expr(l), to_boxed_expr(r), MathOp::Modulo)
        }
        pt::Expression::Power(_, l, r) => {
            Expression::MathOp(to_boxed_expr(l), to_boxed_expr(r), MathOp::Pow)
        }
        pt::Expression::PostIncrement(_, expression) => {
            Expression::Increment(to_boxed_expr(expression))
        }
        pt::Expression::PostDecrement(_, expression) => {
            Expression::Decrement(to_boxed_expr(expression))
        }
        pt::Expression::PreIncrement(_, expression) => {
            Expression::Increment(to_boxed_expr(expression))
        }
        pt::Expression::PreDecrement(_, expression) => {
            Expression::Decrement(to_boxed_expr(expression))
        }
        pt::Expression::Equal(_, l, r) => to_logical_expr(l, r, LogicalOp::Eq),
        pt::Expression::NotEqual(_, l, r) => to_logical_expr(l, r, LogicalOp::NotEq),
        pt::Expression::AssignSubtract(_, l, r) => to_assign_op_expr(l, r, Op::Math(MathOp::Sub)),
        pt::Expression::AssignAdd(_, l, r) => to_assign_op_expr(l, r, Op::Math(MathOp::Add)),
        pt::Expression::Type(_, ty) => Expression::Type(From::from(ty)),
        pt::Expression::BoolLiteral(_, b) => Expression::BoolLiteral(*b),
        pt::Expression::Not(_, expr) => Expression::Not(to_boxed_expr(expr)),
        pt::Expression::Delete(_, expr) => Expression::Assign(to_boxed_expr(expr), None),
        pt::Expression::HexNumberLiteral(_, hex_string) => {
            // Check if the input string starts with "0x" and remove it if present.
            let hex_string = if hex_string.starts_with("0x") {
                &hex_string[2..]
            } else {
                hex_string
            };
            let bytes = hex_string_to_u8_array(hex_string).unwrap_or_default();

            Expression::BytesLiteral(bytes)
        }
        pt::Expression::HexLiteral(hex) => {
            let hex = hex.first().unwrap();
            let bytes = hex_string_to_u8_array(&hex.hex).unwrap_or_default();
            Expression::BytesLiteral(bytes)
        }
        pt::Expression::New(_, initializer) => {
            Expression::Initializer(Box::new(initializer.as_ref().into()))
        }
        pt::Expression::Complement(_, e) => Expression::UnaryOp(to_boxed_expr(e), UnaryOp::Not),
        pt::Expression::UnaryPlus(_, e) => Expression::UnaryOp(to_boxed_expr(e), UnaryOp::Plus),
        pt::Expression::UnaryMinus(_, e) => Expression::UnaryOp(to_boxed_expr(e), UnaryOp::Minus),
        pt::Expression::ShiftLeft(_, l, r) => to_bitwise_op_expr(l, r, BitwiseOp::ShiftLeft),
        pt::Expression::ShiftRight(_, l, r) => to_bitwise_op_expr(l, r, BitwiseOp::ShiftRight),
        pt::Expression::BitwiseAnd(_, l, r) => to_bitwise_op_expr(l, r, BitwiseOp::And),
        pt::Expression::BitwiseXor(_, l, r) => to_bitwise_op_expr(l, r, BitwiseOp::Xor),
        pt::Expression::BitwiseOr(_, l, r) => to_bitwise_op_expr(l, r, BitwiseOp::Or),
        pt::Expression::And(_, l, r) => to_logical_expr(l, r, LogicalOp::And),
        pt::Expression::Or(_, l, r) => to_logical_expr(l, r, LogicalOp::Or),
        pt::Expression::Ternary(_, condition, left, right) => {
            let if_else = Stmt::IfElse(
                condition.as_ref().into(),
                Box::new(Stmt::ReturningBlock(vec![Stmt::Expression(
                    left.as_ref().into(),
                )])),
                Box::new(Stmt::ReturningBlock(vec![Stmt::Expression(
                    right.as_ref().into(),
                )])),
            );
            Expression::Statement(Box::new(if_else))
        }
        pt::Expression::AssignOr(_, l, r) => to_assign_op_expr(l, r, Op::Bitwise(BitwiseOp::Or)),
        pt::Expression::AssignAnd(_, l, r) => to_assign_op_expr(l, r, Op::Bitwise(BitwiseOp::And)),
        pt::Expression::AssignXor(_, l, r) => to_assign_op_expr(l, r, Op::Bitwise(BitwiseOp::Xor)),
        pt::Expression::AssignShiftLeft(_, l, r) => {
            to_assign_op_expr(l, r, Op::Bitwise(BitwiseOp::ShiftLeft))
        }
        pt::Expression::AssignShiftRight(_, l, r) => {
            to_assign_op_expr(l, r, Op::Bitwise(BitwiseOp::ShiftRight))
        }
        pt::Expression::AssignMultiply(_, l, r) => to_assign_op_expr(l, r, Op::Math(MathOp::Mul)),
        pt::Expression::AssignDivide(_, l, r) => to_assign_op_expr(l, r, Op::Math(MathOp::Div)),
        pt::Expression::AssignModulo(_, l, r) => to_assign_op_expr(l, r, Op::Math(MathOp::Modulo)),
        pt::Expression::ArrayLiteral(_, values) => {
            let values = values.iter().map(From::from).collect();
            Expression::ArrayLiteral(values)
        }
        pt::Expression::ArraySlice(_, _, _, _) => todo!(),
        pt::Expression::FunctionCallBlock(_, _, _) => todo!(),
        pt::Expression::NamedFunctionCall(_, _, _) => todo!(),
        pt::Expression::Unit(_, _, _) => todo!(),
        pt::Expression::This(_) => todo!(),
        pt::Expression::RationalNumberLiteral(_, _) => todo!(),
        pt::Expression::AddressLiteral(_, _) => todo!(),
        pt::Expression::List(_, params) => {
            let params = params.iter().map(|(_, p)| p).collect::<Vec<_>>();
            to_tuple(params)
        }
    }
}

fn to_tuple(parameters: Vec<&Option<pt::Parameter>>) -> Expression {
    let item = parameters
        .iter()
        .map(|p| match p {
            Some(Parameter {
                ty, name: Some(id), ..
            }) => TupleItem::Declaration(Expression::from(ty), id.name.to_owned()),
            Some(Parameter { ty, name: None, .. }) => TupleItem::Expr(Expression::from(ty)),
            _ => TupleItem::Wildcard,
        })
        .collect::<Vec<_>>();
    Expression::Tuple(item)
}

fn hex_string_to_u8_array(hex_string: &str) -> Option<Vec<u8>> {
    // Check if the input string has an even number of characters (2 characters per byte).
    let hex_string = if hex_string.len() % 2 != 0 {
        ['0']
            .into_iter()
            .chain(hex_string.chars())
            .collect::<String>()
    } else {
        hex_string.to_owned()
    };

    // Use the chunks iterator to split the string into 2-character chunks.
    let hex_bytes = hex_string.as_bytes();
    let mut result = Vec::with_capacity(hex_bytes.len() / 2);

    for chunk in hex_bytes.chunks(2) {
        // Parse each 2-character chunk as a hexadecimal number.
        if let Ok(byte) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16) {
            result.push(byte);
        } else {
            // If parsing fails, return None.
            return None;
        }
    }

    Some(result)
}

fn to_assign_op_expr(l: &pt::Expression, r: &pt::Expression, op: Op) -> Expression {
    Expression::AssignAnd(to_boxed_expr(l), to_boxed_expr(r), op)
}

fn to_bitwise_op_expr(l: &pt::Expression, r: &pt::Expression, op: BitwiseOp) -> Expression {
    Expression::BitwiseOp(to_boxed_expr(l), to_boxed_expr(r), op)
}

fn to_logical_expr(l: &pt::Expression, r: &pt::Expression, op: LogicalOp) -> Expression {
    Expression::LogicalOp(to_boxed_expr(l), to_boxed_expr(r), op)
}

fn to_boxed_expr(e: &pt::Expression) -> Box<Expression> {
    Box::new(e.into())
}
