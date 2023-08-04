use solidity_parser::pt;
use syn::parse_quote;

#[allow(dead_code)]
pub enum Message {
    Sender,
    Value,
    Data,
}

impl TryInto<syn::Expr> for Message {
    type Error = &'static str;

    fn try_into(self) -> Result<syn::Expr, Self::Error> {
        match self {
            Message::Sender => Ok(parse_quote!(Some(odra::contract_env::caller()))),
            Message::Value => todo!(),
            Message::Data => todo!(),
        }
    }
}

pub enum Expression<'a> {
    Require {
        condition: &'a pt::Expression,
        error: &'a pt::Expression,
    },
    Wildcard,
    ZeroAddress,
    Expr(&'a pt::Expression),
    Message(Message),
}

impl<'a> From<&'a pt::Expression> for Expression<'a> {
    fn from(value: &'a pt::Expression) -> Self {
        match value {
            pt::Expression::FunctionCall(_, name, args) => {
                if let pt::Expression::Type(_, ref ty) = **name {
                    if *ty == pt::Type::Address || *ty == pt::Type::AddressPayable {
                        return Expression::ZeroAddress;
                    }
                }
                if let pt::Expression::Variable(ref id) = **name {
                    if id.name.as_str() == "require" {
                        let condition = args.get(0).expect("Should be revert condition");
                        let error = args.get(1).expect("Should be the error message");
                        return Expression::Require { condition, error };
                    }
                }
                return Expression::Expr(value);
            }
            pt::Expression::Variable(id) => match id.name.as_str() {
                "_" => Expression::Wildcard,
                _ => Expression::Expr(value),
            },
            pt::Expression::MemberAccess(_, expression, id) => match expression.as_ref() {
                pt::Expression::Variable(var) => {
                    if &var.name == "msg" && &id.name == "sender" {
                        Expression::Message(Message::Sender)
                    } else {
                        Expression::Expr(value)
                    }
                }
                _ => Expression::Expr(value),
            },
            _ => Expression::Expr(value),
        }
    }
}
