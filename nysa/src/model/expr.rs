use solidity_parser::pt::{self, Parameter};
use syn::parse_quote;

use crate::{formatted_invalid_expr, parser::context::*, ParserError};

use super::{
    misc::{Type, Var},
    op::{BitwiseOp, LogicalOp, MathOp, Op, UnaryOp},
    stmt::Stmt,
    RESERVED_NAMES,
};

/// Represents a single expression to parse.
///
/// This is an intermediate representation between a solidity statement and the ultimate rust
/// representation.
///
/// An expression is intended to be parsed into [syn::Expr](syn::Expr).
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Expression {
    /// Error handling expression eg. `require(c >= a, "SafeMath: addition overflow");`
    Require(Box<Expression>, Box<Expression>),
    /// A special `_` occurring in modifiers.
    Placeholder,
    /// A special `address(0)` expr.
    ZeroAddress,
    /// keccak256 function call.
    Keccak256(Vec<Expression>),
    /// abi.encodePacked function call.
    AbiEncodePacked(Vec<Expression>),
    /// msg.* expr (eg. msg.sender).
    Message(Message),
    /// A collection access (local array, state array/mapping)
    Collection(String, Vec<Expression>),
    /// A variable access (local, state).
    Variable(String),
    /// Boolean literal (false, true).
    BoolLiteral(bool),
    /// String literal ("some string").
    StringLiteral(String),
    /// Assign operation `left = right`. If the right-hand side expression if None, the default value should be assign.
    Assign(Box<Expression>, Option<Box<Expression>>),
    /// Logical operation eg. `x & y``.
    LogicalOp(Box<Expression>, Box<Expression>, LogicalOp),
    /// Mathematical operation eg. `2 + 4``.
    MathOp(Box<Expression>, Box<Expression>, MathOp),
    /// Assign and operation expr like `x =* y`, `x =& y`.
    AssignAnd(Box<Expression>, Box<Expression>, Op),
    /// Pre/post increment.
    Increment(Box<Expression>),
    /// Pre/post decrement.
    Decrement(Box<Expression>),
    /// Object member access, eg. `obj.value`.
    MemberAccess(String, Box<Expression>),
    /// Number literal, eg. `123`.
    NumberLiteral(Vec<u64>),
    /// A regular function call.
    Func(Box<Expression>, Vec<Expression>),
    /// Super function call.
    SuperCall(String, Vec<Expression>),
    /// External contract call.
    ExternalCall(String, String, Vec<Expression>),
    /// Read a type property expr (eg. u32.max).
    TypeInfo(Box<Expression>, String),
    /// Type expression (eg. `String`, `u32`, etc.)
    Type(Type),
    /// Negation expr eg `!x;`
    Not(Box<Expression>),
    /// Bytes literal eg `0x00af;` or `hex"02ff";`
    BytesLiteral(Vec<u8>),
    /// Array literal eg `[1, 2, 3];`
    ArrayLiteral(Vec<Expression>),
    /// Variable init expression.
    Initializer(Box<Expression>),
    /// A [Stmt].
    Statement(Box<Stmt>),
    BitwiseOp(Box<Expression>, Box<Expression>, BitwiseOp),
    /// One of unary expression eg `~x`, `-x`, `+x`.
    UnaryOp(Box<Expression>, UnaryOp),
    /// A tuple expr eg. `(x, y, z)`.
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
            _ => formatted_invalid_expr!("variable expected"),
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Message {
    Sender,
    Value,
    Data,
}

impl TryInto<syn::Expr> for &Message {
    type Error = ParserError;

    fn try_into(self) -> Result<syn::Expr, Self::Error> {
        match self {
            Message::Sender => Ok(parse_quote!(Some(self.env().caller()))),
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

fn try_to_keccak(name: &pt::Expression, args: &[pt::Expression]) -> Option<Expression> {
    if let pt::Expression::Variable(var) = name {
        if &var.name == "keccak256" {
            return Some(Expression::Keccak256(args.iter().map(From::from).collect()));
        }
    }
    None
}

fn try_to_abi_encode(name: &pt::Expression, args: &[pt::Expression]) -> Option<Expression> {
    if let pt::Expression::MemberAccess(_, box pt::Expression::Variable(var), fn_id) = name {
        if &var.name == "abi" && &fn_id.name == "encodePacked" {
            return Some(Expression::AbiEncodePacked(
                args.iter().map(From::from).collect(),
            ));
        }
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
                .or(try_to_keccak(name, args))
                .or(try_to_abi_encode(name, args))
                .or(try_to_super_call(name, args))
                .or(try_to_require(name, args))
                .or(try_to_ext_contract_call(name, args))
                .unwrap_or_else(to_func)
        }
        pt::Expression::Variable(id) => match id.name.as_str() {
            "_" => Expression::Placeholder,
            name => {
                if RESERVED_NAMES.contains(&name) {
                    Expression::Variable(format!("_{}", name))
                } else {
                    Expression::Variable(name.to_string())
                }
            }
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

pub fn eval_expression_type<T>(expr: &Expression, ctx: &T) -> Option<Type>
where
    T: TypeInfo + ContractInfo,
{
    match expr {
        Expression::Require(_, _) => None,
        Expression::Placeholder => None,
        Expression::ZeroAddress => Some(Type::Address),
        Expression::Message(msg) => match msg {
            Message::Sender => Some(Type::Address),
            Message::Value => None,
            Message::Data => None,
        },
        Expression::Collection(name, key) => ctx
            .type_from_string(name)
            .map(|t| match t {
                ItemType::Contract(_) => None,
                ItemType::Library(_) => None,
                ItemType::Interface(_) => None,
                ItemType::Enum(e) => Some(Type::Custom(e)),
                ItemType::Struct(s) => Some(Type::Custom(s.name)),
                ItemType::Event => None,
                ItemType::Storage(Var {
                    ty: Type::Mapping(t, ta),
                    ..
                }) => eval_expression_type(&t, ctx),
                ItemType::Storage(v) => Some(v.ty.clone()),
                ItemType::Local(Var {
                    ty: Type::Array(t), ..
                }) => Some(*t),
                ItemType::Local(v) => Some(v.ty.clone()),
            })
            .flatten(),
        Expression::Variable(name) => ctx
            .type_from_string(name)
            .map(|t| match t {
                ItemType::Contract(_) => None,
                ItemType::Library(_) => None,
                ItemType::Interface(_) => None,
                ItemType::Enum(e) => Some(Type::Custom(e)),
                ItemType::Struct(s) => Some(Type::Custom(s.name)),
                ItemType::Event => None,
                ItemType::Storage(Var {
                    ty: Type::Array(t), ..
                }) => Some(*t),
                ItemType::Storage(v) => Some(v.ty.clone()),
                ItemType::Local(Var {
                    ty: Type::Array(t), ..
                }) => Some(*t),
                ItemType::Local(v) => Some(v.ty.clone()),
            })
            .flatten(),
        Expression::BoolLiteral(_) => Some(Type::Bool),
        Expression::StringLiteral(_) => Some(Type::String),
        Expression::Assign(l, _) => eval_expression_type(l, ctx),
        Expression::LogicalOp(_, _, _) => Some(Type::Bool),
        Expression::MathOp(l, r, op) => {
            let lty = eval_expression_type(l, ctx);
            let rty = eval_expression_type(r, ctx);
            match (lty, rty) {
                (Some(Type::Uint(s1)), Some(Type::Uint(s2))) => Some(Type::Uint(u16::max(s1, s2))),
                (None, Some(Type::Uint(s))) => Some(Type::Uint(s)),
                (Some(Type::Uint(s)), None) => Some(Type::Uint(s)),
                (Some(Type::Int(s1)), Some(Type::Int(s2))) => Some(Type::Int(u16::max(s1, s2))),
                (None, Some(Type::Int(s))) => Some(Type::Int(s)),
                (Some(Type::Int(s)), None) => Some(Type::Int(s)),
                (Some(Type::Bytes(s1)), Some(Type::Bytes(s2))) => {
                    Some(Type::Bytes(u8::max(s1, s2)))
                }
                _ => None,
            }
        }
        Expression::AssignAnd(l, _, _) => eval_expression_type(l, ctx),
        Expression::Increment(e) => eval_expression_type(e, ctx),
        Expression::Decrement(e) => eval_expression_type(e, ctx),
        Expression::MemberAccess(name, e) => {
            if let Expression::MemberAccess(nested_name, nested_e) = &**e {
                let ty = eval_expression_type(nested_e, ctx).unwrap();
                match ty {
                    Type::Custom(class_name) => {
                        if let Some(ItemType::Struct(s)) = ctx.type_from_string(&class_name) {
                            let f = s.fields.iter().find(|f| &f.0 == nested_name).unwrap();
                            return eval_expression_type(&f.1, ctx);
                        } else {
                            todo!()
                        }
                    }
                    _ => todo!(),
                }
            }
            match ctx.type_from_expression(e) {
                Some(ItemType::Enum(ty)) => Some(Type::Custom(ty)),
                Some(ItemType::Struct(ty)) => {
                    if let Some((name, fty)) = ty.fields.iter().find(|(f, t)| f == &ty.name) {
                        eval_expression_type(fty, ctx)
                    } else {
                        None
                    }
                }
                Some(ItemType::Library(ty)) => ty
                    .vars()
                    .iter()
                    .find(|v| &v.name == name)
                    .map(|v| v.ty.clone()),
                Some(ItemType::Storage(Var { ty, .. })) => Some(ty),
                Some(ItemType::Local(Var {
                    ty: Type::Custom(struct_name),
                    ..
                })) => {
                    if let Some(ItemType::Struct(ty)) = ctx.type_from_string(&struct_name) {
                        if let Some((name, fty)) = ty.fields.iter().find(|(f, t)| f == name) {
                            eval_expression_type(fty, ctx)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some(ItemType::Local(Var { ty, .. })) => Some(ty),
                e => panic!("{:?}", e),
            }
        }
        Expression::NumberLiteral(_) => None,
        Expression::Func(f, args) => {
            if let Expression::MemberAccess(function_name, ty_expr) = &**f {
                let ty = eval_expression_type(ty_expr, ctx);

                // find matching lib
                let matching_lib = ctx
                    .current_contract()
                    .libs()
                    .iter()
                    .find(|lib| eval_expression_type(&lib.ty, ctx) == ty)
                    .unwrap();

                let matching_fn = ctx.find_fn(&matching_lib.name, function_name).unwrap();
                return matching_fn.ret_ty(ctx);
            }
            if let Expression::Type(t) = &**f {
                return Some(t.clone());
            }
            todo!()
        }
        Expression::SuperCall(_, _) => todo!(),
        Expression::ExternalCall(_, _, _) => todo!(),
        Expression::TypeInfo(_, _) => todo!(),
        Expression::Type(t) => Some(t.clone()),
        Expression::Not(e) => eval_expression_type(e, ctx),
        Expression::BytesLiteral(b) => Some(Type::Bytes(b.len() as u8)),
        Expression::ArrayLiteral(_) => todo!(),
        Expression::Initializer(_) => todo!(),
        Expression::Statement(s) => todo!(),
        Expression::BitwiseOp(_, _, _) => None,
        Expression::UnaryOp(_, _) => todo!(),
        Expression::Tuple(_) => None,
        #[cfg(test)]
        Expression::Fail => None,
        Expression::Keccak256(_) => Some(Type::Bytes(32)),
        Expression::AbiEncodePacked(_) => Some(Type::Array(Box::new(Type::Uint(8)))),
    }
}
