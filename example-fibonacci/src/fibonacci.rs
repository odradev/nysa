use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen,
};
use std::collections::HashMap;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Fibonacci {
    results: HashMap<u32, u32>,
}

#[near_bindgen]
impl Fibonacci {
    pub fn compute(&mut self, input: u32) {
        self.results.insert(input, self.fibb(input));
    }

    pub fn get_result(&self, input: u32) -> u32 {
        self.results.get(&input).cloned().unwrap_or_default()
    }

    fn fibb(&self, n: u32) -> u32 {
        if n <= 1 {
            return n;
        } else {
            return self.fibb(n - 1) + self.fibb(n - 2);
        }
    }
}
