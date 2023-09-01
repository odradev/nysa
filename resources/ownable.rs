pub mod errors {}
pub mod events {
    #[derive(odra::Event, PartialEq, Eq, Debug)]
    pub struct OwnershipTransferred {
        previous_owner: Option<odra::types::Address>,
        new_owner: Option<odra::types::Address>,
    }

    impl OwnershipTransferred {
        pub fn new(
            previous_owner: Option<odra::types::Address>,
            new_owner: Option<odra::types::Address>,
        ) -> Self {
            Self { previous_owner, new_owner }
        }
    }
}
pub mod owner {
    #![allow(unused_braces, non_snake_case, unused_imports)]

    use super::errors::*;
    use super::events::*;

    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        Owner
    }

    #[odra::module(events = [OwnershipTransferred])] 
    pub struct Owner { 
        __stack: PathStack, 
        owner: odra::Variable<Option<odra::types::Address>>
    } 

    #[odra::module] 
    impl Owner { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Owner];

        pub fn get_owner(&self) -> Option<odra::types::Address> {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get_owner();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get_owner(&self) -> Option<odra::types::Address> {
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
            self.owner.set(Some(odra::contract_env::caller()));
        }

        fn modifier_before_only_owner(&mut self) {
            if !(Some(odra::contract_env::caller()) == self.owner.get().unwrap_or(None)) {
                odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "Only the contract owner can call this function."))
            };
        }

        fn modifier_after_only_owner(&mut self) {
        }

        pub fn transfer_ownership(&mut self, new_owner: Option<odra::types::Address>) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_transfer_ownership(new_owner);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_transfer_ownership(&mut self, new_owner: Option<odra::types::Address>) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Owner => {
                    self.modifier_before_only_owner();
                    let old_owner = self.owner.get().unwrap_or(None);
                    self.owner.set(new_owner);
                    <OwnershipTransferred as odra::types::event::OdraEvent>::emit(OwnershipTransferred::new(
                        old_owner, new_owner
                    ));
                    self.modifier_after_only_owner();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_transfer_ownership(new_owner),
            }
        }
    }
}