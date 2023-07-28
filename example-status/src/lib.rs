#[cfg(feature = "solidity")]
mod status_message_sol;

#[cfg(feature = "solidity")]
pub use status_message_sol::{StatusMessage, StatusMessageDeployer, StatusMessageRef};

#[cfg(feature = "native-odra")]
mod status_message;

#[cfg(feature = "native-odra")]
pub use status_message::{StatusMessage, StatusMessageDeployer, StatusMessageRef};

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_message() {
        let mut contract = StatusMessageDeployer::default();

        let address = odra::test_env::get_account(0);

        contract.set_status("hello".to_string());
        assert_eq!("hello".to_string(), contract.get_status(address));
    }

    #[test]
    fn get_nonexistent_message() {
        let contract = StatusMessageDeployer::default();

        assert_eq!(
            String::new(),
            contract.get_status(odra::test_env::get_account(0))
        );
    }
}
