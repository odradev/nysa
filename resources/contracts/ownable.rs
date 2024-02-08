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

    #[derive(Clone)]
    enum ClassName {
        Owner
    }

    #[odra::module(events = [OwnershipTransferred])] 
    pub struct Owner { 
        __stack: PathStack, 
        owner: odra::Var<Option<odra::Address>>
    } 

    #[odra::module] 
    impl Owner { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Owner];

        pub fn get_owner(&self) -> Option<odra::Address> {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get_owner();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get_owner(&self) -> Option<odra::Address> {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Owner => {
                    return self.owner.get().unwrap_or(None);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_owner(),
            }
        }

        #[odra(init)]
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
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_transfer_ownership(new_owner);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_transfer_ownership(&mut self, new_owner: Option<odra::Address>) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Owner => {
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