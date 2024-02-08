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

    #[derive(Clone)]
    enum ClassName {
        A,
    }
    #[odra::module]
    pub struct A {
        __stack: PathStack,
        a: odra::Var<nysa_types::U256>,
        b: odra::Var<nysa_types::U256>,
        map: odra::Mapping<nysa_types::U256, bool>,
    }

    #[odra::module]
    impl A {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::A];

        fn f(&self) -> (nysa_types::U256, nysa_types::U256) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_f();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_f(&self) -> (nysa_types::U256, nysa_types::U256) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::A => {
                    return (nysa_types::U256::ZERO, nysa_types::U256::ONE);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_f(),
            }
        }

        pub fn get(&mut self) -> nysa_types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get(&mut self) -> nysa_types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::A => {
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

        #[odra(init)]
        pub fn init(&mut self) {
        }
    }
}
