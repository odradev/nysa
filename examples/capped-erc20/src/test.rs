use nysa_types::U256;
use odra::host::{Deployer, HostEnv};

use self::plascoin::plascoin::PlascoinInitArgs;

use super::*;
use crate::plascoin::errors::Error;
use odra::prelude::*;

fn setup() -> (PlascoinHostRef, HostEnv) {
    let env = odra_test::env();
    let init_args = PlascoinInitArgs {
        name: String::from("Plascoin"),
        symbol: String::from("PLS"),
        cap: U256::from(1_000_000_000_000_000u64),
        initial_owner: Some(env.get_account(1)),
    };

    (PlascoinHostRef::deploy(&env, init_args), env)
}

#[test]
fn test_setup() {
    let (contract, _) = setup();

    assert_eq!(contract.cap(), U256::from(1_000_000_000_000_000u64));
    assert_eq!(contract.name(), String::from("Plascoin"));
    assert_eq!(contract.symbol(), String::from("PLS"));
}

#[test]
fn test_ownership() {
    let (mut contract, env) = setup();

    let (owner, non_owner, new_owner) =
        (env.get_account(1), env.get_account(2), env.get_account(3));

    env.set_caller(non_owner);
    assert_eq!(
        contract.try_renounce_ownership(),
        Err(Error::OwnableUnauthorizedAccount.into())
    );

    env.set_caller(owner);
    contract.transfer_ownership(Some(new_owner));
    assert_eq!(contract.owner(), Some(new_owner));

    env.set_caller(new_owner);
    contract.renounce_ownership();

    assert_eq!(contract.owner(), None);
}

#[test]
fn test_mint() {
    let (mut contract, env) = setup();

    let owner = env.get_account(1);
    let recipient = Some(env.get_account(2));

    env.set_caller(owner);
    contract.mint(recipient, U256::from(1_000));

    assert_eq!(contract.balance_of(recipient), U256::from(1_000));
}
