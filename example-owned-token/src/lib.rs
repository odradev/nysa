#[cfg(feature = "solidity")]
mod owned_token_sol;

#[cfg(feature = "solidity")]
pub use owned_token_sol::{OwnedToken, OwnedTokenDeployer, OwnedTokenRef};

#[cfg(feature = "native-odra")]
mod owned_token;

#[cfg(feature = "native-odra")]
pub use owned_token::{OwnedToken, OwnedTokenDeployer, OwnedTokenRef};

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

}
