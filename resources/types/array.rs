pub mod errors {}
pub mod events {}
pub mod enums {}

pub mod array {
    #![allow(unused_braces, non_snake_case, unused_imports)]
    {{DEFAULT_IMPORTS}}
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        Array,
    }
    #[odra::module]
    pub struct Array {
        __stack: PathStack,
        arr: odra::Variable<Vec<odra::types::U256>>,
        arr_2: odra::Variable<Vec<odra::types::U256>>,
        my_fixed_size_arr: odra::Variable<Vec<odra::types::U256>>,
    }


    #[odra::module]
    impl Array {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Array];

        pub fn examples(&mut self) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_examples();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_examples(&mut self) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Array => {
                    let mut a = odra::prelude::vec::Vec::with_capacity(5u8.into());
                    a[0] = 123.into();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_examples(),
            }
        }

        pub fn get(&self, i: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get(i);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get(&self, i: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Array => {
                    return self.arr.get_or_default()[i.as_usize()];
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get(i),
            }
        }

        pub fn get_arr(&self) -> odra::prelude::vec::Vec<odra::types::U256> {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get_arr();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get_arr(&self) -> odra::prelude::vec::Vec<odra::types::U256> {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Array => {
                    return self.arr.get_or_default();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_arr(),
            }
        }

        pub fn get_length(&self) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get_length();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get_length(&self) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Array => {
                    return self.arr.get_or_default().len().into();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_length(),
            }
        }

        #[odra(init)]
        pub fn init(&mut self) {
            self.arr_2.set(odra::prelude::vec![1u8.into(), 2u8.into(), 3u8.into()]);
        }

        pub fn pop(&mut self) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_pop();
            self.__stack.drop_one_from_stack();
            result
        }
        
        fn super_pop(&mut self) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Array => {
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

        pub fn push(&mut self, i: odra::types::U256) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_push(i);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_push(&mut self, i: odra::types::U256) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Array => {
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

        pub fn remove(&mut self, index: odra::types::U256) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_remove(index);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_remove(&mut self, index: odra::types::U256) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Array => {
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
