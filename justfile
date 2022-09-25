lint:
    cargo clippy --all-targets
    cargo fmt

build-status-contract:
    cargo build -p example-status --release --target wasm32-unknown-unknown
    wasm-strip target/wasm32-unknown-unknown/release/example_status.wasm

build-fibbonacci-contract:
    cargo build -p example-fibbonacci --release --target wasm32-unknown-unknown
    wasm-strip target/wasm32-unknown-unknown/release/example_fibbonacci.wasm

test-fibbonacci:
    cargo test -p example-fibbonacci