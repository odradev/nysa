mod status_message {
    odra::casper::codegen::gen_contract!(example_status::StatusMessage, "status_message");
}

fn main() {
    status_message::main();
}
