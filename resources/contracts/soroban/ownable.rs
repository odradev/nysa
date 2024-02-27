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
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports, unused_variables)]

    {{DEFAULT_IMPORTS}}

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
        pub fn get_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) -> Option<soroban_sdk::Address> {
            unsafe { STACK.push_path_on_stack(); }
            let result = Self::super_get_owner(env, caller);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) -> Option<soroban_sdk::Address> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    return env.storage().instance().get::<_, Option<soroban_sdk::Address>>(&OWNER).unwrap_or(None);
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_get_owner(env, caller),
            }
        }

        pub fn init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
            env.storage().instance().set(&OWNER, &caller);
        }

        fn modifier_before_only_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
            caller.expect("`caller` must not be `None`").require_auth();
        }

        fn modifier_after_only_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
        }

        pub fn transfer_ownership(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, new_owner: Option<soroban_sdk::Address>) {
            unsafe { STACK.push_path_on_stack(); }
            let result = Self::super_transfer_ownership(env, caller, new_owner);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_transfer_ownership(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>, new_owner: Option<soroban_sdk::Address>) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    Self::modifier_before_only_owner(env.clone(), caller.clone());
                    let mut old_owner = env
                        .storage()
                        .persistent()
                        .get::<_, Option<soroban_sdk::Address>>(&OWNER)
                        .unwrap_or(None);
                    env.storage().instance().set(&OWNER, &new_owner);
                    env.events().publish((), OwnershipTransferred::new(old_owner, new_owner));
                    Self::modifier_after_only_owner(env.clone(), caller.clone());
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_transfer_ownership(env, caller, new_owner),
            }
        }
    }
}