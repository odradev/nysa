pub mod errors {
    odra::execution_error! {
        pub enum Error { CannotUpdateEmptyPosition => 0u16, }
    }
}
pub mod events {}
pub mod enums {}
pub mod structs {
    pub mod position {
        #[derive(odra::OdraType, Copy, PartialEq, Eq, Debug, Default)]
        pub struct Info {
            liquidity: nysa_types::U128,
            fee_growth_inside_0_last_x_128: nysa_types::U256,
            fee_growth_inside_1_last_x_128: nysa_types::U256,
        }
    }
    pub mod pool {
        #[derive(odra::OdraType, Copy, PartialEq, Eq, Debug, Default)]
        pub struct ModifyPositionParams {
            owner: Option<odra::types::Address>,
            tick_lower: nysa_types::I24,
            tick_upper: nysa_types::I24,
            liquidity_delta: nysa_types::I128,
            tick_spacing: nysa_types::I24,
        }
        #[derive(odra::OdraType, Copy, PartialEq, Eq, Debug, Default)]
        pub struct ModifyPositionState {
            flipped_lower: bool,
            liquidity_gross_after_lower: nysa_types::U128,
            flipped_upper: bool,
            liquidity_gross_after_upper: nysa_types::U128,
            fee_growth_inside_0_x_128: nysa_types::U256,
            fee_growth_inside_1_x_128: nysa_types::U256,
        }
        #[derive(odra::OdraType, Copy, PartialEq, Eq, Debug, Default)]
        pub struct State {
            fee_growth_global_0_x_128: nysa_types::U256,
            fee_growth_global_1_x_128: nysa_types::U256,
            liquidity: nysa_types::U128,
            tick_bitmap: odra::Mapping<nysa_types::I16, nysa_types::U256>,
            positions: odra::Mapping<nysa_types::FixedBytes<32usize>, position::Info>,
        }
    }
}

pub mod fixed_point_128 {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}
    #[derive(Clone)]
    enum ClassName {
        FixedPoint128,
    }
    #[odra::module]
    pub struct FixedPoint128 {
        __stack: PathStack,
    }
    #[odra::module]
    impl FixedPoint128 {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::FixedPoint128];
        pub const Q128: nysa_types::U256 = nysa_types::U256::from_limbs([
            0u64,
            0u64,
            1u64,
            0u64,
        ]);
    }
}

pub mod full_math {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        FullMath,
    }
    #[odra::module]
    pub struct FullMath {
        __stack: PathStack,
    }
    #[odra::module]
    impl FullMath {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::FullMath];

        pub(crate) fn mul_div(a: nysa_types::U256, b: nysa_types::U256, denominator: nysa_types::U256) -> nysa_types::U256 {
            let mut result = Default::default();
            return (result);
        }

        pub(crate) fn mul_div_rounding_up(a: nysa_types::U256, b: nysa_types::U256, denominator: nysa_types::U256) -> nysa_types::U256 {
            let mut result = Default::default();
            return (result);
        }
    }
}

pub mod pool {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
    use super::errors::*;
    use super::events::*;
    use super::structs::*;
    use odra::prelude::vec::Vec;
    #[cfg(not(target_arch = "wasm32"))]
    impl odra::types::contract_def::Node for PathStack {
        const COUNT: u32 = 0;
        const IS_LEAF: bool = false;
    }
    impl odra::types::OdraItem for PathStack {
        fn is_module() -> bool {
            false
        }
    }
    impl odra::StaticInstance for PathStack {
        fn instance<'a>(keys: &'a [&'a str]) -> (Self, &'a [&'a str]) {
            (PathStack::default(), keys)
        }
    }
    impl odra::DynamicInstance for PathStack {
        #[allow(unused_variables)]
        fn instance(namespace: &[u8]) -> Self {
            PathStack::default()
        }
    }
    #[derive(Clone, Default)]
    struct PathStack {
        stack: alloc::rc::Rc<core::cell::RefCell<Vec<Vec<ClassName>>>>,
    }
    impl PathStack {
        pub fn push_path_on_stack(&self, path: &[ClassName]) {
            let mut stack = self.stack.take();
            stack.push(path.to_vec());
            self.stack.replace(stack);
        }
        pub fn drop_one_from_stack(&self) {
            let mut stack = self.stack.take();
            stack.pop().unwrap();
            self.stack.replace(stack);
        }
        pub fn pop_from_top_path(&self) -> ClassName {
            let mut stack = self.stack.take();
            let mut path = stack.pop().unwrap();
            let class = path.pop().unwrap();
            stack.push(path);
            self.stack.replace(stack);
            class
        }
    }
    #[derive(Clone)]
    enum ClassName {
        Pool,
    }
    #[odra::module]
    pub struct Pool {
        __stack: PathStack,
    }
    #[odra::module]
    impl Pool {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Pool];
        pub(crate) fn get_swap_fee(fees_storage: nysa_types::U24) -> nysa_types::U16 {
            return nysa_types::U16::from(
                *fees_storage >> nysa_types::U24::from_limbs_slice(&[12u64]),
            );
        }
        pub(crate) fn get_withdraw_fee(
            fees_storage: nysa_types::U24,
        ) -> nysa_types::U16 {
            return nysa_types::U16::from(
                *fees_storage & nysa_types::FixedBytes([15u8, 255u8]),
            );
        }
        pub(crate) fn modify_position(
            self: pool::State,
            params: pool::ModifyPositionParams,
        ) {
            let mut fees_owed_0 = Default::default();
            let mut fees_owed_1 = Default::default();
            {
                let mut state = Default::default();
                {
                    let (_0, _1) = Position::update(
                        Position::get(
                            self.positions,
                            params.owner,
                            params.tick_lower,
                            params.tick_upper,
                        ),
                        params.liquidity_delta,
                        state.fee_growth_inside_0_x_128,
                        state.fee_growth_inside_1_x_128,
                    );
                    fees_owed_0 = _0;
                    fees_owed_1 = _1;
                };
            }
        }
    }
}

