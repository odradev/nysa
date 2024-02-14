{{DEFAULT_MODULES}}
pub mod function_modifier {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}
    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::FunctionModifier],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum ClassName {
        FunctionModifier,
    }

    #[odra::module] 
    pub struct FunctionModifier { 
        owner: odra::Var<Option<odra::Address>>,
        x: odra::Var<nysa_types::U32>,
        locked: odra::Var<bool>
    } 

    #[odra::module] 
    impl FunctionModifier { 
        pub fn change_owner(&mut self, _new_owner: Option<odra::Address>) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_change_owner(_new_owner);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_change_owner(&mut self, _new_owner: Option<odra::Address>) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::FunctionModifier) => {
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
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_decrement(i);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_decrement(&mut self, i: nysa_types::U32)  {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::FunctionModifier) => {
                    self.modifier_before_no_reentrancy();

                    self.x.set(self.x.get_or_default() - i);

                    if i > nysa_types::U32::ONE {
                        self.decrement((i - nysa_types::U32::ONE));
                    }

                    self.modifier_after_no_reentrancy();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_decrement(i),
            }
        }

        pub fn init(&mut self) {
            self.owner.set(Some(self.env().caller()));
            self.x.set(nysa_types::U32::from_limbs_slice(&[10u64]));
        }

        fn modifier_before_no_reentrancy(&mut self) {
            if !(!(self.locked.get_or_default())) {
                self.env().revert(odra::ExecutionError::User(1u16))
            };
            self.locked.set(true);
        }

        fn modifier_after_no_reentrancy(&mut self) {
            self.locked.set(false);
        }

        fn modifier_before_only_owner(&mut self) {
            if !(Some(self.env().caller()) == self.owner.get().unwrap_or(None)) {
                self.env().revert(odra::ExecutionError::User(1u16))
            };
        }

        fn modifier_after_only_owner(&mut self) {
        }

        fn modifier_before_valid_address(&mut self, _addr: Option<odra::Address>) {
            if !(_addr != None) {
                self.env().revert(odra::ExecutionError::User(1u16))
            };
        }

        fn modifier_after_valid_address(&mut self,  _addr: Option<odra::Address>) {
        }
    }
}