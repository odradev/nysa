pub mod errors {}
pub mod events {}
pub mod my_contract {
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
        MyContract
    }

    #[odra::module] 
    pub struct MyContract { 
        __stack: PathStack, 
        my_number: odra::Variable<odra::types::U256>,
        min_int: odra::Variable<i16>,
        boo: odra::Variable<bool>
    } 

    #[odra::module] 
    impl MyContract { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::MyContract];

        pub fn get_my_number(&self) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get_my_number();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get_my_number(&self) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::MyContract => {
                    return self.my_number.get_or_default();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_my_number(),
            }
        }

        #[odra(init)]
        pub fn init(&mut self) {
            self.my_number.set(42u8.into());
            self.min_int.set(i16::MIN);
            self.boo.set(true);
        }
    }
}