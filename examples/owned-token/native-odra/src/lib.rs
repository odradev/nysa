mod owned_token;
pub use owned_token::{OwnedToken, OwnedTokenDeployer, OwnedTokenRef};

#[cfg(test)]
mod tests {
    use odra::test_env;

    use super::*;

    #[test]
    fn test() {
        let name = String::from("Plascoin");
        let symbol = String::from("PLS");
        let decimals = 18;
        let initial_supply = 1_000_000_000_000_000u64.into();
        let mut contract = OwnedTokenDeployer::init(name, symbol, decimals, initial_supply);

        contract.transfer_ownership(Some(test_env::get_account(1)));

        assert_eq!(contract.get_owner(), Some(test_env::get_account(1)));
    }
}
