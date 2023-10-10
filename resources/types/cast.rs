pub mod errors {}
pub mod events {}
pub mod enums {}

pub mod erc_20 {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
    {{DEFAULT_IMPORTS}}
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        ERC20,
    }
    
    #[odra::module]
    pub struct ERC20 {
        __stack: PathStack,
        name: odra::Variable<odra::prelude::string::String>,
        symbol: odra::Variable<odra::prelude::string::String>,
        decimals: odra::Variable<nysa_types::U8>,
        total_supply: odra::Variable<nysa_types::U256>,
        balance_of: odra::Mapping<Option<odra::types::Address>, nysa_types::U256>
    }
    #[odra::module]
    impl ERC20 {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::ERC20];

        #[odra(init)]
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
                &Some(odra::contract_env::caller()), 
                self.total_supply.get_or_default()
            );
        }
    }
}
