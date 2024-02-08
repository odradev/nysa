{{DEFAULT_MODULES}}
pub mod safe_math {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        SafeMath,
    }
    #[odra::module]
    pub struct SafeMath {
        __stack: PathStack,
    }
    #[odra::module]
    impl SafeMath {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::SafeMath];
        
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