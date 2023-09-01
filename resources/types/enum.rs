pub mod errors {}
pub mod events {}
pub mod enums {
    #[derive(odra::OdraType, Copy, PartialEq, Eq, Debug, Default)]
    pub enum Status {
        #[default]
        Pending = 0u8,
        Shipped = 1u8,
        Accepted = 2u8,
        Rejected = 3u8,
        Canceled = 4u8,
    }
}
pub mod enum_test {
    #![allow(unused_braces, non_snake_case, unused_imports)]
    use super::enums::*;
    {{DEFAULT_IMPORTS}}
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        EnumTest,
    }
    #[odra::module]
    pub struct EnumTest {
        __stack: PathStack,
        status: odra::Variable<Status>,
    }
    #[odra::module]
    impl EnumTest {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::EnumTest];

        pub fn cancel(&mut self) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_cancel();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_cancel(&mut self) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::EnumTest => {
                    self.status.set(Status::Canceled);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_cancel(),
            }
        }

        pub fn get(&self) -> Status {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get(&self) -> Status {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::EnumTest => {
                    return self.status.get_or_default();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get(),
            }
        }

        #[odra(init)]
        pub fn init(&mut self) {}

        pub fn reset(&mut self) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_reset();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_reset(&mut self) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::EnumTest => {
                    self.status.set(Default::default());
                }
                #[allow(unreachable_patterns)]
                _ => self.super_reset(),
            }
        }

        pub fn set(&mut self, _status: Status) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_set(_status);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_set(&mut self, _status: Status) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::EnumTest => {
                    self.status.set(_status);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_set(_status),
            }
        }
    }
}
