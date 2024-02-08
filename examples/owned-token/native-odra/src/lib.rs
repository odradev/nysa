#![no_std]

mod owned_token;
pub use owned_token::{OwnedToken, OwnedTokenHostRef};

#[cfg(test)]
mod test;
