pub mod errors {}
pub mod events {}
pub mod d {
    #![allow(unused_braces, non_snake_case)]

    use super::errors::*;
    use super::events::*;
    
    {{STACK_DEF}}
    
    #[derive(Clone)]
    enum ClassName {
        D, Y, X
    }
    #[odra::module] 
    pub struct D { 
        __stack: PathStack, 
        name: odra::Variable<odra::prelude::string::String>,
        text: odra::Variable<odra::prelude::string::String>
    } 

    #[odra::module] 
    impl D { 
        const PATH: &'static [ClassName; 3usize] = &[ClassName::X, ClassName::Y, ClassName::D];

        fn _x_init(&mut self, _name: odra::prelude::string::String) {
            self.name.set(_name);
        }

        fn _y_init(&mut self, _text: odra::prelude::string::String) {
            self.text.set(_text);
        } 

        #[odra(init)]
        pub fn init(&mut self) {
            self._x_init(odra::prelude::string::String::from("Input to XXX"));
            self._y_init(odra::prelude::string::String::from("Input to Y"));
        }
    }
}