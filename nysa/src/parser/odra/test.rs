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
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/misc/default_values.sol"));
    assert_impl(result, "../resources/misc/default_values.rs");
}

#[test]
fn test_ext() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/ext/1.sol"));
    assert_impl(result, "../resources/ext/1.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/ext/2.sol"));
    assert_impl(result, "../resources/ext/2.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/ext/3.sol"));
    assert_impl(result, "../resources/ext/3.rs");
}

#[test]
fn test_ownable() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/contracts/ownable.sol"));
    assert_impl(result, "../resources/contracts/ownable.rs");
}

#[test]
fn test_types() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/types/enum.sol"));
    assert_impl(result, "../resources/types/enum.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/types/bytes.sol"));
    assert_impl(result, "../resources/types/bytes.rs");

    let result = parse::<OdraParser, _>(include_str!("../../../../resources/types/array.sol"));
    assert_impl(result, "../resources/types/array.rs");
}

#[test]
#[ignore]
fn test_plascoin() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/contracts/plascoin.sol"));
    assert_impl(result, "../resources/contracts/plascoin.rs");
}

fn assert_impl(result: TokenStream, file_path: &str) {
    let parse = |str| {
        let file = syn::parse_file(str).unwrap();
        prettyplease::unparse(&file)
    };

    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let content = content.replace("{{STACK_DEF}}", STACK_DEF);
    let content = content.replace("{{DEFAULT_MODULES}}", DEFAULT_MODULES);
    let content = content.replace("{{DEFAULT_IMPORTS}}", DEFAULT_IMPORTS);

    pretty_assertions::assert_eq!(parse(result.to_string().as_str()), parse(content.as_str()));
}

const DEFAULT_MODULES: &str = r#"
pub mod errors {}
pub mod events {}
pub mod enums {}
"#;

const DEFAULT_IMPORTS: &str = r#"
use super::errors::*;
use super::events::*;
"#;

const STACK_DEF: &str = r#"
use odra::prelude::vec::Vec;
#[cfg(not(target_arch = "wasm32"))]
impl odra::types::contract_def::Node for PathStack {
    const COUNT: u32 = 0;
    const IS_LEAF: bool = false;
}
impl odra::OdraItem for PathStack {
    fn is_module() -> bool {
        false
    }
}
impl odra::StaticInstance for PathStack {
    fn instance<'a>(keys: &'a [&'a str]) -> (Self, &'a [&'a str]) {
        (PathStack::default(), keys)
    }
}
impl odra::DynamicInstance for PathStack {
    #[allow(unused_variables)]
    fn instance(namespace: &[u8]) -> Self {
        PathStack::default()
    }
}

#[derive(Clone, Default)]
struct PathStack {
    stack: alloc::rc::Rc<core::cell::RefCell<Vec<Vec<ClassName>>>>
}

impl PathStack {
    pub fn push_path_on_stack(&self, path: &[ClassName]) {
        let mut stack = self.stack.take();
        stack.push(path.to_vec());
        self.stack.replace(stack);
    }
    pub fn drop_one_from_stack(&self) {
        let mut stack = self.stack.take();
        stack.pop().unwrap();
        self.stack.replace(stack);
    }
    pub fn pop_from_top_path(&self) -> ClassName {
        let mut stack = self.stack.take();
        let mut path = stack.pop().unwrap();
        let class = path.pop().unwrap();
        stack.push(path);
        self.stack.replace(stack);
        class
    }
}
"#;
