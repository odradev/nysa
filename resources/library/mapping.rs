pub mod errors {
    #[derive(odra::Error, PartialEq, Eq, Debug)]
    pub enum Error { CannotUpdateEmptyPosition = 0u16, }
}
pub mod events {
    use odra::prelude::*;
}
pub mod enums {}
pub mod structs {
    pub mod position {
        #[derive(odra::OdraType, PartialEq, Eq, Debug, Default)]
        pub struct Info {
            pub liquidity: nysa_types::U128,
            pub fee_growth_inside_0_last_x_128: nysa_types::U256,
            pub fee_growth_inside_1_last_x_128: nysa_types::U256,
        }
    }
    pub mod pool {
        #[derive(odra::OdraType, PartialEq, Eq, Debug, Default)]
        pub struct ModifyPositionParams {
            pub owner: Option<odra::Address>,
            pub tick_lower: nysa_types::I24,
            pub tick_upper: nysa_types::I24,
            pub liquidity_delta: nysa_types::I128,
            pub tick_spacing: nysa_types::I24,
        }
        #[derive(odra::OdraType, PartialEq, Eq, Debug, Default)]
        pub struct ModifyPositionState {
            pub flipped_lower: bool,
            pub liquidity_gross_after_lower: nysa_types::U128,
            pub flipped_upper: bool,
            pub liquidity_gross_after_upper: nysa_types::U128,
            pub fee_growth_inside_0_x_128: nysa_types::U256,
            pub fee_growth_inside_1_x_128: nysa_types::U256,
        }
        #[derive(odra::OdraType, PartialEq, Eq, Debug, Default)]
        pub struct State {
            pub fee_growth_global_0_x_128: nysa_types::U256,
            pub fee_growth_global_1_x_128: nysa_types::U256,
            pub liquidity: nysa_types::U128,
            pub tick_bitmap: odra::Mapping<nysa_types::I16, nysa_types::U256>,
            pub positions: odra::Mapping<nysa_types::FixedBytes<32usize>, super::position::Info>,
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

    {{DEFAULT_IMPORTS}}
    
    {{STACK_DEF}}
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
                *(fees_storage >> nysa_types::U24::from_limbs_slice(&[12u64])),
            );
        }
        pub(crate) fn get_withdraw_fee(
            fees_storage: nysa_types::U24,
        ) -> nysa_types::U16 {
            return nysa_types::U16::from(
                *(fees_storage & nysa_types::U24::from_limbs_slice(&[4095u64])),
            );
        }
        pub(crate) fn modify_position(
            _self: pool::State,
            params: pool::ModifyPositionParams,
        ) {
            let mut fees_owed_0 = Default::default();
            let mut fees_owed_1 = Default::default();
            {
                let mut state: pool::ModifyPositionState = Default::default();
                {
                    let (_0, _1) = super::position::Position::update(
                        super::position::Position::get(
                            _self.positions,
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
            _self: odra::Mapping<nysa_types::FixedBytes<32usize>, position::Info>,
            owner: Option<odra::Address>,
            tick_lower: nysa_types::I24,
            tick_upper: nysa_types::I24,
        ) -> position::Info {
            let mut position = Default::default();
            position = odra::UnwrapOrRevert::unwrap_or_revert(_self.get(
                &nysa_types::FixedBytes::try_from(self.env().hash({
                    let mut result = Vec::new();
                    result.extend(odra::UnwrapOrRevert::unwrap_or_revert(odra::casper_types::bytesrepr::ToBytes::to_bytes(&owner), &self.env()));
                    result.extend(odra::UnwrapOrRevert::unwrap_or_revert(odra::casper_types::bytesrepr::ToBytes::to_bytes(&tick_lower), &self.env()));
                    result.extend(odra::UnwrapOrRevert::unwrap_or_revert(odra::casper_types::bytesrepr::ToBytes::to_bytes(&tick_upper), &self.env()));
                    result
                }).as_slice()).unwrap_or_default()
            ), &self.env());
            return (position);
        }
        pub(crate) fn update(
            _self: position::Info,
            liquidity_delta: nysa_types::I128,
            fee_growth_inside_0_x_128: nysa_types::U256,
            fee_growth_inside_1_x_128: nysa_types::U256,
        ) -> (nysa_types::U256, nysa_types::U256) {
            let mut fees_owed_0 = Default::default();
            let mut fees_owed_1 = Default::default();
            let mut _self = _self;
            let mut liquidity_next = Default::default();
            if liquidity_delta == nysa_types::I128::ZERO {
                if _self.liquidity == nysa_types::U128::ZERO {
                    self.env().revert(Error::CannotUpdateEmptyPosition);
                }
                liquidity_next = _self.liquidity;
            } else {
                liquidity_next = if liquidity_delta < nysa_types::I128::ZERO {
                    (_self.liquidity - nysa_types::U128::from(*(-liquidity_delta)))
                } else {
                    (_self.liquidity + nysa_types::U128::from(*liquidity_delta))
                };
            }
            {
                fees_owed_0 = super::full_math::FullMath::mul_div(
                    (fee_growth_inside_0_x_128 - _self.fee_growth_inside_0_last_x_128),
                    (_self.liquidity).cast(),
                    super::fixed_point_128::FixedPoint128::Q128,
                );
                fees_owed_1 = super::full_math::FullMath::mul_div(
                    (fee_growth_inside_1_x_128 - _self.fee_growth_inside_1_last_x_128),
                    (_self.liquidity).cast(),
                    super::fixed_point_128::FixedPoint128::Q128,
                );
            }
            if liquidity_delta != nysa_types::I128::ZERO {
                _self.liquidity = liquidity_next;
            }
            _self.fee_growth_inside_0_last_x_128 = fee_growth_inside_0_x_128;
            _self.fee_growth_inside_1_last_x_128 = fee_growth_inside_1_x_128;
            return (fees_owed_0, fees_owed_1);
        }
    }
}