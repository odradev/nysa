lint:
    cargo clippy --all-targets -- -D warnings
    cargo fmt

test-status-contract-solidity:
    rm -f examples/status-message/nysa/src/status_message.rs
    cd examples/status-message/nysa && cargo odra test -b casper

test-status-contract-odra:
    cd examples/status-message/native-odra && cargo odra test -b casper

test-token-contract-solidity:
    rm -f examples/owned-token/nysa/src/owned_token.rs
    cd examples/owned-token/nysa && cargo check
    cd examples/owned-token/nysa && cargo odra test -b casper

test-token-contract-odra:
    cd examples/owned-token/native-odra && cargo odra test -b casper

test-examples:
    just test-status-contract-solidity
    just test-status-contract-odra
    just test-token-contract-solidity
    just test-token-contract-odra

test:
    cargo test --package nysa

coverage:
    rm -rf target/coverage/
    mkdir -p target/coverage/
    CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/coverage/cargo-test-%p-%m.profraw' cargo test
    # Uncomment the following line to generate local HTML report
    grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore '../*' --ignore "/*" -o target/coverage/html
    grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov