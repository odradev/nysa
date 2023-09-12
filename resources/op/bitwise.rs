{{DEFAULT_MODULES}}
pub mod bitwise_ops {
    #![allow(unused_braces, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        BitwiseOps,
    }

    #[odra::module] 
    pub struct BitwiseOps { 
        __stack: PathStack, 
    } 

    #[odra::module] 
    impl BitwiseOps { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::BitwiseOps];

        pub fn and(&self, x: odra::types::U256, y: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_and(x, y);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_and(&self, x: odra::types::U256, y: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::BitwiseOps => {
                    return x & y;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_and(x, y)
            }
        }

        pub fn get_last_n_bits(
            &self,
            x: odra::types::U256,
            n: odra::types::U256,
        ) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get_last_n_bits(x, n);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_get_last_n_bits(
            &self,
            x: odra::types::U256,
            n: odra::types::U256,
        ) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::BitwiseOps => {
                    let mut mask = (1u8.into() << n - 1);
                    return x & mask;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_last_n_bits(x, n),
            }
        }
        pub fn get_last_n_bits_using_mod(
            &self,
            x: odra::types::U256,
            n: odra::types::U256,
        ) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get_last_n_bits_using_mod(x, n);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_get_last_n_bits_using_mod(
            &self,
            x: odra::types::U256,
            n: odra::types::U256,
        ) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::BitwiseOps => {
                    return ((x & 1u8.into()) << n);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_last_n_bits_using_mod(x, n),
            }
        }
        
        #[odra(init)]
        pub fn init(&mut self) {}

        pub fn not(&self, x: u8) -> u8 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_not(x);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_not(&self, x: u8) -> u8 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::BitwiseOps => {
                    return !x;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_not(x)
            }
        }

        pub fn or(&self, x: odra::types::U256, y: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_or(x, y);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_or(&self, x: odra::types::U256, y: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::BitwiseOps => {
                    return x | y;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_or(x, y)
            }
        }

        pub fn shift_left(&self, x: odra::types::U256, bits: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_shift_left(x, bits);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_shift_left(&self, x: odra::types::U256, bits: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::BitwiseOps => {
                    return x << bits;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_shift_left(x, bits)
            }
        }

        pub fn shift_right(&self, x: odra::types::U256, bits: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_shift_right(x, bits);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_shift_right(&self, x: odra::types::U256, bits: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::BitwiseOps => {
                    return x >> bits;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_shift_right(x, bits)
            }
        }

        pub fn xor(&self, x: odra::types::U256, y: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_xor(x, y);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_xor(&self, x: odra::types::U256, y: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::BitwiseOps => {
                    return x ^ y;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_xor(x, y)
            }
        }
    }
}