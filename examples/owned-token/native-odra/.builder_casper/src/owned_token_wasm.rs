#![no_main]
const KEYS: [&'static str; 6usize] = [
    "owner",
    "name",
    "symbol",
    "decimals",
    "total_supply",
    "balance_of",
];
#[no_mangle]
fn call() {
    let schemas = vec![];
    let mut entry_points = odra::casper::casper_types::EntryPoints::new();
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "init",
        vec![
            odra::casper::casper_types::Parameter::new(
                "name",
                odra::casper::casper_types::CLType::String,
            ),
            odra::casper::casper_types::Parameter::new(
                "symbol",
                odra::casper::casper_types::CLType::String,
            ),
            odra::casper::casper_types::Parameter::new(
                "decimals",
                odra::casper::casper_types::CLType::U8,
            ),
            odra::casper::casper_types::Parameter::new(
                "initial_supply",
                odra::casper::casper_types::CLType::U256,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Groups(vec![
            odra::casper::casper_types::Group::new("constructor_group"),
        ]),
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "transfer_ownership",
        vec![odra::casper::casper_types::Parameter::new(
            "new_owner",
            odra::casper::casper_types::CLType::Option(Box::new(
                odra::casper::casper_types::CLType::Key,
            )),
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "only_owner",
        vec![],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_owner",
        vec![],
        odra::casper::casper_types::CLType::Option(Box::new(
            odra::casper::casper_types::CLType::Key,
        )),
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "burn",
        vec![odra::casper::casper_types::Parameter::new(
            "amount",
            odra::casper::casper_types::CLType::U256,
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "mint",
        vec![
            odra::casper::casper_types::Parameter::new(
                "to",
                odra::casper::casper_types::CLType::Option(Box::new(
                    odra::casper::casper_types::CLType::Key,
                )),
            ),
            odra::casper::casper_types::Parameter::new(
                "amount",
                odra::casper::casper_types::CLType::U256,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "transfer",
        vec![
            odra::casper::casper_types::Parameter::new(
                "to",
                odra::casper::casper_types::CLType::Option(Box::new(
                    odra::casper::casper_types::CLType::Key,
                )),
            ),
            odra::casper::casper_types::Parameter::new(
                "value",
                odra::casper::casper_types::CLType::U256,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "_transfer",
        vec![
            odra::casper::casper_types::Parameter::new(
                "from",
                odra::casper::casper_types::CLType::Option(Box::new(
                    odra::casper::casper_types::CLType::Key,
                )),
            ),
            odra::casper::casper_types::Parameter::new(
                "to",
                odra::casper::casper_types::CLType::Option(Box::new(
                    odra::casper::casper_types::CLType::Key,
                )),
            ),
            odra::casper::casper_types::Parameter::new(
                "value",
                odra::casper::casper_types::CLType::U256,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    #[allow(unused_variables)]
    let contract_package_hash = odra::casper::utils::install_contract(entry_points, schemas);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let constructor_access = odra::casper::utils::create_constructor_group(contract_package_hash);
    let constructor_name = odra::casper::utils::load_constructor_name_arg();
    match constructor_name.as_str() {
        "init" => {
            let odra_address = odra::types::Address::try_from(contract_package_hash)
                .map_err(|err| {
                    let code = odra::types::ExecutionError::from(err).code();
                    odra::casper::casper_types::ApiError::User(code)
                })
                .unwrap_or_revert();
            let mut contract_ref = odra_owned_token::OwnedTokenRef::at(&odra_address);
            let name = odra::casper::casper_contract::contract_api::runtime::get_named_arg("name");
            let symbol =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg("symbol");
            let decimals =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg("decimals");
            let initial_supply =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg(
                    "initial_supply",
                );
            contract_ref.init(name, symbol, decimals, initial_supply);
        }
        _ => odra::casper::utils::revert_on_unknown_constructor(),
    };
    odra::casper::utils::revoke_access_to_constructor_group(
        contract_package_hash,
        constructor_access,
    );
}
#[no_mangle]
fn init() {
    let (mut _contract, _): (odra_owned_token::OwnedToken, _) =
        odra::StaticInstance::instance(&KEYS);
    let name = odra::casper::casper_contract::contract_api::runtime::get_named_arg("name");
    let symbol = odra::casper::casper_contract::contract_api::runtime::get_named_arg("symbol");
    let decimals = odra::casper::casper_contract::contract_api::runtime::get_named_arg("decimals");
    let initial_supply =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("initial_supply");
    _contract.init(name, symbol, decimals, initial_supply);
}
#[no_mangle]
fn transfer_ownership() {
    let (mut _contract, _): (odra_owned_token::OwnedToken, _) =
        odra::StaticInstance::instance(&KEYS);
    let new_owner =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("new_owner");
    _contract.transfer_ownership(new_owner);
}
#[no_mangle]
fn only_owner() {
    let (_contract, _): (odra_owned_token::OwnedToken, _) = odra::StaticInstance::instance(&KEYS);
    _contract.only_owner();
}
#[no_mangle]
fn get_owner() {
    let (_contract, _): (odra_owned_token::OwnedToken, _) = odra::StaticInstance::instance(&KEYS);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.get_owner();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn burn() {
    let (mut _contract, _): (odra_owned_token::OwnedToken, _) =
        odra::StaticInstance::instance(&KEYS);
    let amount = odra::casper::casper_contract::contract_api::runtime::get_named_arg("amount");
    _contract.burn(amount);
}
#[no_mangle]
fn mint() {
    let (mut _contract, _): (odra_owned_token::OwnedToken, _) =
        odra::StaticInstance::instance(&KEYS);
    let to = odra::casper::casper_contract::contract_api::runtime::get_named_arg("to");
    let amount = odra::casper::casper_contract::contract_api::runtime::get_named_arg("amount");
    _contract.mint(to, amount);
}
#[no_mangle]
fn transfer() {
    let (mut _contract, _): (odra_owned_token::OwnedToken, _) =
        odra::StaticInstance::instance(&KEYS);
    let to = odra::casper::casper_contract::contract_api::runtime::get_named_arg("to");
    let value = odra::casper::casper_contract::contract_api::runtime::get_named_arg("value");
    _contract.transfer(to, value);
}
#[no_mangle]
fn _transfer() {
    let (mut _contract, _): (odra_owned_token::OwnedToken, _) =
        odra::StaticInstance::instance(&KEYS);
    let from = odra::casper::casper_contract::contract_api::runtime::get_named_arg("from");
    let to = odra::casper::casper_contract::contract_api::runtime::get_named_arg("to");
    let value = odra::casper::casper_contract::contract_api::runtime::get_named_arg("value");
    _contract._transfer(from, to, value);
}
