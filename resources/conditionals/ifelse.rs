{{DEFAULT_MODULES}}
pub mod if_else {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}

    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        IfElse,
    }
    #[odra::module]
    pub struct IfElse {
        __stack: PathStack,
    }
    #[odra::module]
    impl IfElse {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::IfElse];
        pub fn foo(&self, x: nysa_types::U256) -> nysa_types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_foo(x);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_foo(&self, x: nysa_types::U256) -> nysa_types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::IfElse => {
                    if x < nysa_types::U256::from_limbs_slice(&[10u64]) {
                        return nysa_types::U256::from_limbs_slice(&[]);
                    } else if x < nysa_types::U256::from_limbs_slice(&[20u64]) {
                        return nysa_types::U256::from_limbs_slice(&[1u64]);
                    } else {
                        return nysa_types::U256::from_limbs_slice(&[2u64]);
                    }
                }
                #[allow(unreachable_patterns)]
                _ => self.super_foo(x),
            }
        }
        #[odra(init)]
        pub fn init(&mut self) {}
        pub fn ternary(&self, _x: nysa_types::U24) -> nysa_types::U32 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_ternary(_x);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_ternary(&self, _x: nysa_types::U24) -> nysa_types::U32 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::IfElse => {
                    return if _x < nysa_types::U24::from_limbs_slice(&[10u64]) { 
                        nysa_types::U32::from_limbs_slice(&[1u64]) 
                    } else { 
                        nysa_types::U32::from_limbs_slice(&[2u64]) 
                    };
                }
                #[allow(unreachable_patterns)]
                _ => self.super_ternary(_x),
            }
        }
    }
}
