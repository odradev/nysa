use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    process::Command,
};

use quote::ToTokens;

use crate::{parse, parser::Parser};

pub fn generate_file<P, T>(source_code_path: P, dest_code_path: P)
where
    P: AsRef<Path>,
    T: Parser,
{
    if file_exists(dest_code_path.as_ref()) {
        return;
    }

    let mut file = File::open(source_code_path).unwrap();
    let mut solidity_code = String::new();
    file.read_to_string(&mut solidity_code).unwrap();
    let c3_ast = parse::<T>(solidity_code);
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
