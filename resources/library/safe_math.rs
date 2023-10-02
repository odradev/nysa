{{DEFAULT_MODULES}}
pub mod safe_math {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        SafeMath,
    }
    #[odra::module]
    pub struct SafeMath {
        __stack: PathStack,
    }
    #[odra::module]
    impl SafeMath {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::SafeMath];
        fn add(&self, x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_add(x, y);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_add(
            &self,
            x: nysa_types::U256,
            y: nysa_types::U256,
        ) -> nysa_types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::SafeMath => {
                    let mut z = Default::default();
                    if !({
                        z = (x + y);
                        z
                    } >= x)
                    {
                        odra::contract_env::revert(
                            odra::types::ExecutionError::new(
                                1u16,
                                "ds-math-add-overflow",
                            ),
                        )
                    }
                    return (z);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_add(x, y),
            }
        }
        #[odra(init)]
        pub fn init(&mut self) {}
        fn mul(&self, x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_mul(x, y);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_mul(
            &self,
            x: nysa_types::U256,
            y: nysa_types::U256,
        ) -> nysa_types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::SafeMath => {
                    let mut z = Default::default();
                    if !(y == nysa_types::U256::from_limbs_slice(&[])
                        || ({
                            z = (x * y);
                            z
                        } / y) == x)
                    {
                        odra::contract_env::revert(
                            odra::types::ExecutionError::new(
                                1u16,
                                "ds-math-mul-overflow",
                            ),
                        )
                    }
                    return (z);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_mul(x, y),
            }
        }
        fn sub(&self, x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_sub(x, y);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_sub(
            &self,
            x: nysa_types::U256,
            y: nysa_types::U256,
        ) -> nysa_types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::SafeMath => {
                    let mut z = Default::default();
                    if !({
                        z = (x - y);
                        z
                    } <= x)
                    {
                        odra::contract_env::revert(
                            odra::types::ExecutionError::new(
                                1u16,
                                "ds-math-sub-underflow",
                            ),
                        )
                    }
                    return (z);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_sub(x, y),
            }
        }
    }
}