use solidity_parser::pt::{self, Statement};

use super::{expr::Expression, misc::Type};

/// An individual statement representation.
///
/// This is an intermediate representation between a solidity statement and the ultimate rust
/// representation.
///
/// A statement is intended to be parsed into syn::Stmt.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    /// Return statement that does not return any value.
    ReturnVoid,
    /// Return statement with the returning expression.
    Return(Expression),
    /// An [Expression]
    Expression(Expression),
    /// Variable definition with the name, type and the initializing expression.
    VarDefinition(String, Type, Expression),
    /// Variable declaration with the name, and the type.
    VarDeclaration(String, Type),
    /// If expression with the condition, and the conditional statement.
    If(Expression, Box<Stmt>),
    /// If expression with the condition expression, the conditional statement, the fallback statement.
    IfElse(Expression, Box<Stmt>, Box<Stmt>),
    /// A regular block of statements.
    Block(Vec<Stmt>),
    /// A block that the last statement returns a value.
    ReturningBlock(Vec<Stmt>),
    /// Emit event statement.
    Emit(Expression),
    /// Revert statement with a string message.
    RevertWithError(String),
    /// Revert statement with a complex error expression.
    Revert(Option<Expression>),
    /// _ statement.
    Placeholder,
    /// While loop with the condition, and the conditional block.
    While(Expression, Box<Stmt>),
    /// Line or block comment.
    DocComment(String),
    /// Unknown statement.
    Unknown,
    #[cfg(test)]
    /// A statement that cannot be parsed. Used to fail fast in a test.
    Fail,
}

pub fn contains_sender_expr(stmts: &[Stmt]) -> bool {
    stmts.iter().any(Stmt::contains_sender_expr)
}

impl Stmt {
    /// Returns true if the statement contains Expression::Message.
    pub fn contains_sender_expr(&self) -> bool {
        match self {
            Stmt::Return(expr) => expr.is_sender_expr(),
            Stmt::VarDefinition(_, _, expr) => expr.is_sender_expr(),
            Stmt::VarDeclaration(_, _) => false,
            Stmt::If(cond, if_body) => cond.is_sender_expr() || if_body.contains_sender_expr(),
            Stmt::IfElse(cond, if_body, else_body) => {
                cond.is_sender_expr()
                    || if_body.contains_sender_expr()
                    || else_body.contains_sender_expr()
            }
            Stmt::Block(statements) => statements.iter().any(Stmt::contains_sender_expr),
            Stmt::ReturningBlock(statements) => statements.iter().any(Stmt::contains_sender_expr),
            Stmt::Emit(expr) => expr.is_sender_expr(),
            Stmt::RevertWithError(_) => false,
            Stmt::Revert(Some(expr)) => expr.is_sender_expr(),
            Stmt::Revert(None) => false,
            Stmt::Placeholder => false,
            Stmt::While(cond, body) => cond.is_sender_expr() || body.contains_sender_expr(),
            Stmt::DocComment(_) => false,
            Stmt::Unknown => false,
            #[cfg(test)]
            Stmt::Fail => false,
            Stmt::ReturnVoid => false,
            Stmt::Expression(expr) => expr.is_sender_expr(),
        }
    }

