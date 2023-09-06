pub mod errors {}
pub mod events {}
pub mod enums {}

pub mod a {
    #![allow(unused_braces, non_snake_case, unused_imports)]
    {{DEFAULT_IMPORTS}}
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        A,
    }
    #[odra::module]
    pub struct A {
        __stack: PathStack,
        a: odra::Variable<[u8; 1usize]>,
        b: odra::Variable<[u8; 2usize]>,
        c: odra::Variable<[u8; 4usize]>,
        d: odra::Variable<[u8; 4usize]>,
    }
    #[odra::module]
    impl A {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::A];

        #[odra(init)]
        pub fn init(&mut self) {
            self.a.set([181u8]);
            self.b.set([2u8, 255u8]);
            self.c.set([0u8, 17u8, 34u8, 255u8]);
            self.d.set([255u8, 0u8, 255u8, 0u8]);
        }
    }
}
