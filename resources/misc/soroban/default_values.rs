{{DEFAULT_MODULES}}
pub mod my_contract {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports, unused_variables)]

    {{DEFAULT_IMPORTS}}
    const MY_NUMBER: soroban_sdk::Symbol = soroban_sdk::symbol_short!("MY_NUMBER");
    const MIN_INT: soroban_sdk::Symbol = soroban_sdk::symbol_short!("MIN_INT");
    const NEG: soroban_sdk::Symbol = soroban_sdk::symbol_short!("NEG");
    const BOO: soroban_sdk::Symbol = soroban_sdk::symbol_short!("BOO");
    const MY_UNIT_2: soroban_sdk::Symbol = soroban_sdk::symbol_short!("MY_UNIT_2");
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
        MyContract
    }

    #[soroban_sdk::contract]
    pub struct MyContract { 
        my_number: odra::Var<nysa_types::U256>,
        min_int: odra::Var<nysa_types::I256>,
        neg: odra::Var<nysa_types::I32>,
        boo: odra::Var<bool>,
        my_uint_2: odra::Var<nysa_types::U256>,
    } 

    #[soroban_sdk::contractimpl]
    impl MyContract { 
        pub const MY_UINT: nysa_types::U192 = nysa_types::U192::from_limbs([123u64, 0u64, 0u64]);
        pub const NAME: &str = "my name";
        pub const FLAG: bool = false;
        pub const BYTE_ARRAY: nysa_types::FixedBytes<2usize> = nysa_types::FixedBytes([171u8, 205u8]);
        
        pub fn get_my_number(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) -> soroban_sdk::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = Self::super_get_my_number(env, caller);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get_my_number(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) -> soroban_sdk::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::MyContract) => {
                    return env.storage.persistent().get(&MY_NUMBER).unwrap_or_default();
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_get_my_number(env, caller),
            }
        }

        pub fn init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _my_uint: soroban_sdk::U256) {
            env.storage.persistent().set(&MY_NUMBER, soroban_sdk::U256::from_parts(&env, 0u64, 0u64, 0u64, 42u64));
            env.storage.persistent().set(&MIN_INT, soroban_sdk::U256::from_parts(&env, 0u64, 0u64, 0u64, 42u64));
            env.storage.persistent().set(&NEG, soroban_sdk::U256::from_parts(&env, 0u64, 0u64, 0u64, 42u64));
            env.storage.persistent().set(&BOO, &true);
            env.storage.persistent().set(&MY_UNIT_2, &_my_uint);

            self.my_number.set(nysa_types::U256::from_limbs_slice(&[42u64]));
            self.min_int.set(nysa_types::I256::MIN);
            self.neg.set(-nysa_types::I32::from_limbs_slice(&[9u64]));
            self.boo.set(true);
            self.my_uint_2.set(_my_uint);
        }
    }
}