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

use self::misc::{Contract, Enum, Error, Event};

pub mod ir {
    pub use super::expr::{to_expr, Expression};
    pub use super::func::*;
    pub use super::interface::InterfaceData;
    pub use super::misc::*;
    pub use super::op::{BitwiseOp, LogicalOp, MathOp, Op, UnaryOp};
    pub use super::package::Package;
    pub use super::stmt::Stmt;
}

pub trait Named {
    fn name(&self) -> String;
}

pub trait AsStringVec {
    fn as_string_vec(&self) -> Vec<String>;
}

impl<T: Named> AsStringVec for &[T] {
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

impl_named!(Enum, Error, Event, Contract);
