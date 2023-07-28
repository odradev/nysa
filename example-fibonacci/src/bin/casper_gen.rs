mod owned_token {
    odra::casper::codegen::gen_contract!(example_owned_token::OwnedToken, "owned_token");
}

fn main() {
    owned_token::main();
}
