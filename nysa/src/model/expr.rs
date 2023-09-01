use solidity_parser::pt;
use syn::parse_quote;

use crate::ParserError;

use super::misc::NysaType;

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumSize {
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    I8,
    I16,
    I32,
    I64,
    I128,
    I256,
}

#[derive(Debug, Hash, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum NysaExpression {
    Require {
        condition: Box<NysaExpression>,
        error: Box<NysaExpression>,
    },
    Placeholder,
    ZeroAddress,
    Message(Message),
    Mapping {
        name: String,
        key: Box<NysaExpression>,
    },
    Mapping2 {
        name: String,
        keys: Box<(NysaExpression, NysaExpression)>,
    },
    Mapping3 {
        name: String,
        keys: Box<(NysaExpression, NysaExpression, NysaExpression)>,
    },
    Variable {
        name: String,
    },
    BoolLiteral(bool),
    StringLiteral(String),
    Assign {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    LessEqual {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    MoreEqual {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    Less {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    More {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    Add {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    Subtract {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    Power {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    Equal {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    NotEqual {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    AssignSubtract {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    AssignAdd {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    AssignDefault {
        left: Box<NysaExpression>,
    },
    Increment {
        expr: Box<NysaExpression>,
    },
    Decrement {
        expr: Box<NysaExpression>,
    },
    MemberAccess {
        expr: Box<NysaExpression>,
        name: String,
    },
    NumberLiteral {
        ty: NumSize,
        value: Vec<u8>,
    },
    Func {
        name: Box<NysaExpression>,
        args: Vec<NysaExpression>,
    },
    SuperCall {
        name: String,
        args: Vec<NysaExpression>,
    },
    ExternalCall {
        variable: String,
        fn_name: String,
        args: Vec<NysaExpression>,
    },
    TypeInfo {
        ty: Box<NysaExpression>,
        property: String,
    },
    Type {
        ty: NysaType,
    },
    Not {
        expr: Box<NysaExpression>,
    },
    UnknownExpr,
}

pub fn to_nysa_expr(solidity_expressions: Vec<pt::Expression>) -> Vec<NysaExpression> {
    solidity_expressions.iter().map(From::from).collect()
}

impl From<&pt::Expression> for NysaExpression {
    fn from(value: &pt::Expression) -> Self {
        parse_expr(value)
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

fn try_to_zero_address(name: &pt::Expression) -> Option<NysaExpression> {
    if let pt::Expression::Type(_, ty) = name {
        if *ty == pt::Type::Address || *ty == pt::Type::AddressPayable {
            return Some(NysaExpression::ZeroAddress);
        }
    }
    None
}

fn try_to_require(name: &pt::Expression, args: &[pt::Expression]) -> Option<NysaExpression> {
    if let pt::Expression::Variable(ref id) = name {
        if id.name.as_str() == "require" {
            let condition = args.get(0).expect("Should be revert condition").into();
            let error = args.get(1).expect("Should be the error message").into();
            return Some(NysaExpression::Require {
                condition: Box::new(condition),
                error: Box::new(error),
            });
        }
    }
    None
}

fn try_to_super_call(name: &pt::Expression, args: &[pt::Expression]) -> Option<NysaExpression> {
    if let pt::Expression::MemberAccess(_, box pt::Expression::Variable(var), fn_id) = name {
        if &var.name == "super" {
            return Some(NysaExpression::SuperCall {
                name: fn_id.name.to_owned(),
                args: args.iter().map(From::from).collect(),
            });
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

fn try_to_ext_contract_call(
    name: &pt::Expression,
    args: &[pt::Expression],
) -> Option<NysaExpression> {
    if let pt::Expression::MemberAccess(_, box pt::Expression::Variable(var), fn_id) = name {
        return Some(NysaExpression::ExternalCall {
            variable: var.name.to_owned(),
            fn_name: fn_id.name.to_owned(),
            args: args.iter().map(From::from).collect(),
        });
    }
    None
}

fn parse_expr(e: &pt::Expression) -> NysaExpression {
    match e {
        pt::Expression::ArraySubscript(_, arr, key) => {
            let key_expr = key
                .clone()
                .map(|key_expr| key_expr.as_ref().into())
                .expect("Unspecfied key");

            if let pt::Expression::ArraySubscript(_, arr2, key2) = &**arr {
                let key_expr2 = key2
                    .clone()
                    .map(|key_expr| key_expr.as_ref().into())
                    .expect("Unspecfied key");
                let name = try_to_variable_name(arr2).expect("Mapping name expected");
                NysaExpression::Mapping2 {
                    name,
                    keys: Box::new((key_expr2, key_expr)),
                }
            } else {
                let name = try_to_variable_name(arr).expect("Mapping name expected");
                NysaExpression::Mapping {
                    name,
                    key: Box::new(key_expr),
                }
            }
        }
        pt::Expression::Assign(_, l, r) => NysaExpression::Assign {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::StringLiteral(strings) => {
            let strings = strings
                .iter()
                .map(|lit| lit.string.clone())
                .collect::<Vec<_>>();
            NysaExpression::StringLiteral(strings.join(","))
        }
        pt::Expression::FunctionCall(_, name, args) => {
            let to_func = || NysaExpression::Func {
                name: Box::new(name.as_ref().into()),
                args: args.iter().map(From::from).collect(),
            };

            try_to_zero_address(name)
                .or(try_to_super_call(name, args))
                .or(try_to_require(name, args))
                .or(try_to_ext_contract_call(name, args))
                .unwrap_or_else(to_func)
        }
        pt::Expression::Variable(id) => match id.name.as_str() {
            "_" => NysaExpression::Placeholder,
            name => NysaExpression::Variable {
                name: name.to_string(),
            },
        },
        pt::Expression::MemberAccess(_, expression, id) => match expression.as_ref() {
            pt::Expression::Variable(var) => {
                if &var.name == "msg" && &id.name == "sender" {
                    NysaExpression::Message(Message::Sender)
                } else {
                    NysaExpression::MemberAccess {
                        expr: Box::new(expression.as_ref().into()),
                        name: id.name.to_owned(),
                    }
                }
            }
            pt::Expression::FunctionCall(_, name, args) => {
                let expr = match &**name {
                    // expr like type(unit256).min https://docs.soliditylang.org/en/latest/units-and-global-variables.html#meta-type
                    pt::Expression::Variable(v) => {
                        if &v.name == "type" {
                            let ty = args.first().unwrap();
                            Some(NysaExpression::TypeInfo {
                                ty: Box::new(ty.into()),
                                property: id.name.to_owned(),
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                expr.unwrap_or(NysaExpression::MemberAccess {
                    expr: Box::new(expression.as_ref().into()),
                    name: id.name.to_owned(),
                })
            }
            _ => NysaExpression::MemberAccess {
                expr: Box::new(expression.as_ref().into()),
                name: id.name.to_owned(),
            },
        },
        pt::Expression::LessEqual(_, l, r) => NysaExpression::LessEqual {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::Less(_, l, r) => NysaExpression::Less {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::MoreEqual(_, l, r) => NysaExpression::MoreEqual {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::More(_, l, r) => NysaExpression::More {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::NumberLiteral(_, num) => {
            let (sign, digs) = num.to_u32_digits();

            // u32::MAX or less
            if digs.is_empty() {
                return NysaExpression::NumberLiteral {
                    ty: NumSize::U8,
                    value: vec![],
                };
            }
            if digs.len() == 1 {
                let value = digs[0];
                let mut ty = NumSize::U32;
                if value <= u8::MAX.into() {
                    ty = NumSize::U8;
                } else if value <= u16::MAX.into() {
                    ty = NumSize::U16;
                }
                NysaExpression::NumberLiteral {
                    ty,
                    value: digs[0].to_le_bytes().to_vec(),
                }
            } else {
                let (sign, digs) = num.to_u64_digits();
                // u32::MAX..u64::MAX
                if digs.len() == 1 {
                    NysaExpression::NumberLiteral {
                        ty: NumSize::U64,
                        value: digs[0].to_le_bytes().to_vec(),
                    }
                } else {
                    let (_, bytes) = num.to_bytes_le();
                    NysaExpression::NumberLiteral {
                        ty: NumSize::U256,
                        value: bytes,
                    }
                }
            }
        }
        pt::Expression::Add(_, l, r) => NysaExpression::Add {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::Subtract(_, l, r) => NysaExpression::Subtract {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::PostIncrement(_, expression) => NysaExpression::Increment {
            expr: Box::new(expression.as_ref().into()),
        },
        pt::Expression::PostDecrement(_, expression) => NysaExpression::Decrement {
            expr: Box::new(expression.as_ref().into()),
        },
        pt::Expression::PreIncrement(_, expression) => NysaExpression::Increment {
            expr: Box::new(expression.as_ref().into()),
        },
        pt::Expression::PreDecrement(_, expression) => NysaExpression::Decrement {
            expr: Box::new(expression.as_ref().into()),
        },
        pt::Expression::Equal(_, l, r) => NysaExpression::Equal {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::NotEqual(_, l, r) => NysaExpression::NotEqual {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::AssignSubtract(_, l, r) => NysaExpression::AssignSubtract {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::AssignAdd(_, l, r) => NysaExpression::AssignAdd {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::Type(_, ty) => NysaExpression::Type { ty: From::from(ty) },
        pt::Expression::Power(_, l, r) => NysaExpression::Power {
            left: Box::new(l.as_ref().into()),
            right: Box::new(r.as_ref().into()),
        },
        pt::Expression::BoolLiteral(_, b) => NysaExpression::BoolLiteral(*b),
        pt::Expression::Not(_, expr) => NysaExpression::Not {
            expr: Box::new(expr.as_ref().into()),
        },
        pt::Expression::Delete(_, expr) => NysaExpression::AssignDefault {
            left: Box::new(expr.as_ref().into()),
        },
        _ => NysaExpression::UnknownExpr,
    }
}
