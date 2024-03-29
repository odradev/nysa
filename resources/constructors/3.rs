{{DEFAULT_MODULES}}
pub mod c {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        C, Y, X
    }
    #[odra::module] 
    pub struct C { 
        __stack: PathStack, 
        name: odra::Var<odra::prelude::string::String>,
        text: odra::Var<odra::prelude::string::String>
    } 

    #[odra::module] 
    impl C { 
        const PATH: &'static [ClassName; 3usize] = &[ClassName::X, ClassName::Y, ClassName::C];

        fn _x_init(&mut self, _name: odra::prelude::string::String) {
            self.name.set(_name);
        }

        fn _y_init(&mut self, _text: odra::prelude::string::String) {
            self.text.set(_text);
        } 

        #[odra(init)]
        pub fn init(&mut self, _name: odra::prelude::string::String, _text: odra::prelude::string::String) {
            self._x_init(_name);
            self._y_init(_text);
        }
    }
}