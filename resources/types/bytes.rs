pub mod errors {}
pub mod events {}
pub mod enums {}

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
        a: odra::Variable<nysa_types::FixedBytes<1usize>>,
        b: odra::Variable<nysa_types::FixedBytes<2usize>>,
        c: odra::Variable<nysa_types::FixedBytes<4usize>>,
        d: odra::Variable<nysa_types::FixedBytes<4usize>>,
    }
    #[odra::module]
    impl A {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::A];

        #[odra(init)]
        pub fn init(&mut self) {
            self.a.set(nysa_types::FixedBytes([181u8]));
            self.b.set(nysa_types::FixedBytes([2u8, 255u8]));
            self.c.set(nysa_types::FixedBytes([0u8, 17u8, 34u8, 255u8]));
            self.d.set(nysa_types::FixedBytes([255u8, 0u8, 255u8, 0u8]));
        }
    }
}
