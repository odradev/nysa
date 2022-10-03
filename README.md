# NYSA
**N**EAR **Y**ielding **S**olidity **A**lternative

A Solidity to Near-Sdk Rust Transpiler.

The project aims to transpile smart contracts written in Solidity to smart contract compatible with near-sdk.

## Usage

<div align="center">
    <img src=".images/wasm_build.gif"></img>
</div>

## Prerequisites

- Rust toolchain installed (see [rustup.rs](https://rustup.rs/)).
- WebAssembly Binary Toolkit (wabt) installed (see [wabt](https://github.com/WebAssembly/wabt)).
- just (not required but recommended) (see [just](https://github.com/casey/just)).

## Test Nysa

To test `Nysa` internal tests, execute:

```bash
$ just test-nysa
```

## Build and test examples

The easiest way to build and test examples is to run commands defined in the `justfile`.

To build wasm files, execute:

```bash
$ just build-status-contract
$ just build-fibonacci-contract
```

To run all the example tests, execute:

```bash
$ just test-examples
```
