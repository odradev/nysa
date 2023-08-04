lint:
    cargo clippy --all-targets -- -D warnings
    cargo fmt

test-status-contract-solidity:
    cd example-status && cargo gen-casper-solidity
    cd example-status && cargo wasm-solidity
    wasm-strip example-status/target/wasm32-unknown-unknown/release/status_message.wasm
    cd example-status && cargo test-solidity

test-status-contract-odra:
    cd example-status && cargo gen-casper-odra
    cd example-status && cargo wasm-odra
    wasm-strip example-status/target/wasm32-unknown-unknown/release/status_message.wasm
    cd example-status && cargo test-odra

test-token-contract-solidity:
    cd example-owned-token && cargo gen-casper-solidity
    cd example-owned-token && cargo wasm-solidity
    wasm-strip example-owned-token/target/wasm32-unknown-unknown/release/owned_token.wasm
    cd example-owned-token && cargo test-odra

test-token-contract-odra:
    cd example-owned-token && cargo gen-casper-odra
    cd example-owned-token && cargo wasm-odra
    wasm-strip example-owned-token/target/wasm32-unknown-unknown/release/owned_token.wasm
    cd example-owned-token && cargo test-odra

test-examples:
    just test-status-contract-solidity
    just test-status-contract-odra
    just test-token-contract-solidity
    just test-token-contract-odra
