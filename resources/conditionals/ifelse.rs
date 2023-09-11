{{DEFAULT_MODULES}}
pub mod if_else {
    #![allow(unused_braces, non_snake_case, unused_imports)]

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
        pub fn foo(&self, x: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_foo(x);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_foo(&self, x: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::IfElse => {
                    if x < 10u8.into() {
                        return 0u8.into();
                    } else if x < 20u8.into() {
                        return 1u8.into();
                    } else {
                        return 2u8.into();
                    }
                }
                #[allow(unreachable_patterns)]
                _ => self.super_foo(x),
            }
        }
        #[odra(init)]
        pub fn init(&mut self) {}
        pub fn ternary(&self, _x: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_ternary(_x);
            self.__stack.drop_one_from_stack();
            result
        }
        fn super_ternary(&self, _x: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::IfElse => {
                    return if _x < 10u8.into() { 1u8.into() } else { 2u8.into() };
                }
                #[allow(unreachable_patterns)]
                _ => self.super_ternary(_x),
            }
        }
    }
}
