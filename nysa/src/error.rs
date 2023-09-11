use thiserror::Error;

use crate::model::ir::{Expression, Function, NumSize, Type};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unsupported type `{0:?}`.")]
    UnsupportedType(Type),
    #[error("Unsupported state type `{0:?}`.")]
    UnsupportedStateType(Type),
    #[error("Unsupported num size `{0:?}`.")]
    UnsupportedUnit(NumSize),
    #[error("Unexpected expression, expected `{0}`, but found `{1:?}`.")]
    UnexpectedExpression(String, Expression),
    #[error("Unsupported message type {0}.")]
    UnsupportedMessageType(String),
    #[error("Empty expression")]
    EmptyExpression,
    #[error("Constructor not found")]
    ConstructorNotFound,
    #[error("Invalid function type, expected `{0}`, but found `{1:?}`.")]
    InvalidFunctionType(String, Function),
    #[error("Modifier {0} must have exactly one implementation")]
    InvalidModifier(String),
    #[error("Unknown type property {0}")]
    UnknownProperty(String),
    #[error("Not a state variable")]
    NotStateVariable,
    #[error("Invalid collection")]
    InvalidCollection,
    #[error("Invalid type")]
    InvalidType,
    #[error("Mapping cannot be initialized")]
    MappingInit,
    #[error("Invalid expression")]
    InvalidExpression,
}
