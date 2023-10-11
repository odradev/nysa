{{DEFAULT_MODULES}}

pub mod i_uniswap_v_3_pool {
    #[odra::external_contract]
    pub trait IUniswapV3Pool {
        fn deposit(
            &mut self,
            token_id: nysa_types::U256,
            liquidity: nysa_types::U128,
            amount_0_min: nysa_types::U256, 
            amount_1_min: nysa_types::U256
        );
    }
}

pub mod simple_uniswap_v_3_pool {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
    
    use super::i_uniswap_v_3_pool::*;
    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        SimpleUniswapV3Pool,
    }

    #[odra::module] 
    pub struct SimpleUniswapV3Pool { 
        __stack: PathStack,
        token_0: odra::Variable<Option<odra::types::Address>>, 
        token_1: odra::Variable<Option<odra::types::Address>>, 
        fee: odra::Variable<nysa_types::U24>, 
        pool: odra::Variable<Option<odra::types::Address>>
    } 

    #[odra::module] 
    impl SimpleUniswapV3Pool { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::SimpleUniswapV3Pool];

        pub fn deposit(
            &mut self,
            liquidity: nysa_types::U128,
            amount_0_min: nysa_types::U256,
            amount_1_min: nysa_types::U256
        ) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_deposit(liquidity, amount_0_min, amount_1_min);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_deposit(
            &mut self,
            liquidity: nysa_types::U128,
            amount_0_min: nysa_types::U256,
            amount_1_min: nysa_types::U256
        ) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::SimpleUniswapV3Pool => {
                    IUniswapV3PoolRef::at(&odra::UnwrapOrRevert::unwrap_or_revert(self.pool.get().unwrap_or(None)))
                        .deposit(
                            nysa_types::U256::ZERO, 
                            liquidity, 
                            amount_0_min, 
                            amount_1_min
                        );
                }
                #[allow(unreachable_patterns)]
                _ => self.super_deposit(liquidity, amount_0_min, amount_1_min)
            }
        }

        #[odra(init)]
        pub fn init(
            &mut self, 
            _token_0: Option<odra::types::Address>, 
            _token_1: Option<odra::types::Address>, 
            _fee: nysa_types::U24, 
            _pool: Option<odra::types::Address>
        ) {
            self.token_0.set(_token_0);
            self.token_1.set(_token_1);
            self.fee.set(_fee);
            self.pool.set(_pool);
        }
    }
}
