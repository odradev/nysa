{{DEFAULT_MODULES}}
pub mod callee {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}

    {{STACK_DEF}}

    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::Callee],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        Callee,
    }

    #[odra::module] 
    pub struct Callee { 
        x: odra::Var<nysa_types::U256>,
    } 

    #[odra::module] 
    impl Callee { 
        pub fn init(&mut self) {
        }

        pub fn set_x(&mut self, _x: nysa_types::U256) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_set_x(_x);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_set_x(&mut self, _x: nysa_types::U256) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Callee) => {
                    self.x.set(_x);
                    return self.x.get_or_default()
                }
                #[allow(unreachable_patterns)]
                _ => self.super_set_x(_x)
            }
        }
    }
}

pub mod caller {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    use super::callee::*;
    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::Caller],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        Caller,
    }

    #[odra::module] 
    pub struct Caller { 
    } 

    #[odra::module] 
    impl Caller { 
        pub fn init(&mut self) {
        }

        pub fn set_x(&mut self, _callee: Option<odra::Address>, _x: nysa_types::U256) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_set_x(_callee, _x);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_set_x(&mut self, _callee: Option<odra::Address>, _x: nysa_types::U256) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Caller) => {
                    let mut _callee = CalleeContractRef::new(self.env(), odra::UnwrapOrRevert::unwrap_or_revert(_callee, &self.env()));
                    let mut x = _callee.set_x(_x);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_set_x(_callee, _x)
            }
        }

        pub fn set_x_from_address(&mut self, _addr: Option<odra::Address>, _x: nysa_types::U256) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_set_x_from_address(_addr, _x);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_set_x_from_address(&mut self, _addr: Option<odra::Address>, _x: nysa_types::U256) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Caller) => {
                    let mut callee = CalleeContractRef::new(self.env(), odra::UnwrapOrRevert::unwrap_or_revert(_addr, &self.env()));
                    callee.set_x(_x);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_set_x_from_address(_addr, _x)
            }
        }  
    }
}
