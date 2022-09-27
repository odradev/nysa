lint:
    cargo clippy --all-targets -- -D warnings
    cargo fmt

build-status-contract:
    cargo build -p example-status --release --target wasm32-unknown-unknown
    wasm-strip target/wasm32-unknown-unknown/release/example_status.wasm

build-fibonacci-contract:
    cargo build -p example-fibonacci --release --target wasm32-unknown-unknown
    wasm-strip target/wasm32-unknown-unknown/release/example_fibonacci.wasm

test-status-solidity:
    cargo test -p example-status

test-status-near:
    cargo test -p example-status --no-default-features --features "near"

test-fibonacci-solidity:
    cargo test -p example-fibonacci

test-fibonacci-near:
    cargo test -p example-fibonacci --no-default-features --features "near"

test-nysa:
    cargo test -p nysa

test-examples:
    just test-status-solidity
    just test-status-near
    just test-fibonacci-solidity
    just test-fibonacci-near
