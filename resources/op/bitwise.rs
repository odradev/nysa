{{DEFAULT_MODULES}}
pub mod bitwise_ops {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}
    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::BitwiseOps],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum ClassName {
        BitwiseOps,
    }

    #[odra::module] 
    pub struct BitwiseOps { 
    } 

    #[odra::module] 
    impl BitwiseOps { 
        pub fn and(&self, x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_and(x, y);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_and(&self, x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    return x & y;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_and(x, y)
            }
        }

        pub fn get_first_n_bits(
            &self,
            x: nysa_types::U256,
            n: nysa_types::U256,
            len: nysa_types::U256,
        ) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get_first_n_bits(x, n, len);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get_first_n_bits(
            &self,
            x: nysa_types::U256,
            n: nysa_types::U256,
            len: nysa_types::U256,
        ) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    let mut mask = (nysa_types::U256::ONE
                        << n - nysa_types::U256::ONE) << (len - n);
                    return x & mask;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_first_n_bits(x, n, len),
            }
        }

        pub fn get_last_n_bits(
            &self,
            x: nysa_types::U256,
            n: nysa_types::U256,
        ) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get_last_n_bits(x, n);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get_last_n_bits(
            &self,
            x: nysa_types::U256,
            n: nysa_types::U256,
        ) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    let mut mask = (nysa_types::U256::ONE
                        << n - nysa_types::U256::ONE);
                    return x & mask;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_last_n_bits(x, n),
            }
        }
        pub fn get_last_n_bits_using_mod(
            &self,
            x: nysa_types::U256,
            n: nysa_types::U256,
        ) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get_last_n_bits_using_mod(x, n);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }
        fn super_get_last_n_bits_using_mod(
            &self,
            x: nysa_types::U256,
            n: nysa_types::U256,
        ) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    return (x % nysa_types::U256::ONE << n);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_last_n_bits_using_mod(x, n),
            }
        }
        
        pub fn init(&mut self) {}

        pub fn most_significant_bit(&self, x: nysa_types::U256) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_most_significant_bit(x);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_most_significant_bit(&self, x: nysa_types::U256) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    let mut i = nysa_types::U256::ZERO;
                    while {
                        x = x >> nysa_types::U256::ONE;
                        x
                    } > nysa_types::U256::ZERO
                    {
                        i += nysa_types::Unsigned::ONE;
                    }
                    return i;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_most_significant_bit(x),
            }
        }

        pub fn not(&self, x: nysa_types::U8) -> nysa_types::U8 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_not(x);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_not(&self, x: nysa_types::U8) -> nysa_types::U8 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    return !x;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_not(x)
            }
        }

        pub fn or(&self, x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_or(x, y);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_or(&self, x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    return x | y;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_or(x, y)
            }
        }

        pub fn shift_left(&self, x: nysa_types::U256, bits: nysa_types::U256) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_shift_left(x, bits);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_shift_left(&self, x: nysa_types::U256, bits: nysa_types::U256) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    return x << bits;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_shift_left(x, bits)
            }
        }

        pub fn shift_right(&self, x: nysa_types::U256, bits: nysa_types::U256) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_shift_right(x, bits);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_shift_right(&self, x: nysa_types::U256, bits: nysa_types::U256) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    return x >> bits;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_shift_right(x, bits)
            }
        }

        pub fn xor(&self, x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_xor(x, y);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_xor(&self, x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::BitwiseOps) => {
                    return x ^ y;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_xor(x, y)
            }
        }
    }
}