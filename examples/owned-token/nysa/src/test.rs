use nysa_types::{U256, U8};
use odra::{prelude::string::String, test_env};

use super::*;

#[test]
fn test() {
    let name = String::from("Plascoin");
    let symbol = String::from("PLS");
    let decimals = U8::from(8);
    let initial_supply = U256::from(1_000);
    let mut contract = OwnedTokenDeployer::init(name, symbol, decimals, initial_supply);

    contract.transfer_ownership(Some(test_env::get_account(1)));

    assert_eq!(contract.get_owner(), Some(test_env::get_account(1)));
}
