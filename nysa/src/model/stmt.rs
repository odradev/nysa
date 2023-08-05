use solidity_parser::pt;

use super::expr::NysaExpression;

#[derive(Debug)]
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
        init: NysaExpression,
    },
    VarDeclaration {
        declaration: String,
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
            pt::Statement::While(_, _, _) => Self::Unknown,
            pt::Statement::Expression(_, expr) => Self::Expression { expr: expr.into() },
            pt::Statement::VariableDefinition(_, declaration, init) => match init {
                Some(expr) => Self::VarDefinition {
                    declaration: declaration.name.name.clone(),
                    init: expr.into(),
                },
                None => Self::VarDeclaration {
                    declaration: declaration.name.name.clone(),
                },
            },
            pt::Statement::For(_, _, _, _, _) => Self::Unknown,
            pt::Statement::DoWhile(_, _, _) => Self::Unknown,
            pt::Statement::Continue(_) => Self::Unknown,
            pt::Statement::Break(_) => Self::Unknown,
            pt::Statement::Return(_, r) => match r {
                Some(expr) => Self::Return { expr: expr.into() },
                None => Self::ReturnVoid,
            },
            pt::Statement::Revert(_, _, _) => Self::Unknown,
            pt::Statement::Emit(_, expr) => Self::Emit { expr: expr.into() },
            pt::Statement::Try(_, _, _, _) => Self::Unknown,
            pt::Statement::DocComment(_, _, _) => Self::Unknown,
        }
    }
}
