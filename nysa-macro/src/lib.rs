use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro]
pub fn nysa_lang(item: TokenStream) -> TokenStream {
    let solidity_code = item.to_string();
    let c3_ast = nysa::parse(solidity_code);
    c3_ast.to_token_stream().into()
}
