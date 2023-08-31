pub mod errors {}
pub mod events {}
pub mod my_contract {
    #![allow(unused_braces, non_snake_case)]

    use super::errors::*;
    use super::events::*;
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        MyContract
    }

    #[odra::module] 
    pub struct MyContract { 
        __stack: PathStack, 
        my_number: odra::Variable<odra::types::U256>,
        min_int: odra::Variable<i16>,
        boo: odra::Variable<bool>
    } 

    #[odra::module] 
    impl MyContract { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::MyContract];

        pub fn get_my_number(&self) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get_my_number();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get_my_number(&self) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::MyContract => {
                    return self.my_number.get_or_default();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_my_number(),
            }
        }

        #[odra(init)]
        pub fn init(&mut self) {
            self.my_number.set(42u8.into());
            self.min_int.set(i16::MIN);
            self.boo.set(true);
        }
    }
}