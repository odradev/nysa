{{DEFAULT_MODULES}}
pub mod if_else {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}

    {{STACK_DEF}}

    const MAX_STACK_SIZE: usize = 8;
    const MAX_PATH_LENGTH: usize = 1usize;
    
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::IfElse],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        IfElse,
    }
    #[odra::module]
    pub struct IfElse {}

    #[odra::module]
    impl IfElse {
        pub fn foo(&self, x: nysa_types::U256) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_foo(x);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }
        fn super_foo(&self, x: nysa_types::U256) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::IfElse) => {
                    if x < nysa_types::U256::from_limbs_slice(&[10u64]) {
                        return nysa_types::U256::ZERO;
                    } else if x < nysa_types::U256::from_limbs_slice(&[20u64]) {
                        return nysa_types::U256::ONE;
                    } else {
                        return nysa_types::U256::from_limbs_slice(&[2u64]);
                    }
                }
                #[allow(unreachable_patterns)]
                _ => self.super_foo(x),
            }
        }
        pub fn init(&mut self) {}
        pub fn ternary(&self, _x: nysa_types::U24) -> nysa_types::U32 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_ternary(_x);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }
        fn super_ternary(&self, _x: nysa_types::U24) -> nysa_types::U32 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::IfElse) => {
                    return if _x < nysa_types::U24::from_limbs_slice(&[10u64]) { 
                        nysa_types::U32::ONE
                    } else { 
                        nysa_types::U32::from_limbs_slice(&[2u64]) 
                    };
                }
                #[allow(unreachable_patterns)]
                _ => self.super_ternary(_x),
            }
        }
    }
}
