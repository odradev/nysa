pub trait Cast {
    fn cast<T: Cast>(&self) -> T;
}

impl Cast for soroban::U256 {
    fn cast<T: Cast>(&self) -> T {
        todo!()
    }
}
