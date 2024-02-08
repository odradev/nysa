#![no_std]

extern crate alloc;

mod plascoin;
pub use plascoin::plascoin::{Plascoin, PlascoinHostRef};

#[cfg(test)]
mod test;