pub mod position {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        Position,
    }
    #[odra::module]
    pub struct Position {
        __stack: PathStack,
    }
    #[odra::module]
    impl Position {
        const PATH: &'static [ClassName; 1usize] = &[ClassName::Position];
        pub(crate) fn get(
            self: odra::Mapping<nysa_types::FixedBytes<32usize>, position::Info>,
            owner: Option<odra::types::Address>,
            tick_lower: nysa_types::I24,
            tick_upper: nysa_types::I24,
        ) -> position::Info {
            let mut position = Default::default();
            position = self.get(&{
                let mut result = Vec::new();
                result.extend(odra::UnwrapOrRevert::unwrap_or_revert(odra::types::casper_types::bytesrepr::ToBytes::to_bytes(&owner)));
                result.extend(odra::UnwrapOrRevert::unwrap_or_revert(odra::types::casper_types::bytesrepr::ToBytes::to_bytes(&tick_lower)));
                result.extend(odra::UnwrapOrRevert::unwrap_or_revert(odra::types::casper_types::bytesrepr::ToBytes::to_bytes(&tick_upper)));
                nysa_types::FixedBytes::try_from(&odra::contract_env::hash(result)).unwrap_or_default()
            });
            return (position);
        }
        pub(crate) fn update(
            self: position::Info,
            liquidity_delta: nysa_types::I128,
            fee_growth_inside_0_x_128: nysa_types::U256,
            fee_growth_inside_1_x_128: nysa_types::U256,
        ) -> (nysa_types::U256, nysa_types::U256) {
            let mut fees_owed_0 = Default::default();
            let mut fees_owed_1 = Default::default();
            let mut _self = self;
            let mut liquidity_next = Default::default();
            if liquidity_delta == nysa_types::I128::ZERO {
                if _self.liquidity == nysa_types::U256::ZERO {
                    odra::contract_env::revert(Error::CannotUpdateEmptyPosition);
                }
                liquidity_next = _self.liquidity;
            } else {
                liquidity_next = if liquidity_delta < nysa_types::I128::ZERO {
                    (_self.liquidity - nysa_types::U128::from(*-liquidity_delta))
                } else {
                    (_self.liquidity + nysa_types::U128::from(*liquidity_delta))
                };
            }
            {
                fees_owed_0 = FullMath::mul_div(
                    (fee_growth_inside_0_x_128 - _self.fee_growth_inside_0_last_x_128),
                    (_self.liquidity).cast(),
                    FixedPoint128::Q128,
                );
                fees_owed_1 = FullMath::mul_div(
                    (fee_growth_inside_1_x_128 - _self.fee_growth_inside_1_last_x_128),
                    (_self.liquidity).cast(),
                    FixedPoint128::Q128,
                );
            }
            if liquidity_delta != nysa_types::I128::ZERO {
                self.liquidity = liquidity_next;
            }
            self.fee_growth_inside_0_last_x_128 = fee_growth_inside_0_x_128;
            self.fee_growth_inside_1_last_x_128 = fee_growth_inside_1_x_128;
            return (fees_owed_0, fees_owed_1);
        }
    }
}
