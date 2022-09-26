# NYSA
**N**EAR **Y**ielding **S**olidity **A**lternative

A Solidity to Near-Sdk Rust Transpiler.

The project aims to transpile smart contracts written in Solidity to smart contract compatible with near-sdk.

## Prerequisites

- Rust toolchain installed (see [rustup.rs](https://rustup.rs/))
- WebAssembly Binary Toolkit (wabt) installed (see [wabt](https://github.com/WebAssembly/wabt))
- just (not required but recommended) (see [just](https://github.com/casey/just))
  
## Build examples

The easiest way to build and test examples is to run commands defined in the justfile.

Eg. to run Fibonacci Sequence example execute:

`$ just test-fibonacci`

To build Status Message example, execute:

`$ just build-status-contract`
