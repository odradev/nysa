mod status_message;
pub use status_message::{StatusMessage, StatusMessageDeployer, StatusMessageRef};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_message() {
        let mut contract = StatusMessageDeployer::default();

        let address = odra::test_env::get_account(0);

        contract.set_status("hello".to_string());
        assert_eq!("hello".to_string(), contract.get_status(Some(address)));
    }

    #[test]
    fn get_nonexistent_message() {
        let contract = StatusMessageDeployer::default();

        assert_eq!(
            String::new(),
            contract.get_status(Some(odra::test_env::get_account(0)))
        );
    }
}
