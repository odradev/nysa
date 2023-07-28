#[cfg(feature = "solidity")]
mod fibonacci_sol;

#[cfg(feature = "solidity")]
pub use fibonacci_sol::{Fibonacci, FibonacciDeployer, FibonacciRef};

#[cfg(feature = "native-odra")]
mod fibonacci;

#[cfg(feature = "native-odra")]
pub use fibonacci::Fibonacci;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let mut contract = FibonacciDeployer::default();

        let mut test = |n: u32, expected: u32| {
            contract.compute(n);
            let result = contract.get_result(n);
            assert_eq!(result, expected);
        };

        test(1, 1);
        test(2, 1);
        test(3, 2);
        test(4, 3);
        test(5, 5);
        test(6, 8);
        test(7, 13);
        test(8, 21);
        test(9, 34);
        test(10, 55);
        test(31, 1346269);
    }
}
