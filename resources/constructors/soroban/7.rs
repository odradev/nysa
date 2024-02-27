{{DEFAULT_MODULES}}
pub mod e {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports, unused_variables)]

    {{DEFAULT_IMPORTS}}
    const NAME: soroban_sdk::Symbol = soroban_sdk::symbol_short!("NAME");
    const TEXT: soroban_sdk::Symbol = soroban_sdk::symbol_short!("TEXT");
    {{STACK_DEF}}

    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 4usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::X, ClassName::Z, ClassName::Y, ClassName::E],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        E, Y, Z, X
    }

    #[soroban_sdk::contract]
    pub struct E { 
    } 

    #[soroban_sdk::contractimpl]
    impl E { 
        fn _x_init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _name: soroban_sdk::String) {
            env.storage().instance().set(&NAME, &_name);
        }

        fn _z_init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
        } 

        fn _y_init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _text: soroban_sdk::String) {
            env.storage().instance().set(&TEXT, &_text);
        }

        pub fn init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
            Self::_x_init(env.clone(), caller.clone(), soroban_sdk::String::from_str(&env, "X was called"));
            Self::_z_init(env.clone(), caller.clone());
            Self::_y_init(env.clone(), caller.clone(), soroban_sdk::String::from_str(&env, "Y was called"));
        }
    }
}