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
        ty: &'static str,
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
    TypeInfo {
        ty: Box<NysaExpression>,
        property: String,
    },
    Type {
        ty: pt::Type,
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
                let to_func = || NysaExpression::Func {
                    name: Box::new(name.as_ref().into()),
                    args: args.iter().map(From::from).collect(),
                };

                try_to_zero_address(name)
                    .or(try_to_super_call(name, args))
                    .or(try_to_require(name, args))
                    .unwrap_or_else(to_func)
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
                pt::Expression::FunctionCall(_, name, args) => {
                    let expr = match &**name {
                        // expr like type(unit256).min https://docs.soliditylang.org/en/latest/units-and-global-variables.html#meta-type
                        pt::Expression::Variable(v) => {
                            if &v.name == "type" {
                                let ty = args.first().unwrap();
                                Some(Self::TypeInfo {
                                    ty: Box::new(ty.into()),
                                    property: id.name.to_owned(),
                                })
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    expr.unwrap_or(Self::MemberAccess {
                        expr: Box::new(expression.as_ref().into()),
                        name: id.name.to_owned(),
                    })
                }
                _ => Self::MemberAccess {
                    expr: Box::new(expression.as_ref().into()),
                    name: id.name.to_owned(),
                },
            },
            pt::Expression::LessEqual(_, l, r) => Self::LessEqual {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::Less(_, l, r) => Self::Less {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::MoreEqual(_, l, r) => Self::MoreEqual {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::More(_, l, r) => Self::More {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::NumberLiteral(_, num) => {
                let (sign, digs) = num.to_u32_digits();

                // u32::MAX or less
                if digs.len() == 1 {
                    Self::NumberLiteral {
                        ty: "u32",
                        value: digs[0].to_le_bytes().to_vec(),
                    }
                } else {
                    let (sign, digs) = num.to_u64_digits();
                    // u32::MAX..u64::MAX
                    if digs.len() == 1 {
                        Self::NumberLiteral {
                            ty: "u64",
                            value: digs[0].to_le_bytes().to_vec(),
                        }
                    } else {
                        let (_, bytes) = num.to_bytes_le();
                        Self::NumberLiteral {
                            ty: "U256",
                            value: bytes,
                        }
                    }
                }
            }
            pt::Expression::Add(_, l, r) => Self::Add {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::Subtract(_, l, r) => Self::Subtract {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::PostIncrement(_, expression) => Self::Increment {
                expr: Box::new(expression.as_ref().into()),
            },
            pt::Expression::PostDecrement(_, expression) => Self::Decrement {
                expr: Box::new(expression.as_ref().into()),
            },
            pt::Expression::PreIncrement(_, expression) => Self::Increment {
                expr: Box::new(expression.as_ref().into()),
            },
            pt::Expression::PreDecrement(_, expression) => Self::Decrement {
                expr: Box::new(expression.as_ref().into()),
            },
            pt::Expression::Equal(_, l, r) => Self::Equal {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::NotEqual(_, l, r) => Self::NotEqual {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::AssignSubtract(_, l, r) => Self::AssignSubtract {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::AssignAdd(_, l, r) => Self::AssignAdd {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::Type(_, ty) => Self::Type { ty: ty.clone() },
            pt::Expression::Power(_, l, r) => Self::Power {
                left: Box::new(l.as_ref().into()),
                right: Box::new(r.as_ref().into()),
            },
            pt::Expression::BoolLiteral(_, b) => Self::BoolLiteral(*b),
            _ => Self::Expr(value.to_owned()),
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
