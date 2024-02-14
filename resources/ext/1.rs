{{DEFAULT_MODULES}}

pub mod external_contract {
    #![allow(unused_imports)]
    use odra::prelude::*;

    #[odra::external_contract]
    pub trait ExternalContract {
        fn get_value(&self) -> nysa_types::U256;
        fn set_value(&mut self, new_value: nysa_types::U256);
    }
}

pub mod my_contract {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    use super::external_contract::*;
    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::MyContract],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        MyContract,
    }

    #[odra::module] 
    pub struct MyContract { 
    } 

    #[odra::module] 
    impl MyContract { 
        pub fn init(&mut self) {
        }

        pub fn read_external_contract_value(&self, _addr: Option<odra::Address>) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_read_external_contract_value(_addr);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_read_external_contract_value(&self, _addr: Option<odra::Address>) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::MyContract) => {
                    let mut external_contract = ExternalContractContractRef::new(self.env(), odra::UnwrapOrRevert::unwrap_or_revert(_addr, &self.env()));
                    return external_contract.get_value()
                }
                #[allow(unreachable_patterns)]
                _ => self.super_read_external_contract_value(_addr)
            }
        }

        pub fn write_external_contract_value(&mut self, _addr: Option<odra::Address>, new_value: nysa_types::U256) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_write_external_contract_value(_addr, new_value);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_write_external_contract_value(&mut self, _addr: Option<odra::Address>, new_value: nysa_types::U256) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::MyContract) => {
                    let mut external_contract = ExternalContractContractRef::new(self.env(), odra::UnwrapOrRevert::unwrap_or_revert(_addr, &self.env()));
                    external_contract.set_value(new_value);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_write_external_contract_value(_addr, new_value)
            }
        }
    }
}
