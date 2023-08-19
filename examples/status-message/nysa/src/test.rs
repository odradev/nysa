use super::*;
use odra::{test_env, types::Address};

const ACCOUNT: fn() -> Address = || odra::test_env::get_account(1);

#[test]
fn set_get_message() {
    let mut contract = StatusMessageDeployer::default();

    test_env::set_caller(ACCOUNT());
    contract.set_status("hello".to_string());
    assert_eq!("hello".to_string(), contract.get_status(Some(ACCOUNT())));
}

#[test]
fn get_nonexistent_message() {
    let contract = StatusMessageDeployer::default();

    assert_eq!(String::new(), contract.get_status(Some(ACCOUNT())));
}
