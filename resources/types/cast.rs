pub mod errors {}
pub mod events {
    use odra::prelude::*;
}
pub mod enums {}
pub mod structs {}
pub mod erc_20 {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
    {{DEFAULT_IMPORTS}}
    {{STACK_DEF}}
    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::ERC20],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum ClassName {
        ERC20,
    }
    
    #[odra::module]
    pub struct ERC20 {
        name: odra::Var<odra::prelude::string::String>,
        symbol: odra::Var<odra::prelude::string::String>,
        decimals: odra::Var<nysa_types::U8>,
        total_supply: odra::Var<nysa_types::U256>,
        balance_of: odra::Mapping<Option<odra::Address>, nysa_types::U256>
    }
    #[odra::module]
    impl ERC20 {
        pub fn init(
            &mut self, 
            _name: odra::prelude::string::String, 
            _symbol: odra::prelude::string::String, 
            _decimals: nysa_types::U8, 
            _initial_supply: nysa_types::U256
        ) {
            self.name.set(_name);
            self.symbol.set(_symbol);
            self.decimals.set(_decimals);
            self.total_supply
                .set(
                    (_initial_supply 
                        * nysa_types::U256::from_limbs_slice(&[10u64])
                            .pow(nysa_types::U256::from(*self.decimals.get_or_default())))
                );
            self.balance_of.set(
                &Some(self.env().caller()), 
                self.total_supply.get_or_default()
            );
        }
    }
}
