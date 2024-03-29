use odra::{Address, Mapping};
use odra::prelude::*;

#[odra::module]
pub struct StatusMessage {
    records: Mapping<Address, String>,
}

#[odra::module]
impl StatusMessage {
    pub fn set_status(&mut self, message: String) {
        let account_id = self.env().caller();
        self.records.set(&account_id, message);
    }

    pub fn get_status(&self, account_id: Address) -> String {
        self.records.get_or_default(&account_id)
    }
}
