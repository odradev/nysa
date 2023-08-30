// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IUniswapV3Pool {
    function deposit(uint256 tokenId, uint128 liquidity, uint256 amount0Min, uint256 amount1Min) external;
}

contract SimpleUniswapV3Pool {
    address public token0;
    address public token1;
    uint24 public fee;
    IUniswapV3Pool public pool;

    constructor(address _token0, address _token1, uint24 _fee, address _pool) {
        token0 = _token0;
        token1 = _token1;
        fee = _fee;
        pool = IUniswapV3Pool(_pool);
    }

    function deposit(uint128 liquidity, uint256 amount0Min, uint256 amount1Min) external {
        pool.deposit(0, liquidity, amount0Min, amount1Min);
    }
}
