use crate::parse;
use std::{fs::File, io::Read};

use super::*;

#[test]
fn test_constructor() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/constructors/1.sol"));
    assert_impl(result, "../resources/constructors/1.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/constructors/2.sol"));
    assert_impl(result, "../resources/constructors/2.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/constructors/3.sol"));
    assert_impl(result, "../resources/constructors/3.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/constructors/4.sol"));
    assert_impl(result, "../resources/constructors/4.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/constructors/5.sol"));
    assert_impl(result, "../resources/constructors/5.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/constructors/6.sol"));
    assert_impl(result, "../resources/constructors/6.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/constructors/7.sol"));
    assert_impl(result, "../resources/constructors/7.rs");
}

#[test]
fn test_modifier() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/modifiers/1.sol"));
    assert_impl(result, "../resources/modifiers/1.rs");
}

#[test]
fn test_default_value() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/default_values.sol"));
    assert_impl(result, "../resources/default_values.rs");
}

#[test]
fn test_ext() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/ext/1.sol"));
    assert_impl(result, "../resources/ext/1.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/ext/2.sol"));
    assert_impl(result, "../resources/ext/2.rs");
}

#[test]
fn test_ownable() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/ownable.sol"));
    assert_impl(result, "../resources/ownable.rs");
}

#[test]
#[ignore]
fn test_plascoin() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/plascoin.sol"));
    assert_impl(result, "../resources/plascoin.rs");
}

fn assert_impl(result: TokenStream, file_path: &str) {
    let parse = |str| {
        let file = syn::parse_file(str).unwrap();
        prettyplease::unparse(&file)
    };

    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    pretty_assertions::assert_eq!(parse(result.to_string().as_str()), parse(content.as_str()));
}
