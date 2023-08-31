pub mod errors {}
pub mod events {}

pub mod i_uniswap_v3_pool {
    #[odra::external_contract]
    pub trait IUniswapV3Pool {
        fn deposit(token_id: odra::types::U256, liquidity: odra::types::U128, amount0_min: odra::types::U256, amount1_min: odra::types::U256);
    }
}

pub mod simple_uniswap_v3_pool {
    #![allow(unused_braces, non_snake_case)]
    
    use super::i_uniswap_v3_pool::*;
    use super::errors::*;
    use super::events::*;
    
    {{STACK_DEF}}
    
    #[derive(Clone)]
    enum ClassName {
        SimpleUniswapV3Pool,
    }

    #[odra::module] 
    pub struct SimpleUniswapV3Pool { 
        __stack: PathStack,
    } 

    #[odra::module] 
    impl SimpleUniswapV3Pool { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::SimpleUniswapV3Pool];
        // contract SimpleUniswapV3Pool {
        //     address public token0;
        //     address public token1;
        //     uint24 public fee;
        //     IUniswapV3Pool public pool;
        
        //     constructor(address _token0, address _token1, uint24 _fee, address _pool) {
        //         token0 = _token0;
        //         token1 = _token1;
        //         fee = _fee;
        //         pool = IUniswapV3Pool(_pool);
        //     }
        
        //     function deposit(uint128 liquidity, uint256 amount0Min, uint256 amount1Min) external {
        //         pool.deposit(0, liquidity, amount0Min, amount1Min);
        //     }
        // }

        #[odra(init)]
        pub fn init(&mut self) {
        }

        pub fn read_external_contract_value(&self, _addr: Option<odra::types::Address>) -> odra::types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_read_external_contract_value(_addr);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_read_external_contract_value(&self, _addr: Option<odra::types::Address>) -> odra::types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::SimpleUniswapV3Pool => {
                    let mut external_contract = ExternalContractRef::at(&odra::UnwrapOrRevert::unwrap_or_revert(_addr));
                    return external_contract.get_value()
                }
                #[allow(unreachable_patterns)]
                _ => self.super_read_external_contract_value(_addr)
            }
        }

        pub fn write_external_contract_value(&mut self, _addr: Option<odra::types::Address>, new_value: odra::types::U256) {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_write_external_contract_value(_addr, new_value);
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_write_external_contract_value(&mut self, _addr: Option<odra::types::Address>, new_value: odra::types::U256) {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::SimpleUniswapV3Pool => {
                    let mut external_contract = ExternalContractRef::at(&odra::UnwrapOrRevert::unwrap_or_revert(_addr));
                    external_contract.set_value(new_value);
                }
                #[allow(unreachable_patterns)]
                _ => self.super_write_external_contract_value(_addr, new_value)
            }
        }
    }
}
