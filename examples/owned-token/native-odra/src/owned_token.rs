use odra::{
    casper_types::U256, prelude::*, Address, Mapping, OdraError, UnwrapOrRevert, Var
};

#[derive(odra::Event, PartialEq, Eq, Debug)]
pub struct Transfer {
    from: Option<Address>,
    to: Option<Address>,
    value: U256,
}

#[derive(OdraError, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRecipientAddress = 1,
    InsufficientBalance = 2,
    NotOwner = 3
}

#[odra::module]
pub struct OwnedToken {
    owner: Var<Option<Address>>,
    name: Var<String>,
    symbol: Var<String>,
    decimals: Var<u8>,
    total_supply: Var<U256>,
    balance_of: Mapping<Option<Address>, U256>,
}

#[odra::module]
impl OwnedToken {
    pub fn init(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        let caller = self.env().caller();
        self.owner.set(Some(caller));
        self.name.set(name);
        self.symbol.set(symbol);
        self.decimals.set(decimals);
        self.total_supply.set(initial_supply);
        self.balance_of
            .set(&Some(caller), self.total_supply.get_or_default());
    }

    pub fn transfer_ownership(&mut self, new_owner: Option<Address>) {
        self.only_owner();
        self.owner.set(new_owner);
    }

    pub fn only_owner(&self) {
        let env = self.env();
        if Some(env.caller()) != self.owner.get().unwrap_or_revert(&env) {
            env.revert(Error::NotOwner)
        }
    }

    pub fn get_owner(&self) -> Option<Address> {
        self.owner.get().unwrap_or_revert(&self.env())
    }

    pub fn burn(&mut self, amount: U256) {
        self.only_owner();
        let env = self.env();
        let caller = env.caller();
        let balance = self.balance_of.get_or_default(&Some(caller));
        if balance < amount {
            env.revert(Error::InsufficientBalance)
        }
        self.total_supply.subtract(amount);
        self.balance_of.subtract(&Some(caller), amount);
        env.emit_event(Transfer {
            from: Some(caller),
            to: None,
            value: amount,
        });
    }

    pub fn mint(&mut self, to: Option<Address>, amount: U256) {
        self.only_owner();
        let env = self.env();
        if to.is_none() {
            env.revert(Error::InvalidRecipientAddress)
        }
        self.total_supply.add(amount);
        self.balance_of.add(&to, amount);
        env.emit_event(Transfer {
            from: None,
            to,
            value: amount,
        });
    }

    pub fn transfer(&mut self, to: Option<Address>, value: U256) {
        self._transfer(Some(self.env().caller()), to, value);
    }

    pub fn _transfer(&mut self, from: Option<Address>, to: Option<Address>, value: U256) {
        let env = self.env();
        if to.is_none() {
            env.revert(Error::InvalidRecipientAddress)
        }
        let balance = self.balance_of.get(&from).unwrap_or_default();
        if balance < value {
            env.revert(Error::InsufficientBalance)
        }
        self.balance_of.set(&from, balance - value);
        self.balance_of.add(&to, value);
        env.emit_event(Transfer { from, to, value });
    }
}
