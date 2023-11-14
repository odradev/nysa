use thiserror::Error;

use crate::model::ir::{Expression, Function, Type};

/// Set of errors which may occur when parsing solidity code.
#[derive(Error, Debug, PartialOrd, PartialEq)]
pub enum ParserError {
    /// Type is not supported by the parser.
    #[error("Unsupported type `{0:?}`.")]
    UnsupportedType(Type),
    /// Expression is not supported by the parser.
    #[error("Unexpected expression, expected `{0}`, but found `{1:?}`.")]
    UnexpectedExpression(String, Expression),
    /// Contract constructor expected but found.
    #[error("Constructor not found")]
    ConstructorNotFound,
    /// Unexpected function type (function, modifier, constructor) in the current parser context.
    #[error("Invalid function type, expected `{0}`, but found `{1:?}`.")]
    InvalidFunctionType(String, Function),
    /// A modifier has more implementation in the inheritance graph.
    #[error("Modifier {0} must have exactly one implementation")]
    InvalidModifier(String),
    /// Unsupported type property.
    #[error("Unknown type property {0}")]
    UnknownProperty(String),
    /// A collection - mapping, array is constructed in a invalid way.
    #[error("Invalid collection")]
    InvalidCollection,
    /// Attempt to use a type in invalid context.
    #[error("Invalid type")]
    InvalidType,
    /// Attempt to assign a default value to mapping.
    #[error("Mapping cannot be initialized")]
    MappingInit,
    /// Attempt to use a statement in invalid context.
    #[error("Invalid statement: {0}")]
    InvalidStatement(&'static str),
    /// Attempt to use an expression in invalid context.
    #[error("Could not parse expression: {0}")]
    InvalidExpression(String),
}

#[macro_export]
macro_rules! formatted_invalid_expr {
    ($($arg:tt)*) => {
        Err(crate::ParserError::InvalidExpression(format!($($arg)*)))
    }
}
