lint:
    cargo clippy --all-targets
    cargo fmt

build-status-contract:
    cargo build -p example-status --release --target wasm32-unknown-unknown
    wasm-strip target/wasm32-unknown-unknown/release/example_status.wasm

build-fibonacci-contract:
    cargo build -p example-fibonacci --release --target wasm32-unknown-unknown
    wasm-strip target/wasm32-unknown-unknown/release/example_fibonacci.wasm

test-fibonacci:
    cargo test -p example-fibonacci