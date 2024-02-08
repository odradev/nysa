use super::*;
use odra::{
    host::{Deployer, HostEnv, NoArgs},
    prelude::*,
    Address,
};

const ACCOUNT: fn(&HostEnv) -> Option<Address> = |env: &HostEnv| Some(env.get_account(1));

#[test]
fn set_get_message() {
    let env = odra_test::env();
    let mut contract = StatusMessageHostRef::deploy(&env, NoArgs);

    env.set_caller(ACCOUNT(&env).unwrap());
    contract.set_status("hello".to_string());
    assert_eq!("hello".to_string(), contract.get_status(ACCOUNT(&env)));
}

#[test]
fn get_nonexistent_message() {
    let env = odra_test::env();
    let contract = StatusMessageHostRef::deploy(&env, NoArgs);

    assert_eq!(String::new(), contract.get_status(ACCOUNT(&env)));
}
