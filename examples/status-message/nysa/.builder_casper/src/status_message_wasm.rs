#![no_main]
const KEYS: [&'static str; 1usize] = ["records"];
#[no_mangle]
fn call() {
    let schemas = vec![];
    let mut entry_points = odra::casper::casper_types::EntryPoints::new();
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_status",
        vec![odra::casper::casper_types::Parameter::new(
            "account_id",
            odra::casper::casper_types::CLType::Option(Box::new(
                odra::casper::casper_types::CLType::Key,
            )),
        )],
        odra::casper::casper_types::CLType::String,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "set_status",
        vec![odra::casper::casper_types::Parameter::new(
            "status",
            odra::casper::casper_types::CLType::String,
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    #[allow(unused_variables)]
    let contract_package_hash = odra::casper::utils::install_contract(entry_points, schemas);
}
#[no_mangle]
fn get_status() {
    let (_contract, _): (nysa_example_status::StatusMessage, _) =
        odra::StaticInstance::instance(&KEYS);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let account_id =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("account_id");
    let result = _contract.get_status(account_id);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn set_status() {
    odra::casper::utils::handle_attached_value();
    let (mut _contract, _): (nysa_example_status::StatusMessage, _) =
        odra::StaticInstance::instance(&KEYS);
    let status = odra::casper::casper_contract::contract_api::runtime::get_named_arg("status");
    _contract.set_status(status);
    odra::casper::utils::clear_attached_value();
}
