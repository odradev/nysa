use nysa_types::U256;
use odra::test_env;

use super::*;
use crate::plascoin::errors::Error;
use odra::prelude::string::String;

fn setup() -> PlascoinRef {
    let name = String::from("Plascoin");
    let symbol = String::from("PLS");
    let cap = U256::from(1_000_000_000_000_000u64);
    let initial_owner = Some(test_env::get_account(1));

    PlascoinDeployer::init(name, symbol, cap, initial_owner)
}

#[test]
fn test_setup() {
    let contract = setup();

    assert_eq!(contract.cap(), U256::from(1_000_000_000_000_000u64));
    assert_eq!(contract.name(), String::from("Plascoin"));
    assert_eq!(contract.symbol(), String::from("PLS"));
}

#[test]
fn test_ownership() {
    let mut contract = setup();

    let (owner, non_owner, new_owner) = (
        test_env::get_account(1),
        test_env::get_account(2),
        test_env::get_account(3),
    );

    test_env::set_caller(non_owner);
    test_env::assert_exception(Error::OwnableUnauthorizedAccount, || {
        contract.renounce_ownership();
    });

    test_env::set_caller(owner);
    contract.transfer_ownership(Some(new_owner));
    assert_eq!(contract.owner(), Some(new_owner));

    test_env::set_caller(new_owner);
    contract.renounce_ownership();

    assert_eq!(contract.owner(), None);
}

#[test]
fn test_mint() {
    let mut contract = setup();

    let owner = test_env::get_account(1);
    let recipient = Some(test_env::get_account(2));

    test_env::set_caller(owner);
    contract.mint(recipient, U256::from(1_000));

    assert_eq!(contract.balance_of(recipient), U256::from(1_000));
}
