pub mod errors {}
pub mod events {
    use odra::prelude::*;
    #[derive(odra::Event, PartialEq, Eq, Debug)]
    pub struct OwnershipTransferred {
        previous_owner: Option<odra::Address>,
        new_owner: Option<odra::Address>,
    }

    impl OwnershipTransferred {
        pub fn new(
            previous_owner: Option<odra::Address>,
            new_owner: Option<odra::Address>,
        ) -> Self {
            Self { previous_owner, new_owner }
        }
    }
}
pub mod enums {}
pub mod structs {}
pub mod owner {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}

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

    #[odra::module(events = [OwnershipTransferred])] 
    pub struct Owner { 
        owner: odra::Var<Option<odra::Address>>
    } 

    #[odra::module] 
    impl Owner { 
        pub fn get_owner(&self) -> Option<odra::Address> {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get_owner();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get_owner(&self) -> Option<odra::Address> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    return self.owner.get().unwrap_or(None);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_owner(),
            }
        }

        pub fn init(&mut self) {
            self.owner.set(Some(self.env().caller()));
        }

        fn modifier_before_only_owner(&mut self) {
            if !(Some(self.env().caller()) == self.owner.get().unwrap_or(None)) {
                self.env().revert(odra::ExecutionError::User(1u16))
            };
        }

        fn modifier_after_only_owner(&mut self) {
        }

        pub fn transfer_ownership(&mut self, new_owner: Option<odra::Address>) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_transfer_ownership(new_owner);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_transfer_ownership(&mut self, new_owner: Option<odra::Address>) {
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