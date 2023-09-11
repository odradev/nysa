{{DEFAULT_MODULES}}
pub mod math {
    #![allow(unused_braces, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        Math,
    }
    #[odra::module]
    pub struct Math {
        __stack: PathStack,
    }
    #[odra::module]
    impl Math {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Math];
        #[odra(init)]
        pub fn init(&mut self) {}
        fn min(&self, x: odra::types::U256, y: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_min(x, y);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_min(
            &self,
            x: odra::types::U256,
            y: odra::types::U256,
        ) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Math => {
                    let mut z = Default::default();
                    z = if x < y {
                        x
                    } else {
                        y
                    };
                    return (z);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_min(x, y),
            }
        }

        fn sqrt(&self, y: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_sqrt(y);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_sqrt(&self, y: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Math => {
                    let mut z = Default::default();
                    if y > 3u8.into() {
                        z = y;
                        let mut x = ((y / 2) + 1);
                        while x < z {
                            z = x;
                            x = (((y / x) + x) / 2);
                        }
                    } else if y != 0u8.into() {
                        z = 1u8.into()
                    }
                    return (z);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_sqrt(y),
            }
        }
    }
}