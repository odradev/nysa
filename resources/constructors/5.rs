{{DEFAULT_MODULES}}
pub mod e {
    #![allow(unused_braces, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
   
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        E, Y, X
    }

    #[odra::module] 
    pub struct E { 
        __stack: PathStack, 
        name: odra::Variable<odra::prelude::string::String>,
        text: odra::Variable<odra::prelude::string::String>
    } 

    #[odra::module] 
    impl E { 
        const PATH: &'static [ClassName; 3usize] = &[ClassName::X, ClassName::Y, ClassName::E];

        fn _x_init(&mut self) {
            self.name.set(odra::prelude::string::String::from("name"));
        }

        fn _y_init(&mut self) {
            self.text.set(odra::prelude::string::String::from("text"));
        } 

        #[odra(init)]
        pub fn init(&mut self) {
            self._x_init();
            self._y_init();
        }
    }
}