#[cfg(feature = "solidity")]
nysa_macro::nysa_file!("example-status/src/contract.sol");

#[cfg(feature = "near")]
mod contract;

#[cfg(feature = "near")]
pub use contract::StatusMessage;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn set_get_message() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        let context = get_context(true);
        testing_env!(context);
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".parse().unwrap())
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(
            String::new(),
            contract.get_status("francis.near".parse().unwrap())
        );
    }
}
