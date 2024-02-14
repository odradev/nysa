{{DEFAULT_MODULES}}

pub mod i_uniswap_v_3_pool {
    #![allow(unused_imports)]
    use odra::prelude::*;
    
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

    const MAX_STACK_SIZE: usize = 8; // Maximum number of paths in the stack
    const MAX_PATH_LENGTH: usize = 1usize; // Maximum length of each path
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::SimpleUniswapV3Pool],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ClassName {
        SimpleUniswapV3Pool,
    }

    #[odra::module] 
    pub struct SimpleUniswapV3Pool { 
        token_0: odra::Var<Option<odra::Address>>, 
        token_1: odra::Var<Option<odra::Address>>, 
        fee: odra::Var<nysa_types::U24>, 
        pool: odra::Var<Option<odra::Address>>
    } 

    #[odra::module] 
    impl SimpleUniswapV3Pool { 
        pub fn deposit(
            &mut self,
            liquidity: nysa_types::U128,
            amount_0_min: nysa_types::U256,
            amount_1_min: nysa_types::U256
        ) {
            unsafe { STACK.push_path_on_stack(); }
            let result = self.super_deposit(liquidity, amount_0_min, amount_1_min);
            unsafe { STACK.drop_one_from_stack(); }
            result
        }

        fn super_deposit(
            &mut self,
            liquidity: nysa_types::U128,
            amount_0_min: nysa_types::U256,
            amount_1_min: nysa_types::U256
        ) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::SimpleUniswapV3Pool) => {
                    IUniswapV3PoolContractRef::new(self.env(), odra::UnwrapOrRevert::unwrap_or_revert(self.pool.get().unwrap_or(None), &self.env()))
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

        pub fn init(
            &mut self, 
            _token_0: Option<odra::Address>, 
            _token_1: Option<odra::Address>, 
            _fee: nysa_types::U24, 
            _pool: Option<odra::Address>
        ) {
            self.token_0.set(_token_0);
            self.token_1.set(_token_1);
            self.fee.set(_fee);
            self.pool.set(_pool);
        }
    }
}
