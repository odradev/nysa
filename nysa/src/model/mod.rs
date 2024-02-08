//! However the result of parsing solidity code gives a well-structured representation to work with,
//! for the sake of code generation, an intermediary step is beneficial. At this step we can remove any
//! ambiguities or missing context.
//!
//! The main responsibility of the module is conversion the model from solidity_parser to a more friendly,
//! preprocessed representation.
mod contract;
mod expr;
mod func;
mod interface;
mod misc;
mod op;
mod package;
mod stmt;

pub use contract::ContractData;
use itertools::Itertools;

use crate::utils::AsStringVec;

use self::misc::{ContractMetadata, Enum, Error, Event, Struct};

pub(super) const RESERVED_NAMES: [&str; 1] = ["self"];

pub mod ir {
    pub use super::expr::{eval_expression_type, Expression, TupleItem};
    pub use super::func::*;
    pub use super::interface::InterfaceData;
    pub use super::misc::*;
    pub use super::op::{BitwiseOp, LogicalOp, MathOp, Op, UnaryOp};
    pub use super::package::Package;
    pub use super::stmt::Stmt;
}

/// A type that has a name.
pub trait Named {
    /// Returns the type name.
    fn name(&self) -> String;
}

impl<T: Named> AsStringVec for &[T] {
    fn as_string_vec(&self) -> Vec<String> {
        self.iter().map(|i| i.name()).collect_vec()
    }
}

impl<T: Named> AsStringVec for Vec<&T> {
    fn as_string_vec(&self) -> Vec<String> {
        self.iter().map(|i| i.name()).collect_vec()
    }
}

macro_rules! impl_named {
    ($($t:ty),+) => {
        $(
            impl Named for $t {
                fn name(&self) -> String {
                    self.name.clone()
                }
            }
        )+
    };
}

impl_named!(Enum, Error, Event, ContractMetadata, Struct);
