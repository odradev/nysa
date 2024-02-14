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
        a: odra::Var<nysa_types::FixedBytes<1usize>>,
        b: odra::Var<nysa_types::FixedBytes<2usize>>,
        c: odra::Var<nysa_types::FixedBytes<4usize>>,
        d: odra::Var<nysa_types::FixedBytes<4usize>>,
    }
    #[odra::module]
    impl A {
        pub fn init(&mut self) {
            self.a.set(nysa_types::FixedBytes([181u8]));
            self.b.set(nysa_types::FixedBytes([2u8, 255u8]));
            self.c.set(nysa_types::FixedBytes([0u8, 17u8, 34u8, 255u8]));
            self.d.set(nysa_types::FixedBytes([255u8, 0u8, 255u8, 0u8]));
        }
    }
}
