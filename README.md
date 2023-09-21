# NYSA

NYSA - Solidity to Rust Transpiler.

The project aims to transpile smart contracts written in Solidity to Rust smart contracts.

Nysa performs Solidity-to-Rust translation in four steps:

![nysa-gen](./assets/nysa_generic.drawio.svg)


By design, Nysa is a universal tool, so the `Parser` component from the diagram is exchangeable. In other words, a Solidity input can be converted into Rust code supporting a framework/SDK of your choice, unless a parser implementation is provided.

At this moment, the only implementation is `OdraParser` which takes a contract written in Solidity and prints out [Odra](https://odra.dev/docs/)-compatible code.

## Prerequisites

- Rust toolchain installed (see [rustup.rs](https://rustup.rs/)).
- WebAssembly Binary Toolkit (wabt) installed (see [wabt](https://github.com/WebAssembly/wabt)).
- just (not required but recommended) (see [just](https://github.com/casey/just)).
- cargo-odra (not required but recommended) (see [cargo-odra](https://github.com/odradev/cargo-odra)).

## Test Nysa

To test `Nysa` internal tests, execute:

```bash
$ just test
```

The core of tests is the `resources` directory. It is a test collection of Solidity code samples with the expected Rust output. 

## Build and test examples

The easiest way to build and test examples is to run commands defined in the `justfile`.

To build wasm files and run tests, execute:

```bash
$ just test-status-contract-solidity
$ just test-token-contract-solidity
```

To run all the example tests, execute:

```bash
$ just test-examples
```
## Status

Legend

Done:           :white_check_mark:

Partially Done: :hammer:

Not Done:       :x:

Soon:           :soon:

| Solidity Statements | Status             |
|---------------------|--------------------|
| Block               | :white_check_mark: |
| If                  | :white_check_mark: |
| While               | :white_check_mark: |
| For                 | :soon:             |
| Do while            | :soon:             |
| Continue            | :soon:             |
| Break               | :soon:             |
| Return              | :white_check_mark: |
| Revert              | :white_check_mark: |
| Emit                | :white_check_mark: |
| Doc comment         | :x:                |
| Variable definition | :white_check_mark: |
| Assembly            | :x:                |
| Args                | :x:                |
| Try                 | :soon:             |


| Solidity Expressions | Status             | Solidity Expressions  | Status             |
|----------------------|--------------------|-----------------------|--------------------|
| PostIncrement        | :white_check_mark: | LessEqual             | :white_check_mark: |
| PostDecrement        | :white_check_mark: | MoreEqual             | :white_check_mark: |
| New                  | :x:                | Equal                 | :white_check_mark: |
| ArraySubscript       | :hammer:           | NotEqual              | :white_check_mark: |
| ArraySlice           | :x:                | And                   | :white_check_mark: |
| MemberAccess         | :hammer:           | Or                    | :white_check_mark: |
| FunctionCall         | :hammer:           | Ternary               | :white_check_mark: |
| FunctionCallBlock    | :x:                | Assign                | :white_check_mark: |
| NamedFunctionCall    | :x:                | AssignOr              | :hammer:           |
| Not                  | :white_check_mark: | AssignAnd             | :hammer:           |
| Complement           | :white_check_mark: | AssignXor             | :hammer:           |
| Delete               | :white_check_mark: | AssignShiftLeft       | :hammer:           |
| PreIncrement         | :white_check_mark: | AssignShiftRight      | :hammer:           |
| PreDecrement         | :white_check_mark: | AssignAdd             | :white_check_mark: |
| UnaryPlus            | :x:                | AssignSubtract        | :white_check_mark: |
| UnaryMinus           | :x:                | AssignMultiply        | :white_check_mark: |
| Power                | :white_check_mark: | AssignDivide          | :white_check_mark: |
| Multiply             | :white_check_mark: | AssignModulo          | :white_check_mark: |
| Divide               | :white_check_mark: | BoolLiteral           | :white_check_mark: |
| Modulo               | :white_check_mark: | NumberLiteral         | :hammer:           |
| Add                  | :white_check_mark: | RationalNumberLiteral | :x:                |
| Subtract             | :white_check_mark: | HexNumberLiteral      | :white_check_mark: |
| ShiftLeft            | :hammer:           | StringLiteral         | :white_check_mark: |
| ShiftRight           | :hammer:           | Type                  | :white_check_mark: |
| BitwiseAnd           | :hammer:           | HexLiteral            | :white_check_mark: |
| BitwiseXor           | :hammer:           | BoolLiteral           | :white_check_mark: |
| BitwiseOr            | :hammer:           | AddressLiteral        | :x:                |
| Less                 | :white_check_mark: | Variable              | :white_check_mark: |
| More                 | :white_check_mark: | List                  | :x:                |
| ArrayLiteral         | :x:                | Unit                  | :x:                |
| This                 | :x:                |                       |                    |

