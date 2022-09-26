nysa_macro::nysa_file!("example-fibonacci/src/contract.sol");

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
    fn test_fibonacci() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Fibonacci::default();
        
        let mut test = |n: u32, expected: u32| {
            contract.compute(n);
            let result = contract.get_result(n);
            assert_eq!(result, expected);
        };

        test(1, 1);
        test(2, 1);
        test(3, 2);
        test(4, 3);
        test(5, 5);
        test(6, 8);
        test(7, 13);
        test(8, 21);
        test(9, 34);
        test(10, 55);
        test(31, 1346269);
    }
}
