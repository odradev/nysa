use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro]
pub fn nysa_lang(_item: TokenStream) -> TokenStream {
    let c3_ast = nysa::expected();
    c3_ast.to_token_stream().into()
}
