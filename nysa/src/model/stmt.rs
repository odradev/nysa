use solidity_parser::pt;

use super::{expr::NysaExpression, misc::NysaType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NysaStmt {
    ReturnVoid,
    Return {
        expr: NysaExpression,
    },
    Expression {
        expr: NysaExpression,
    },
    VarDefinition {
        declaration: String,
        ty: NysaType,
        init: NysaExpression,
    },
    VarDeclaration {
        declaration: String,
        ty: NysaType,
    },
    If {
        assertion: NysaExpression,
        if_body: Box<NysaStmt>,
    },
    IfElse {
        assertion: NysaExpression,
        if_body: Box<NysaStmt>,
        else_body: Box<NysaStmt>,
    },
    Block {
        stmts: Vec<NysaStmt>,
    },
    Emit {
        expr: NysaExpression,
    },
    RevertWithError {
        error: String,
    },
    Revert {
        msg: Option<NysaExpression>,
    },
    Placeholder,
    While {
        assertion: NysaExpression,
        block: Box<NysaStmt>,
    },

    Unknown,
}

impl From<&pt::Statement> for NysaStmt {
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
                let expr: NysaExpression = expr.into();
                if expr == NysaExpression::Placeholder {
                    Self::Placeholder
                } else {
                    Self::Expression { expr }
                }
            }
            pt::Statement::VariableDefinition(_, declaration, init) => {
                let name = declaration.name.name.clone();
                let ty = NysaExpression::from(&declaration.ty);
                let ty = NysaType::try_from(&ty).unwrap_or(NysaType::Unknown);
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
