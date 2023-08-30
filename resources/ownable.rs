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
    #![allow(unused_braces, non_snake_case)]

    use super::errors::*;
    use super::events::*;

    impl odra::types::contract_def::Node for PathStack {
        const COUNT: u32 = 0;
        const IS_LEAF: bool = false;
    }
    impl odra::OdraItem for PathStack {
        fn is_module() -> bool {
            false
        }
    }
    impl odra::StaticInstance for PathStack {
        fn instance<'a>(keys: &'a [&'a str]) -> (Self, &'a [&'a str]) {
            (PathStack::default(), keys)
        }
    }
    impl odra::DynamicInstance for PathStack {
        #[allow(unused_variables)]
        fn instance(namespace: &[u8]) -> Self {
            PathStack::default()
        }
    }
    #[derive(Clone)]
    struct PathStack {
        stack: std::sync::Arc<std::sync::Mutex<Vec<Vec<ClassName>>>>,
    }
    impl PathStack {
        pub fn new() -> Self {
            PathStack {
                stack: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            }
        }
        pub fn push_path_on_stack(&self, path: &[ClassName]) {
            let mut stack = self.stack.lock().unwrap();
            stack.push(path.to_vec());
        }
        pub fn drop_one_from_stack(&self) {
            let mut stack = self.stack.lock().unwrap();
            stack.pop().unwrap();
        }
        pub fn pop_from_top_path(&self) -> ClassName {
            let mut stack = self.stack.lock().unwrap();
            let mut path = stack.pop().unwrap();
            let class = path.pop().unwrap();
            stack.push(path);
            class
        }
    }
    impl Default for PathStack {
        fn default() -> PathStack {
            PathStack::new()
        }
    }
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