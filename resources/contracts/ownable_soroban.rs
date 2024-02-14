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
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{STACK_DEF}}

    #[derive(Clone, Copy)]
    enum ClassName {
        Owner
    }

    const OWNER: soroban_sdk::Symbol = soroban_sdk::symbol_short!("OWNER");

    #[soroban_sdk::contract]
    pub struct Owner {
        : PathStack,
    }

    #[soroban_sdk::contractimpl]
    impl Owner {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Owner];

        pub fn get_owner(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get_owner();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get_owner(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    return self.owner.get().unwrap_or(None);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_owner(),
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

        pub fn transfer_ownership(&mut self, new_owner: Option<soroban_sdk::Address>) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_transfer_ownership(new_owner);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_transfer_ownership(&mut self, new_owner: Option<soroban_sdk::Address>) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    self.modifier_before_only_owner();
                    let mut old_owner = self.owner.get().unwrap_or(None);
                    self.owner.set(new_owner);
                    self.env().emit_event(OwnershipTransferred::new(old_owner, new_owner));
                    self.modifier_after_only_owner();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_transfer_ownership(new_owner),
            }
        }
    }
}