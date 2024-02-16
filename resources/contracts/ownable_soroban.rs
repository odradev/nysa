pub mod errors {}
pub mod events {
    #[soroban_sdk::contracttype]
    pub struct OwnershipTransferred {
        previous_owner: Option<soroban_sdk::Address>,
        new_owner: Option<soroban_sdk::Address>,
    }

    impl OwnershipTransferred {
        pub fn new(
            previous_owner: Option<soroban_sdk::Address>,
            new_owner: Option<soroban_sdk::Address>,
        ) -> Self {
            Self { previous_owner, new_owner }
        }
    }
}
pub mod enums {}
pub mod structs {}
pub mod owner {
    const OWNER: soroban_sdk::Symbol = soroban_sdk::symbol_short!("OWNER");

    {{STACK_DEF}}

    const MAX_STACK_SIZE: usize = 8;
    const MAX_PATH_LENGTH: usize = 1usize;
    
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::Owner],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        Owner
    }

    #[soroban_sdk::contract]
    pub struct Owner {
    }

    #[soroban_sdk::contractimpl]
    impl Owner {
        pub fn get_owner(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
            unsafe { STACK.push_path_on_stack(); }
            let result = Self::super_get_owner();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get_owner(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    return Self::owner.get().unwrap_or(None);
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_get_owner(),
            }
        }

        pub fn init(env: soroban_sdk::Env, caller: soroban_sdk::Address) {
            env.storage().persistent().has(&OWNER).then(|| {
                panic!("Owner already set");
            });

            env.storage().persistent().set(&OWNER, &caller);
        }

        fn modifier_before_only_owner(env: soroban_sdk::Env, caller: soroban_sdk::Address) {
            caller.require_auth();
        }

        fn modifier_after_only_owner(env: soroban_sdk::Env, caller: soroban_sdk::Address) {
        }

        pub fn transfer_ownership(env: soroban_sdk::Env, new_owner: Option<soroban_sdk::Address>) {
            unsafe { STACK.push_path_on_stack(); }
            let result = Self::super_transfer_ownership(env, new_owner);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_transfer_ownership(env: soroban_sdk::Env, new_owner: Option<soroban_sdk::Address>) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    Self::modifier_before_only_owner();
                    let mut old_owner = env.storage().persistent().get(&OWNER).unwrap_or(None);
                    env.storage().persistent().set(&OWNER, &new_owner);
                    env.events().publish((), OwnershipTransferred::new(old_owner, new_owner));
                    Self::modifier_after_only_owner();
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_transfer_ownership(env, new_owner),
            }
        }
    }
}