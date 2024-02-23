{{DEFAULT_MODULES}}
pub mod function_modifier {

    {{DEFAULT_IMPORTS}}
    const OWNER: soroban_sdk::Symbol = soroban_sdk::symbol_short!("OWNER");
    const X: soroban_sdk::Symbol = soroban_sdk::symbol_short!("X");
    const LOCKED: soroban_sdk::Symbol = soroban_sdk::symbol_short!("LOCKED");
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

    #[soroban_sdk::contract]
    pub struct FunctionModifier { 

    } 

    #[soroban_sdk::contractimpl]
    impl FunctionModifier { 
        pub fn change_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _new_owner: Option<soroban_sdk::Address>) {
            unsafe { STACK.push_path_on_stack(); }
            let result = Self::super_change_owner(env, caller, _new_owner);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_change_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _new_owner: Option<soroban_sdk::Address>) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::FunctionModifier) => {
                    Self::modifier_before_only_owner(env.clone(), caller.clone());
                    Self::modifier_before_valid_address(env.clone(), caller.clone(), _new_owner);
                    env.storage().persistent().set(&OWNER, &_new_owner);
                    Self::modifier_after_valid_address(env.clone(), caller.clone(), _new_owner);
                    Self::modifier_after_only_owner(env.clone(), caller.clone());
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_change_owner(env, caller, _new_owner)
            }
        }

        pub fn decrement(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, i: u32)  {
            unsafe { STACK.push_path_on_stack(); }
            let result = Self::super_decrement(env, caller, i);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_decrement(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, i: u32)  {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::FunctionModifier) => {
                    Self::modifier_before_no_reentrancy(env.clone(), caller.clone());

                    env.storage().persistent().set(&X, &env.storage().persistent().get::<_, u32>(&X).unwrap_or_default() - i);

                    if i > u32::from_le_bytes(&[1u8, 0u8, 0u8, 0u8]) {
                        Self::decrement(env.clone(), caller.clone(), (i - u32::from_le_bytes(&[1u8, 0u8, 0u8, 0u8])));
                    }

                    Self::modifier_after_no_reentrancy(env.clone(), caller.clone());
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_decrement(env, caller, i),
            }
        }

        pub fn init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
            env.storage().persistent().set(&OWNER, &caller);
            env.storage().persistent().set(&X, &u32::from_le_bytes(&[10u8, 0u8, 0u8, 0u8]));
        }

        fn modifier_before_no_reentrancy(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
            if !(!(env.storage().persistent().get::<_, bool>(&LOCKED).unwrap_or_default())) {
                panic!("No reentrancy")
            };
            env.storage().persistent().set(&LOCKED, &true);
        }

        fn modifier_after_no_reentrancy(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
            env.storage().persistent().set(&LOCKED, &false);
        }

        fn modifier_before_only_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
            caller.require_auth();
        }

        fn modifier_after_only_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
        }

        fn modifier_before_valid_address(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _addr: Option<soroban_sdk::Address>) {
            if !(_addr != None) {
                panic!("Not valid address")
            };
        }

        fn modifier_after_valid_address(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>,  _addr: Option<soroban_sdk::Address>) {
        }
    }
}