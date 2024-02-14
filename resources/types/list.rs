pub mod errors {}
pub mod events {
    use odra::prelude::*;
}
pub mod enums {}
pub mod structs {}
pub mod a {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
    {{DEFAULT_IMPORTS}}
    {{STACK_DEF}}
    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::A],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum ClassName {
        A,
    }
    #[odra::module]
    pub struct A {
        a: odra::Var<nysa_types::U256>,
        b: odra::Var<nysa_types::U256>,
        map: odra::Mapping<nysa_types::U256, bool>,
    }

    #[odra::module]
    impl A {
        fn f(&self) -> (nysa_types::U256, nysa_types::U256) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_f();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_f(&self) -> (nysa_types::U256, nysa_types::U256) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::A) => {
                    return (nysa_types::U256::ZERO, nysa_types::U256::ONE);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_f(),
            }
        }

        pub fn get(&mut self) -> nysa_types::U256 {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get(&mut self) -> nysa_types::U256 {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::A) => {
                    {
                        self.a.set(nysa_types::U256::ONE);
                        self.b.set(nysa_types::U256::ONE);
                    };
                    {
                        self.a.set(nysa_types::U256::ONE);
                        self.map.set(&nysa_types::U256::ZERO, false);
                    };
                    let (mut x, mut y) = (nysa_types::U256::ONE, nysa_types::U256::ZERO);
                    {
                        x = nysa_types::U256::ZERO;
                        y = nysa_types::U256::ONE;
                    };
                    {
                        let _ = nysa_types::U256::ONE;
                        y = nysa_types::U256::ONE;
                    };
                    {
                        self.a.set(nysa_types::U256::ZERO);
                        x = y;
                    };
                    {
                        let (_0, _1) = self.f();
                        x = _0;
                        y = _1;
                    };
                    {
                        x = nysa_types::U256::ONE;
                        y = nysa_types::U256::ZERO;
                        self.a.set(nysa_types::U256::ONE);
                    };
                    return x;
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get(),
            }
        }

        pub fn init(&mut self) {
        }
    }
}
