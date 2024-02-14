pub mod errors {}
pub mod events {
    use odra::prelude::*;
}
pub mod enums {}
pub mod structs {}
pub mod array {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
    {{DEFAULT_IMPORTS}}
    {{STACK_DEF}}
    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::Array],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum ClassName {
        Array,
    }
    #[odra::module]
    pub struct Array {
        arr: odra::Var<odra::prelude::vec::Vec<nysa_types::U256>>,
        arr_2: odra::Var<odra::prelude::vec::Vec<nysa_types::U256>>,
        my_fixed_size_arr: odra::Var<odra::prelude::vec::Vec<nysa_types::U256>>,
    }

    #[odra::module]
    impl Array {
        pub fn examples(&self) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_examples();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_examples(&self) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Array) => {
                    let mut a = odra::prelude::vec::Vec::with_capacity(
                        nysa_types::U256::from_limbs_slice(&[5u64]),
                    );
                    a[1] = nysa_types::U256::from_limbs_slice(&[123u64]);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_examples(),
            }
        }

        pub fn get(&self, i: nysa_types::U256) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get(i);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get(&self, i: nysa_types::U256) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Array) => {
                    return self.arr.get_or_default()[i.as_usize()];
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get(i),
            }
        }

        pub fn get_arr(&self) -> odra::prelude::vec::Vec<nysa_types::U256> {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get_arr();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get_arr(&self) -> odra::prelude::vec::Vec<nysa_types::U256> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Array) => {
                    return self.arr.get_or_default();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_arr(),
            }
        }

        pub fn get_length(&self) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get_length();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get_length(&self) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Array) => {
                    return self.arr.get_or_default().len().into();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_length(),
            }
        }

        pub fn init(&mut self) {
            self.arr_2.set(odra::prelude::vec![
                nysa_types::U256::ONE,
                nysa_types::U256::from_limbs_slice(&[2u64]),
                nysa_types::U256::from_limbs_slice(&[3u64])
            ]);
        }

        pub fn pop(&mut self) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_pop();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }
        
        fn super_pop(&mut self) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Array) => {
                    {
                        let mut result = self.arr.get_or_default();
                        result.pop();
                        self.arr.set(result);
                    };
                }
                #[allow(unreachable_patterns)]
                _ => self.super_pop(),
            }
        }

        pub fn push(&mut self, i: nysa_types::U256) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_push(i);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_push(&mut self, i: nysa_types::U256) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Array) => {
                    {
                        let mut result = self.arr.get_or_default();
                        result.push(i);
                        self.arr.set(result);
                    };
                }
                #[allow(unreachable_patterns)]
                _ => self.super_push(i),
            }
        }

        pub fn remove(&mut self, index: nysa_types::U256) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_remove(index);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_remove(&mut self, index: nysa_types::U256) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Array) => {
                    {
                        let mut result = self.arr.get_or_default();
                        result[index] = Default::default();
                        self.arr.set(result);
                    };
                }
                #[allow(unreachable_patterns)]
                _ => self.super_remove(index),
            }
        }
    }
}
