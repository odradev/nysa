use crate::parse;
use quote::ToTokens;
use std::{fs::File, io::Read, path::Path};

use super::*;

#[test]
fn test_constructor() {
    test_many(7, "constructors")
}

#[test]
fn test_modifier() {
    test_single("modifiers", "1");
}

#[test]
fn test_default_value() {
    test_single("misc", "default_values");
}

#[test]
fn test_ext() {
    test_many(3, "ext")
}

#[test]
fn test_ownable() {
    test_single("contracts", "ownable");
}

#[test]
fn test_array() {
    test_single("types", "array");
}

#[test]
fn test_enum() {
    test_single("types", "enum");
}

#[test]
fn test_bytes() {
    test_single("types", "bytes");
}

#[test]
fn test_cast() {
    test_single("types", "cast");
}

#[test]
#[ignore]
fn test_plascoin() {
    test_single("contracts", "plascoin");
}

#[test]
fn test_if_else() {
    test_single("conditionals", "ifelse");
}

#[test]
fn test_lib_math() {
    test_single("library", "math");
}

#[test]
fn test_lib_safe_math() {
    test_single("library", "safe_math");
}

#[test]
#[ignore]
fn test_bitwise_ops() {
    test_single("op", "bitwise");
}

fn test_many(count: usize, base_path: &str) {
    for i in 1..=count {
        let path = read_file(format!("../resources/{}/{}.sol", base_path, i));
        let result = parse::<OdraParser, _>(path.as_str());
        assert_impl(result, format!("../resources/{}/{}.rs", base_path, i));
    }
}

fn test_single(base_path: &str, test_case: &str) {
    let path = read_file(format!("../resources/{}/{}.sol", base_path, test_case));
    let result = parse::<OdraParser, _>(path.as_str());
    assert_impl(
        result,
        format!("../resources/{}/{}.rs", base_path, test_case),
    );
}

fn assert_impl<P: AsRef<Path>>(result: TokenStream, file_path: P) {
    let parse = |str| {
        let file = syn::parse_file(str).unwrap();
        prettyplease::unparse(&file)
    };

    let content = read_file(file_path);
    let content = content.replace("{{STACK_DEF}}", STACK_DEF);
    let content = content.replace("{{DEFAULT_MODULES}}", DEFAULT_MODULES);
    let content = content.replace("{{DEFAULT_IMPORTS}}", DEFAULT_IMPORTS);

    pretty_assertions::assert_eq!(parse(result.to_string().as_str()), parse(content.as_str()));
}

pub(crate) fn assert_tokens_eq<L, R>(left: L, right: R)
where
    L: ToTokens,
    R: ToTokens,
{
    assert_eq!(
        left.into_token_stream().to_string(),
        right.into_token_stream().to_string()
    )
}

fn read_file<P: AsRef<Path>>(file_path: P) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
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
impl odra::types::OdraItem for PathStack {
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
