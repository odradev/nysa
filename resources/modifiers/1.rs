{{DEFAULT_MODULES}}
pub mod function_modifier {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        FunctionModifier,
    }

    #[odra::module] 
    pub struct FunctionModifier { 
        __stack: PathStack, 
        owner: odra::Variable<Option<odra::types::Address>>,
        x: odra::Variable<nysa_types::U32>,
        locked: odra::Variable<bool>
    } 

    #[odra::module] 
    impl FunctionModifier { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::FunctionModifier];

        pub fn change_owner(&mut self, _new_owner: Option<odra::types::Address>) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_change_owner(_new_owner);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_change_owner(&mut self, _new_owner: Option<odra::types::Address>) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::FunctionModifier => {
                    self.modifier_before_only_owner();
                    self.modifier_before_valid_address(_new_owner);
                    self.owner.set(_new_owner);
                    self.modifier_after_valid_address(_new_owner);
                    self.modifier_after_only_owner();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_change_owner(_new_owner)
            }
        }

        pub fn decrement(&mut self, i: nysa_types::U32)  {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_decrement(i);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_decrement(&mut self, i: nysa_types::U32)  {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::FunctionModifier => {
                    self.modifier_before_no_reentrancy();

                    self.x.set(self.x.get_or_default() - i);

                    if i > nysa_types::U32::from_limbs_slice(&[1u64]) {
                        self.decrement((i - nysa_types::U32::from_limbs_slice(&[1u64])));
                    }

                    self.modifier_after_no_reentrancy();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_decrement(i),
            }
        }

        #[odra(init)]
        pub fn init(&mut self) {
            self.owner.set(Some(odra::contract_env::caller()));
            self.x.set(nysa_types::U32::from_limbs_slice(&[10u64]));
        }

        fn modifier_before_no_reentrancy(&mut self) {
            if !(!(self.locked.get_or_default())) {
                odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "No reentrancy"))
            };
            self.locked.set(true);
        }

        fn modifier_after_no_reentrancy(&mut self) {
            self.locked.set(false);
        }

        fn modifier_before_only_owner(&mut self) {
            if !(Some(odra::contract_env::caller()) == self.owner.get().unwrap_or(None)) {
                odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "Not owner"))
            };
        }

        fn modifier_after_only_owner(&mut self) {
        }

        fn modifier_before_valid_address(&mut self, _addr: Option<odra::types::Address>) {
            if !(_addr != None) {
                odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "Not valid address"))
            };
        }

        fn modifier_after_valid_address(&mut self,  _addr: Option<odra::types::Address>) {
        }
    }
}