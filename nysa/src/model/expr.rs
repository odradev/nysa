use solidity_parser::pt;
use syn::parse_quote;

#[derive(Debug, Clone)]
pub enum NysaExpression {
    Require {
        condition: Box<NysaExpression>,
        error: Box<NysaExpression>,
    },
    Wildcard,
    ZeroAddress,
    Message(Message),
    ArraySubscript {
        array: Box<NysaExpression>,
        key: Box<Option<NysaExpression>>,
    },
    Variable {
        name: String,
    },
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
    Add {
        left: Box<NysaExpression>,
        right: Box<NysaExpression>,
    },
    Subtract {
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
    NumberLiteral(u32),
    Func {
        name: Box<NysaExpression>,
        args: Vec<NysaExpression>,
    },
    Expr(pt::Expression),
}

pub fn to_nysa_expr(solidity_expressions: Vec<pt::Expression>) -> Vec<NysaExpression> {
    solidity_expressions.iter().map(From::from).collect()
}

impl From<&pt::Expression> for NysaExpression {
    fn from(value: &pt::Expression) -> Self {
        match value {
            pt::Expression::ArraySubscript(_, arr, key) => {
                let key_expr = key.clone().map(|key_expr| key_expr.as_ref().into());

                NysaExpression::ArraySubscript {
                    array: Box::new(arr.as_ref().into()),
                    key: Box::new(key_expr),
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
                if let pt::Expression::Type(_, ref ty) = **name {
                    if *ty == pt::Type::Address || *ty == pt::Type::AddressPayable {
                        return NysaExpression::ZeroAddress;
                    }
                }
                if let pt::Expression::Variable(ref id) = **name {
                    if id.name.as_str() == "require" {
                        let condition = args.get(0).expect("Should be revert condition").into();
                        let error = args.get(1).expect("Should be the error message").into();
                        return NysaExpression::Require {
                            condition: Box::new(condition),
                            error: Box::new(error),
                        };
                    }
                }
                return NysaExpression::Func {
                    name: Box::new(name.as_ref().into()),
                    args: args.iter().map(From::from).collect(),
                };
            }
            pt::Expression::Variable(id) => match id.name.as_str() {
                "_" => NysaExpression::Wildcard,
                name => NysaExpression::Variable {
                    name: name.to_string(),
                },
            },
            pt::Expression::MemberAccess(_, expression, id) => match expression.as_ref() {
                pt::Expression::Variable(var) => {
                    if &var.name == "msg" && &id.name == "sender" {
                        NysaExpression::Message(Message::Sender)
                    } else {
                        NysaExpression::Variable {
                            name: var.name.to_owned(),
                        }
                    }
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
            pt::Expression::MoreEqual(_, l, r) => NysaExpression::MoreEqual {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::NumberLiteral(_, num) => {
                let (sign, digs) = num.to_u32_digits();
                let num = digs[0];
                NysaExpression::NumberLiteral(num)
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
            _ => NysaExpression::Expr(value.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    Sender,
    Value,
    Data,
}

impl TryInto<syn::Expr> for &Message {
    type Error = &'static str;

    fn try_into(self) -> Result<syn::Expr, Self::Error> {
        match self {
            Message::Sender => Ok(parse_quote!(Some(odra::contract_env::caller()))),
            Message::Value => todo!(),
            Message::Data => todo!(),
        }
    }
}
