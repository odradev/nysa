{{DEFAULT_MODULES}}
pub mod if_else {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports, unused_variables)]
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
    #[soroban_sdk::contract]
    pub struct IfElse {}

    #[soroban_sdk::contractimpl]
    impl IfElse {
        pub fn foo(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, x: soroban_sdk::U256) -> soroban_sdk::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = Self::super_foo(env, caller, x);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }
        fn super_foo(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, x: soroban_sdk::U256) -> soroban_sdk::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::IfElse) => {
                    if x < soroban_sdk::U256::from_parts(&env, 0u64, 0u64, 0u64, 10u64) {
                        return soroban_sdk::U256::from_parts(&env, 0u64, 0u64, 0u64, 0u64);
                    } else if x < soroban_sdk::U256::from_parts(&env, 0u64, 0u64, 0u64, 20u64) {
                        return soroban_sdk::U256::from_parts(&env, 0u64, 0u64, 0u64, 1u64);
                    } else {
                        return soroban_sdk::U256::from_parts(&env, 0u64, 0u64, 0u64, 2u64);
                    }
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_foo(env, caller, x),
            }
        }
        pub fn init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {}
        pub fn ternary(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _x: u32) -> u32 {
            unsafe { STACK.push_path_on_stack(); }
            let result = Self::super_ternary(env, caller, _x);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }
        fn super_ternary(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _x: u32) -> u32 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::IfElse) => {
                    return if _x < u32::from_le_bytes([10u8, 0u8, 0u8, 0u8]) { 
                        u32::from_le_bytes([1u8, 0u8, 0u8, 0u8])
                    } else { 
                        u32::from_le_bytes([2u8, 0u8, 0u8, 0u8])
                    };
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_ternary(env, caller, _x),
            }
        }
    }
}
