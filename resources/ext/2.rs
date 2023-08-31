pub mod errors {}
pub mod events {}
pub mod caller {
    #![allow(unused_braces, non_snake_case)]

    use super::callee::*;
    use super::errors::*;
    use super::events::*;
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        Caller,
    }

    #[odra::module] 
    pub struct Caller { 
        __stack: PathStack, 
    } 

    #[odra::module] 
    impl Caller { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Caller];

        #[odra(init)]
        pub fn init(&mut self) {
        }

        pub fn set_x(&mut self, _callee: Option<odra::types::Address>, _x: odra::types::U256) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_set_x(_callee, _x);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_set_x(&mut self, _callee: Option<odra::types::Address>, _x: odra::types::U256) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Caller => {
                    let mut _callee = CalleeRef::at(&odra::UnwrapOrRevert::unwrap_or_revert(_callee));
                    let x = _callee.set_x(_x);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_set_x(_callee, _x)
            }
        }

        pub fn set_x_from_address(&mut self, _addr: Option<odra::types::Address>, _x: odra::types::U256) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_set_x_from_address(_addr, _x);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_set_x_from_address(&mut self, _addr: Option<odra::types::Address>, _x: odra::types::U256) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Caller => {
                    let mut callee = CalleeRef::at(&odra::UnwrapOrRevert::unwrap_or_revert(_addr));
                    callee.set_x(_x);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_set_x_from_address(_addr, _x)
            }
        }  
    }
}

pub mod callee {
    #![allow(unused_braces, non_snake_case)]

    use super::errors::*;
    use super::events::*;
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        Callee,
    }

    #[odra::module] 
    pub struct Callee { 
        __stack: PathStack,
        x: odra::Variable<odra::types::U256>,
    } 

    #[odra::module] 
    impl Callee { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Callee];

        #[odra(init)]
        pub fn init(&mut self) {
        }

        pub fn set_x(&mut self, _x: odra::types::U256) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_set_x(_x);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_set_x(&mut self, _x: odra::types::U256) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::Callee => {
                    self.x.set(_x);
                    return self.x.get_or_default()
                }
                #[allow(unreachable_patterns)]
                _ => self.super_set_x(_x)
            }
        }
    }
}