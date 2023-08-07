mod owned_token {
    odra::casper::codegen::gen_contract!(odra_owned_token::OwnedToken, "owned_token");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    args.iter().skip(1).for_each(|arg| match arg.as_str() {
        "owned_token" => owned_token::main(),
        _ => println!("Please provide a valid module name!"),
    });
}
