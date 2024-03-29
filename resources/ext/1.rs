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
        }

        pub fn read_external_contract_value(&self, _addr: Option<odra::Address>) -> nysa_types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_read_external_contract_value(_addr);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_read_external_contract_value(&self, _addr: Option<odra::Address>) -> nysa_types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::MyContract => {
                    let mut external_contract = ExternalContractContractRef::new(self.env(), odra::UnwrapOrRevert::unwrap_or_revert(_addr, &self.env()));
                    return external_contract.get_value()
                }
                #[allow(unreachable_patterns)]
                _ => self.super_read_external_contract_value(_addr)
            }
        }

        pub fn write_external_contract_value(&mut self, _addr: Option<odra::Address>, new_value: nysa_types::U256) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_write_external_contract_value(_addr, new_value);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_write_external_contract_value(&mut self, _addr: Option<odra::Address>, new_value: nysa_types::U256) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::MyContract => {
                    let mut external_contract = ExternalContractContractRef::new(self.env(), odra::UnwrapOrRevert::unwrap_or_revert(_addr, &self.env()));
                    external_contract.set_value(new_value);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_write_external_contract_value(_addr, new_value)
            }
        }
    }
}
