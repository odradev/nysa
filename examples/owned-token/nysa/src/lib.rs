#![no_std]

extern crate alloc;

mod owned_token;
pub use owned_token::owned_token::{OwnedToken, OwnedTokenDeployer, OwnedTokenRef};

#[cfg(test)]
mod test;
