pub mod errors {}
pub mod events {
    use odra::prelude::*;
}
pub mod enums {
    #[derive(odra::OdraType, PartialEq, Eq, Debug, Default)]
    pub enum Status {
        #[default]
        Pending = 0u8,
        Shipped = 1u8,
        Accepted = 2u8,
        Rejected = 3u8,
        Canceled = 4u8,
    }
}
pub mod structs {}
pub mod enum_test {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
    use super::enums::*;
    {{DEFAULT_IMPORTS}}
    {{STACK_DEF}}
    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::EnumTest],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum ClassName {
        EnumTest,
    }
    #[odra::module]
    pub struct EnumTest {
        status: odra::Var<Status>,
    }
    #[odra::module]
    impl EnumTest {
        pub fn cancel(&mut self) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_cancel();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_cancel(&mut self) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::EnumTest) => {
                    self.status.set(Status::Canceled);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_cancel(),
            }
        }

        pub fn get(&self) -> Status {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_get();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_get(&self) -> Status {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::EnumTest) => {
                    return self.status.get_or_default();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get(),
            }
        }

        pub fn init(&mut self) {}

        pub fn reset(&mut self) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_reset();
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_reset(&mut self) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::EnumTest) => {
                    self.status.set(Default::default());
                }
                #[allow(unreachable_patterns)]
                _ => self.super_reset(),
            }
        }

        pub fn set(&mut self, _status: Status) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_set(_status);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_set(&mut self, _status: Status) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::EnumTest) => {
                    self.status.set(_status);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_set(_status),
            }
        }
    }
}
