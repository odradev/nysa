{{DEFAULT_MODULES}}
pub mod e {
    #![allow(unused_braces, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
   
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        E, Y, Z, X
    }

    #[odra::module] 
    pub struct E { 
        __stack: PathStack, 
        name: odra::Variable<odra::prelude::string::String>,
        text: odra::Variable<odra::prelude::string::String>
    } 

    #[odra::module] 
    impl E { 
        const PATH: &'static [ClassName; 4usize] = &[ClassName::X, ClassName::Z, ClassName::Y, ClassName::E];

        fn _x_init(&mut self, _name: odra::prelude::string::String) {
            self.name.set(_name);
        }

        fn _z_init(&mut self) {
        } 

        fn _y_init(&mut self, _text: odra::prelude::string::String) {
            self.text.set(_text);
        }

        #[odra(init)]
        pub fn init(&mut self) {
            self._x_init(odra::prelude::string::String::from("X was called"));
            self._z_init();
            self._y_init(odra::prelude::string::String::from("Y was called"));
        }
    }
}