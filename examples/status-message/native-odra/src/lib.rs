#![no_std]

mod status_message;

pub use status_message::{StatusMessage, StatusMessageDeployer, StatusMessageRef};

#[cfg(test)]
mod test;
