use std::{fs::File, io::Read};

use nysa::OdraParser;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};

/// Transforms solidity code into rust code.
///
/// # Example
///
/// ```
/// nysa_macro::nysa_lang! {
///     contract StatusMessage {
///         mapping(address => string) records;
///
///         function set_status(string memory status) public payable {
///             address account_id = msg.sender;
///             records[account_id] = status;
///         }
///
///         function get_status(address account_id) public view returns (string memory) {
///             return records[account_id];
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn nysa_lang(item: TokenStream) -> TokenStream {
    let solidity_code = item.to_string();
    to_odra(solidity_code)
}

/// Reads solidity code from a given path and transforms it into rust code.
///
/// # Example
///
/// ```
/// nysa_macro::nysa_file!("example-status/src/contract.sol");
/// ```
#[proc_macro]
pub fn nysa_file(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as FileName);
    let solidity_code = read_solidity_code(input);
    to_odra(solidity_code)
}

fn to_odra(solidity_code: String) -> TokenStream {
    let c3_ast = nysa::parse::<OdraParser, _>(solidity_code);
    c3_ast.to_token_stream().into()
}

fn read_solidity_code(input: FileName) -> String {
    let cwd = project_root::get_project_root().unwrap();
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
        Ok(Self {
            filename: lit_file.value(),
        })
    }
}
