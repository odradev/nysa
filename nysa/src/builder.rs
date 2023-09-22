use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    process::Command,
};

use quote::ToTokens;

use crate::{parse, parser::Parser};

/// Generates a rust code file at `dest_code_path` from a solidity source code located at `source_code_path`.
///
/// Panics if:
///  * `source_code_path` does not exist
///  * could not read the input file,
///  * parsing solidity code files
///  * could not create/write to the destination file.
pub fn generate_file<P, T>(source_code_path: P, dest_code_path: P)
where
    P: AsRef<Path>,
    T: Parser,
{
    if file_exists(dest_code_path.as_ref()) {
        return;
    }

    let mut file = File::open(source_code_path).expect("Invalid path to the solidity code.");
    let mut solidity_code = String::new();
    file.read_to_string(&mut solidity_code)
        .expect("Could not read the solidity code.");
    let c3_ast = parse::<T, _>(solidity_code);
    let code = c3_ast.to_token_stream().to_string();

    let mut file = File::create(dest_code_path).expect("Failed to create the output file");
    writeln!(file, "{}", code).expect("Failed to write to file");

    run_cargo_fmt();
}

fn file_exists(file_path: &Path) -> bool {
    file_path.exists()
}

fn run_cargo_fmt() {
    let status = Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("Failed to execute cargo fmt");

    if !status.success() {
        panic!("cargo fmt failed");
    }
}
