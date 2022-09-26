use std::{fs::File, io::Read};

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};

#[proc_macro]
pub fn nysa_lang(item: TokenStream) -> TokenStream {
    let solidity_code = item.to_string();
    to_near(solidity_code)
}

#[proc_macro]
pub fn nysa_file(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as FileName);
    let solidity_code = read_solidity_code(input);
    to_near(solidity_code)
}

fn to_near(solidity_code: String) -> TokenStream {
    let c3_ast = nysa::parse(solidity_code);
    c3_ast.to_token_stream().into()
}

fn read_solidity_code(input: FileName) -> String {
    let cwd = std::env::current_dir().unwrap();

    let file_path = cwd.join(&input.filename);

    let mut file = File::open(file_path).unwrap();
    let mut solidity_code = String::new();
    file.read_to_string(&mut solidity_code).unwrap();
    solidity_code
}

#[derive(Debug)]
struct FileName {
    filename: String,
}

impl Parse for FileName {

    fn parse(input: ParseStream) -> Result<Self> {
        let lit_file: syn::LitStr = input.parse()?;
        Ok(Self { filename: lit_file.value() })
    }
}
