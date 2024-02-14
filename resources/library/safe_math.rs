{{DEFAULT_MODULES}}
pub mod safe_math {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}
    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::SafeMath],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum ClassName {
        SafeMath,
    }
    #[odra::module]
    pub struct SafeMath {

    }
    #[odra::module]
    impl SafeMath {        
        pub(crate) fn add(x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            let mut z = Default::default();
            if !({
                z = (x + y);
                z
            } >= x)
            {
                self.env().revert(odra::ExecutionError::User(1u16))
            }
            return (z);
        }

        pub(crate) fn mul(x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            let mut z = Default::default();
            if !(y == nysa_types::U256::ZERO
                || ({
                    z = (x * y);
                    z
                } / y) == x)
            {
                self.env().revert(odra::ExecutionError::User(1u16))
            }
            return (z);
        }

        pub(crate) fn sub(x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            let mut z = Default::default();
            if !({
                z = (x - y);
                z
            } <= x)
            {
                self.env().revert(odra::ExecutionError::User(1u16))
            }
            return (z);
        }
    }
}