    pub fn function_calls(&self) -> Vec<String> {
        match self {
            Stmt::Return(expr) => expr.func_call(),
            Stmt::VarDefinition(_, _, expr) => expr.func_call(),
            Stmt::VarDeclaration(_, _) => vec![],
            Stmt::If(cond, if_body) => [cond.func_call(), if_body.function_calls()].concat(),
            Stmt::IfElse(cond, if_body, else_body) => [
                cond.func_call(),
                if_body.function_calls(),
                else_body.function_calls(),
            ]
            .concat(),
            Stmt::Block(statements) => statements
                .iter()
                .map(Stmt::function_calls)
                .flatten()
                .collect(),
            Stmt::ReturningBlock(statements) => statements
                .iter()
                .map(Stmt::function_calls)
                .flatten()
                .collect(),
            Stmt::Emit(expr) => {
                if let Expression::Func(_, args) = expr {
                    args.iter().map(Expression::func_call).flatten().collect()
                } else {
                    expr.func_call()
                }
            }
            Stmt::RevertWithError(_) => vec![],
            Stmt::Revert(Some(expr)) => expr.func_call(),
            Stmt::Revert(None) => vec![],
            Stmt::Placeholder => vec![],
            Stmt::While(cond, body) => [cond.func_call(), body.function_calls()].concat(),
            Stmt::DocComment(_) => vec![],
            Stmt::Unknown => vec![],
            #[cfg(test)]
            Stmt::Fail => vec![],
            Stmt::ReturnVoid => vec![],
            Stmt::Expression(expr) => expr.func_call(),
        }
    }
}

impl From<&pt::Statement> for Stmt {
    fn from(value: &pt::Statement) -> Self {
        match value {
            pt::Statement::Block {
                loc,
                unchecked,
                statements,
            } => Self::Block(statements.iter().map(From::from).collect()),
            pt::Statement::Assembly {
                loc,
                dialect,
                statements,
            } => todo!(),
            pt::Statement::Args(_, _) => todo!(),
            pt::Statement::If(_, assertion, if_body, else_body) => {
                let if_body = if matches!(**if_body, Statement::Block { .. }) {
                    Box::new(if_body.as_ref().into())
                } else {
                    Box::new(Stmt::Block(vec![if_body.as_ref().into()]))
                };
                let else_body = else_body.as_ref().map(|stmt| {
                    if matches!(**stmt, Statement::Block { .. }) {
                        Box::new(stmt.as_ref().into())
                    } else if matches!(**stmt, Statement::If { .. }) {
                        Box::new(stmt.as_ref().into())
                    } else {
                        Box::new(Stmt::Block(vec![stmt.as_ref().into()]))
                    }
                });

                match else_body {
                    Some(else_body) => Self::IfElse(assertion.into(), if_body, else_body),
                    None => Self::If(assertion.into(), if_body),
                }
            }
            pt::Statement::While(_, assertion, block) => {
                Self::While(assertion.into(), Box::new(block.as_ref().into()))
            }
            pt::Statement::Expression(_, expr) => {
                let expr: Expression = expr.into();
                if expr == Expression::Placeholder {
                    Self::Placeholder
                } else {
                    Self::Expression(expr)
                }
            }
            pt::Statement::VariableDefinition(_, declaration, init) => {
                let name = declaration.name.name.clone();
                let ty = Expression::from(&declaration.ty);
                let ty = Type::try_from(&ty).unwrap_or(Type::Unknown);
                match init {
                    Some(expr) => Self::VarDefinition(name, ty, expr.into()),
                    None => Self::VarDeclaration(name, ty),
                }
            }
            pt::Statement::For(_, _, _, _, _) => Self::Unknown,
            pt::Statement::DoWhile(_, _, _) => Self::Unknown,
            pt::Statement::Continue(_) => Self::Unknown,
            pt::Statement::Break(_) => Self::Unknown,
            pt::Statement::Return(_, r) => match r {
                Some(expr) => Self::Return(expr.into()),
                None => Self::ReturnVoid,
            },
            pt::Statement::Revert(_, error_id, err) => {
                if let Some(id) = error_id {
                    Self::RevertWithError(id.name.to_owned())
                } else {
                    if err.is_empty() {
                        Self::Revert(None)
                    } else {
                        Self::Revert(err.first().map(|e| e.into()))
                    }
                }
            }
            pt::Statement::Emit(_, expr) => Self::Emit(expr.into()),
            pt::Statement::Try(_, _, _, _) => Self::Unknown,
            pt::Statement::DocComment(_, _, comment) => Self::DocComment(comment.to_owned()),
        }
    }
}
