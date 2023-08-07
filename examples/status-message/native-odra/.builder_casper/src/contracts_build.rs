mod status_message {
    odra::casper::codegen::gen_contract!(odra_example_status::StatusMessage, "status_message");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    args.iter().skip(1).for_each(|arg| match arg.as_str() {
        "status_message" => status_message::main(),
        _ => println!("Please provide a valid module name!"),
    });
}
