{{DEFAULT_MODULES}}
pub mod c {
    {{DEFAULT_IMPORTS}}
    const NAME: soroban_sdk::Symbol = soroban_sdk::symbol_short!("NAME");
    const TEXT: soroban_sdk::Symbol = soroban_sdk::symbol_short!("TEXT");
    {{STACK_DEF}}

    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 3usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::X, ClassName::Y, ClassName::C],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        C, Y, X
    }
    #[soroban_sdk::contract]
    pub struct C { 
    } 

    #[soroban_sdk::contractimpl]
    impl C { 
        fn _x_init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _name: soroban_sdk::String) {
            env.storage().persistent().set(&NAME, &_name);
        }

        fn _y_init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _text: soroban_sdk::String) {
            env.storage().persistent().set(&TEXT, &_text);
        } 

        pub fn init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, _name: soroban_sdk::String, _text: soroban_sdk::String) {
            Self::_x_init(env.clone(), caller.clone(), _name);
            Self::_y_init(env.clone(), caller.clone(), _text);
        }
    }
}