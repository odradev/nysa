{{DEFAULT_MODULES}}
pub mod c {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 3usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::X, ClassName::Y, ClassName::C],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        C, Y, X
    }
    #[odra::module] 
    pub struct C { 
        name: odra::Var<odra::prelude::string::String>,
        text: odra::Var<odra::prelude::string::String>
    } 

    #[odra::module] 
    impl C { 
        fn _x_init(&mut self, _name: odra::prelude::string::String) {
            self.name.set(_name);
        }

        fn _y_init(&mut self, _text: odra::prelude::string::String) {
            self.text.set(_text);
        } 

        pub fn init(&mut self, _name: odra::prelude::string::String, _text: odra::prelude::string::String) {
            self._x_init(_name);
            self._y_init(_text);
        }
    }
}