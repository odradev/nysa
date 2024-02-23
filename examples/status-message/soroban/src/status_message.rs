pub mod errors {}
pub mod events {}
pub mod enums {}
pub mod structs {}
pub mod status_message {
    use super::errors::*;
    use super::events::*;
    use super::structs::*;
    #[soroban_sdk::contracttype]
    pub struct Records(pub Option<soroban_sdk::Address>);
    #[derive(Clone)]
    struct PathStack {
        path: [ClassName; MAX_PATH_LENGTH],
        stack_pointer: usize,
        path_pointer: usize,
    }
    impl PathStack {
        pub fn push_path_on_stack(&mut self) {
            self.path_pointer = 0;
            if self.stack_pointer < MAX_STACK_SIZE {
                self.stack_pointer += 1;
            }
        }
        pub fn drop_one_from_stack(&mut self) {
            if self.stack_pointer > 0 {
                self.stack_pointer -= 1;
            }
        }
        pub fn pop_from_top_path(&mut self) -> Option<ClassName> {
            if self.path_pointer < MAX_PATH_LENGTH {
                let class = self.path[MAX_PATH_LENGTH - self.path_pointer - 1];
                self.path_pointer += 1;
                Some(class)
            } else {
                None
            }
        }
    }
    static mut STACK: PathStack = PathStack::new();
    const MAX_STACK_SIZE: usize = 8;
    const MAX_PATH_LENGTH: usize = 1usize;
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::StatusMessage],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum ClassName {
        StatusMessage,
    }
    #[soroban_sdk::contract]
    pub struct StatusMessage {}
    #[soroban_sdk::contractimpl]
    impl StatusMessage {
        pub fn get_status(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            account_id: Option<soroban_sdk::Address>,
        ) -> soroban_sdk::String {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super_get_status(env, caller, account_id);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super_get_status(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            account_id: Option<soroban_sdk::Address>,
        ) -> soroban_sdk::String {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::StatusMessage) => {
                    return env
                        .storage()
                        .persistent()
                        .get::<_, soroban_sdk::String>(&Records(account_id))
                        .unwrap_or(soroban_sdk::String::from_str(&env, ""));
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_get_status(env, caller, account_id),
            }
        }
        pub fn init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {}
        pub fn set_status(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            status: soroban_sdk::String,
        ) {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super_set_status(env, caller, status);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super_set_status(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            status: soroban_sdk::String,
        ) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::StatusMessage) => {
                    let mut account_id = caller;
                    env.storage()
                        .persistent()
                        .set(&Records(account_id), &status);
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_set_status(env, caller, status),
            }
        }
    }
}
