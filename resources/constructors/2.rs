pub mod errors {}
pub mod events {}
pub mod b {
    #![allow(unused_braces, non_snake_case)]

    use super::errors::*;
    use super::events::*;

    {{STACK_DEF}}
    
    #[derive(Clone)]
    enum ClassName {
        B, Y, X
    }

    #[odra::module] 
    pub struct B { 
        __stack: PathStack, 
        name: odra::Variable<odra::prelude::string::String>,
        text: odra::Variable<odra::prelude::string::String>
    } 

    #[odra::module] 
    impl B { 
        const PATH: &'static [ClassName; 3usize] = &[ClassName::X, ClassName::Y, ClassName::B];

        fn _x_init(&mut self, _name: odra::prelude::string::String) {
            self.name.set(_name);
        }

        fn _y_init(&mut self, _text: odra::prelude::string::String) {
            self.text.set(_text);
        } 
        
        #[odra(init)]
        pub fn init(&mut self) {
            self._x_init(odra::prelude::string::String::from("Input to X"));
            self._y_init(odra::prelude::string::String::from("Input to Y"));
        }
    }
}