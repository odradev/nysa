#![no_std]

extern crate alloc;

mod status_message;
pub use status_message::status_message::{StatusMessage, StatusMessageHostRef};

#[cfg(test)]
mod test;
