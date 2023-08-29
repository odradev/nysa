pub mod errors {}
pub mod events {}

pub mod external_contract {
    #[odra::external_contract]
    pub trait ExternalContract {
        fn get_value(&self) -> odra::types::U256;
        fn set_value(&mut self, new_value: odra::types::U256);
    }
}

pub mod my_contract {
    #![allow(unused_braces, non_snake_case)]

    use super::external_contract::*;
    use super::errors::*;
    use super::events::*;

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
    #[derive(Clone)]
    struct PathStack {
        stack: std::sync::Arc<std::sync::Mutex<Vec<Vec<ClassName>>>>,
    }
    impl PathStack {
        pub fn new() -> Self {
            PathStack {
                stack: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            }
        }
        pub fn push_path_on_stack(&self, path: &[ClassName]) {
            let mut stack = self.stack.lock().unwrap();
            stack.push(path.to_vec());
        }
        pub fn drop_one_from_stack(&self) {
            let mut stack = self.stack.lock().unwrap();
            stack.pop().unwrap();
        }
        pub fn pop_from_top_path(&self) -> ClassName {
            let mut stack = self.stack.lock().unwrap();
            let mut path = stack.pop().unwrap();
            let class = path.pop().unwrap();
            stack.push(path);
            class
        }
    }
    impl Default for PathStack {
        fn default() -> PathStack {
            PathStack::new()
        }
    }
    #[derive(Clone)]
    enum ClassName {
        MyContract,
    }

    #[odra::module] 
    pub struct MyContract { 
        __stack: PathStack,
    } 

    #[odra::module] 
    impl MyContract { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::MyContract];

        #[odra(init)]
        pub fn init(&mut self) {
            {}
        }

        pub fn read_external_contract_value(&self, _addr: Option<odra::types::Address>) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_read_external_contract_value(_addr);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_read_external_contract_value(&self, _addr: Option<odra::types::Address>) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::MyContract => {
                    let mut external_contract = ExternalContractRef::at(&odra::UnwrapOrRevert::unwrap_or_revert(_addr));
                    return external_contract.get_value()
                }
                #[allow(unreachable_patterns)]
                _ => self.super_read_external_contract_value(_addr)
            }
        }

        pub fn write_external_contract_value(&mut self, _addr: Option<odra::types::Address>, new_value: odra::types::U256) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_write_external_contract_value(_addr, new_value);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_write_external_contract_value(&mut self, _addr: Option<odra::types::Address>, new_value: odra::types::U256) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::MyContract => {
                    let mut external_contract = ExternalContractRef::at(&odra::UnwrapOrRevert::unwrap_or_revert(_addr));
                    external_contract.set_value(new_value);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_write_external_contract_value(_addr, new_value)
            }
        }
    }
}
