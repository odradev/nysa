#![no_std]

mod status_message;

pub use status_message::{StatusMessage, StatusMessageHostRef};

#[cfg(test)]
mod test;
