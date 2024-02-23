#![cfg(test)]

use super::{StatusMessage, StatusMessageClient};
use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};
use soroban_sdk::{Address, Env, IntoVal, String};

extern crate std;

#[test]
fn set_get_message() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StatusMessage {});
    let client = StatusMessageClient::new(&env, &contract_id);
    let account_id = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let message = String::from_str(&env, "hello");

    client
        .mock_auths(&[MockAuth {
            address: &account_id,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "set_status",
                args: (account_id.clone(), message.clone()).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_status(&account_id, &message);

    assert_eq!(client.get_status(&account_id, &account_id), message);
}

#[test]
fn get_nonexistent_message() {
    let env = Env::default();

    let contract_id = env.register_contract(None, StatusMessage {});
    let client = StatusMessageClient::new(&env, &contract_id);

    let account_id = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let message = String::from_str(&env, "");
    assert_eq!(client.get_status(&account_id, &account_id), message);
}
