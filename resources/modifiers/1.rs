pub mod errors {}
pub mod events {}
pub mod function_modifier {
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
        FunctionModifier,
    }

    #[odra::module] 
    pub struct FunctionModifier { 
        __stack: PathStack, 
        owner: odra::Variable<Option<odra::types::Address>>,
        x: odra::Variable<u32>,
        locked: odra::Variable<bool>
    } 

    #[odra::module] 
    impl FunctionModifier { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::FunctionModifier];

        pub fn change_owner(&mut self, _new_owner: Option<odra::types::Address>) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_change_owner(_new_owner);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_change_owner(&mut self, _new_owner: Option<odra::types::Address>) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::FunctionModifier => {
                    self.modifier_before_only_owner();
                    self.modifier_before_valid_address(_new_owner);
                    self.owner.set(_new_owner);
                    self.modifier_after_valid_address(_new_owner);
                    self.modifier_after_only_owner();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_change_owner(_new_owner)
            }
        }

        pub fn decrement(&mut self, i: u32)  {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_decrement(i);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_decrement(&mut self, i: u32)  {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::FunctionModifier => {
                    self.modifier_before_no_reentrancy();

                    self.x.set(self.x.get_or_default() - i);

                    if i > 1u8.into() {
                        self.decrement(i - 1);
                    }

                    self.modifier_after_no_reentrancy();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_decrement(i),
            }
        }

        #[odra(init)]
        pub fn init(&mut self) {
            self.owner.set(Some(odra::contract_env::caller()));
            self.x.set(10u8.into());
        }

        fn modifier_before_no_reentrancy(&mut self) {
            if !(!(self.locked.get_or_default())) {
                odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "No reentrancy"))
            };
            self.locked.set(true);
        }

        fn modifier_after_no_reentrancy(&mut self) {
            self.locked.set(false);
        }

        fn modifier_before_only_owner(&mut self) {
            if !(Some(odra::contract_env::caller()) == self.owner.get().unwrap_or(None)) {
                odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "Not owner"))
            };
        }

        fn modifier_after_only_owner(&mut self) {
        }

        fn modifier_before_valid_address(&mut self, _addr: Option<odra::types::Address>) {
            if !(_addr != None) {
                odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "Not valid address"))
            };
        }

        fn modifier_after_valid_address(&mut self,  _addr: Option<odra::types::Address>) {
        }
    }
}