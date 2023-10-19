// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.19;

library FullMath {
    function mulDiv(uint256 a, uint256 b, uint256 denominator) internal pure returns (uint256 result) {
        
    }

    function mulDivRoundingUp(uint256 a, uint256 b, uint256 denominator) internal pure returns (uint256 result) {
    
    }
}

library FixedPoint128 {
    uint256 internal constant Q128 = 0x100000000000000000000000000000000;
}

library Position {
    error CannotUpdateEmptyPosition();

    struct Info {
        uint128 liquidity;
        uint256 feeGrowthInside0LastX128;
        uint256 feeGrowthInside1LastX128;
    }

    function get(mapping(bytes32 => Info) storage self, address owner, int24 tickLower, int24 tickUpper)
        internal
        view
        returns (Info storage position)
    {
        position = self[keccak256(abi.encodePacked(owner, tickLower, tickUpper))];
    }

    function update(
        Info storage self,
        int128 liquidityDelta,
        uint256 feeGrowthInside0X128,
        uint256 feeGrowthInside1X128
    ) internal returns (uint256 feesOwed0, uint256 feesOwed1) {
        Info memory _self = self;

        uint128 liquidityNext;
        if (liquidityDelta == 0) {
            if (_self.liquidity == 0) revert CannotUpdateEmptyPosition(); // disallow pokes for 0 liquidity positions
            liquidityNext = _self.liquidity;
        } else {
            liquidityNext = liquidityDelta < 0
                ? _self.liquidity - uint128(-liquidityDelta)
                : _self.liquidity + uint128(liquidityDelta);
        }

        unchecked {
            feesOwed0 = FullMath.mulDiv(
                feeGrowthInside0X128 - _self.feeGrowthInside0LastX128, _self.liquidity, FixedPoint128.Q128
            );
            feesOwed1 = FullMath.mulDiv(
                feeGrowthInside1X128 - _self.feeGrowthInside1LastX128, _self.liquidity, FixedPoint128.Q128
            );
        }

        if (liquidityDelta != 0) self.liquidity = liquidityNext;
        self.feeGrowthInside0LastX128 = feeGrowthInside0X128;
        self.feeGrowthInside1LastX128 = feeGrowthInside1X128;
    }
}

library Pool {
    using Position for mapping(bytes32 => Position.Info);
    using Position for Position.Info;

    function getSwapFee(uint24 feesStorage) internal pure returns (uint16) {
        return uint16(feesStorage >> 12);
    }

    function getWithdrawFee(uint24 feesStorage) internal pure returns (uint16) {
        return uint16(feesStorage & 0xFFF);
    }

    struct ModifyPositionParams {
        address owner;
        int24 tickLower;
        int24 tickUpper;
        int128 liquidityDelta;
        int24 tickSpacing;
    }

    struct ModifyPositionState {
        bool flippedLower;
        uint128 liquidityGrossAfterLower;
        bool flippedUpper;
        uint128 liquidityGrossAfterUpper;
        uint256 feeGrowthInside0X128;
        uint256 feeGrowthInside1X128;
    }

    struct State {
        uint256 feeGrowthGlobal0X128;
        uint256 feeGrowthGlobal1X128;
        uint128 liquidity;
        mapping(int16 => uint256) tickBitmap;
        mapping(bytes32 => Position.Info) positions;
    }

    function modifyPosition(State storage self, ModifyPositionParams memory params) internal
    {

        uint256 feesOwed0;
        uint256 feesOwed1;
        {
            ModifyPositionState memory state;

            (feesOwed0, feesOwed1) = self.positions
                .get(params.owner, params.tickLower, params.tickUpper)
                .update(params.liquidityDelta, state.feeGrowthInside0X128, state.feeGrowthInside1X128);
        }
    }
}