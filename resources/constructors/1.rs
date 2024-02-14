{{DEFAULT_MODULES}}
pub mod b {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}

    {{STACK_DEF}}

    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 3usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::X, ClassName::Y, ClassName::B],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        B, Y, X
    }

    #[odra::module] 
    pub struct B { 
        name: odra::Var<odra::prelude::string::String>,
        text: odra::Var<odra::prelude::string::String>
    } 

    #[odra::module] 
    impl B { 
        fn _x_init(&mut self, _name: odra::prelude::string::String) {
            self.name.set(_name);
        }

        fn _y_init(&mut self, _text: odra::prelude::string::String) {
            self._x_init(odra::prelude::string::String::from("Input to X"));
            self.text.set(_text);
        }

        pub fn init(&mut self) {
            self._y_init(odra::prelude::string::String::from("Input to Y"));
        }
    }
}