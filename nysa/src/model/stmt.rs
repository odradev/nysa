use solidity_parser::pt;

use super::{expr::Expression, misc::Type};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    ReturnVoid,
    Return {
        expr: Expression,
    },
    Expression {
        expr: Expression,
    },
    VarDefinition {
        declaration: String,
        ty: Type,
        init: Expression,
    },
    VarDeclaration {
        declaration: String,
        ty: Type,
    },
    If {
        assertion: Expression,
        if_body: Box<Stmt>,
    },
    IfElse {
        assertion: Expression,
        if_body: Box<Stmt>,
        else_body: Box<Stmt>,
    },
    Block {
        stmts: Vec<Stmt>,
    },
    Emit {
        expr: Expression,
    },
    RevertWithError {
        error: String,
    },
    Revert {
        msg: Option<Expression>,
    },
    Placeholder,
    While {
        assertion: Expression,
        block: Box<Stmt>,
    },

    Unknown,
}

impl From<&pt::Statement> for Stmt {
    fn from(value: &pt::Statement) -> Self {
        match value {
            pt::Statement::Block {
                loc,
                unchecked,
                statements,
            } => Self::Block {
                stmts: statements.iter().map(From::from).collect(),
            },
            pt::Statement::Assembly {
                loc,
                dialect,
                statements,
            } => todo!(),
            pt::Statement::Args(_, _) => todo!(),
            pt::Statement::If(_, assertion, if_body, else_body) => match else_body {
                Some(else_body) => Self::IfElse {
                    assertion: assertion.into(),
                    if_body: Box::new(if_body.as_ref().into()),
                    else_body: Box::new(else_body.as_ref().into()),
                },
                None => Self::If {
                    assertion: assertion.into(),
                    if_body: Box::new(if_body.as_ref().into()),
                },
            },
            pt::Statement::While(_, assertion, block) => Self::While {
                assertion: assertion.into(),
                block: Box::new(block.as_ref().into()),
            },
            pt::Statement::Expression(_, expr) => {
                let expr: Expression = expr.into();
                if expr == Expression::Placeholder {
                    Self::Placeholder
                } else {
                    Self::Expression { expr }
                }
            }
            pt::Statement::VariableDefinition(_, declaration, init) => {
                let name = declaration.name.name.clone();
                let ty = Expression::from(&declaration.ty);
                let ty = Type::try_from(&ty).unwrap_or(Type::Unknown);
                match init {
                    Some(expr) => Self::VarDefinition {
                        declaration: name,
                        ty,
                        init: expr.into(),
                    },
                    None => Self::VarDeclaration {
                        declaration: name,
                        ty,
                    },
                }
            }
            pt::Statement::For(_, _, _, _, _) => Self::Unknown,
            pt::Statement::DoWhile(_, _, _) => Self::Unknown,
            pt::Statement::Continue(_) => Self::Unknown,
            pt::Statement::Break(_) => Self::Unknown,
            pt::Statement::Return(_, r) => match r {
                Some(expr) => Self::Return { expr: expr.into() },
                None => Self::ReturnVoid,
            },
            pt::Statement::Revert(_, error_id, err) => {
                if let Some(id) = error_id {
                    Self::RevertWithError {
                        error: id.name.to_owned(),
                    }
                } else {
                    if err.is_empty() {
                        Self::Revert { msg: None }
                    } else {
                        Self::Revert {
                            msg: err.first().map(|e| e.into()),
                        }
                    }
                }
            }
            pt::Statement::Emit(_, expr) => Self::Emit { expr: expr.into() },
            pt::Statement::Try(_, _, _, _) => Self::Unknown,
            pt::Statement::DocComment(_, _, _) => Self::Unknown,
        }
    }
}
