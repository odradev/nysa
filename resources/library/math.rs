{{DEFAULT_MODULES}}
pub mod math {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        Math,
    }
    #[odra::module]
    pub struct Math {
        __stack: PathStack,
    }
    #[odra::module]
    impl Math {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Math];
        
        pub(crate) fn min(x: nysa_types::U256, y: nysa_types::U256) -> nysa_types::U256 {
            let mut z = Default::default();
            z = if x < y {
                x
            } else {
                y
            };
            return (z);
        }

        pub(crate) fn sqrt(y: nysa_types::U256) -> nysa_types::U256 {
            let mut z = Default::default();
            if y > nysa_types::U256::from_limbs_slice(&[3u64]) {
                z = y;
                let mut x = ((y / nysa_types::U256::from_limbs_slice(&[2u64])) + nysa_types::U256::ONE);
                while x < z {
                    z = x;
                    x = (((y / x) + x) / nysa_types::U256::from_limbs_slice(&[2u64]));
                }
            } else if y != nysa_types::U256::ZERO {
                z = nysa_types::U256::ONE;
            }
            return (z);
        }
    }
}

