use odra::{host::Deployer, prelude::*};
use crate::owned_token::OwnedTokenInitArgs;

use super::*;

#[test]
fn test() {
    let env: odra::host::HostEnv = odra_test::env();
    let init_args = OwnedTokenInitArgs {
        name: String::from("Plascoin"),
        symbol: String::from("PLS"),
        decimals: 18,
        initial_supply: 1_000_000_000_000_000u64.into()
    };
    let mut contract = OwnedTokenHostRef::deploy(&env, init_args);

    contract.transfer_ownership(Some(env.get_account(1)));

    assert_eq!(contract.get_owner(), Some(env.get_account(1)));
}
