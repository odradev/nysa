use odra::Mapping;

#[odra::module]
pub struct Fibonacci {
    results: Mapping<u32, u32>,
}

#[odra::module]
impl Fibonacci {
    pub fn compute(&mut self, input: u32) {
        self.results.set(input, self.fibb(input));
    }

    pub fn get_result(&self, input: u32) -> u32 {
        self.results.get_or_default(&input)
    }

    fn fibb(&self, n: u32) -> u32 {
        if n <= 1 {
            return n;
        } else {
            return self.fibb(n - 1) + self.fibb(n - 2);
        }
    }
}
