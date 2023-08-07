use odra::{
    contract_env, execution_error,
    types::{event::OdraEvent, Address, U256},
    Mapping, UnwrapOrRevert, Variable,
};

#[derive(odra::Event, PartialEq, Eq, Debug)]
pub struct Transfer {
    from: Option<Address>,
    to: Option<Address>,
    value: U256,
}

execution_error! {
    pub enum Error {
        InvalidRecipientAddress => 1,
        InsufficientBalance => 2,
        NotOwner => 3
    }
}

#[odra::module]
pub struct OwnedToken {
    owner: Variable<Option<Address>>,
    name: Variable<String>,
    symbol: Variable<String>,
    decimals: Variable<u8>,
    total_supply: Variable<U256>,
    balance_of: Mapping<Option<Address>, U256>,
}

#[odra::module]
impl OwnedToken {
    #[odra(init)]
    pub fn init(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        let caller = contract_env::caller();
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
        if Some(contract_env::caller()) != self.owner.get().unwrap_or_revert() {
            contract_env::revert(Error::NotOwner)
        }
    }

    pub fn get_owner(&self) -> Option<Address> {
        self.owner.get().unwrap_or_revert()
    }

    pub fn burn(&mut self, amount: U256) {
        self.only_owner();
        let caller = contract_env::caller();
        let balance = self.balance_of.get_or_default(&Some(caller));
        if balance < amount {
            contract_env::revert(Error::InsufficientBalance)
        }
        self.total_supply.subtract(amount);
        self.balance_of.subtract(&Some(caller), amount);
        <Transfer as OdraEvent>::emit(Transfer {
            from: Some(caller),
            to: None,
            value: amount,
        });
    }

    pub fn mint(&mut self, to: Option<Address>, amount: U256) {
        self.only_owner();
        if to.is_none() {
            contract_env::revert(Error::InvalidRecipientAddress)
        }
        self.total_supply.add(amount);
        self.balance_of.add(&to, amount);
        <Transfer as OdraEvent>::emit(Transfer {
            from: None,
            to,
            value: amount,
        });
    }

    pub fn transfer(&mut self, to: Option<Address>, value: U256) {
        self._transfer(Some(contract_env::caller()), to, value);
    }

    pub fn _transfer(&mut self, from: Option<Address>, to: Option<Address>, value: U256) {
        if to.is_none() {
            contract_env::revert(Error::InvalidRecipientAddress)
        }
        let balance = self.balance_of.get(&from).unwrap_or_default();
        if balance < value {
            contract_env::revert(Error::InsufficientBalance)
        }
        self.balance_of.set(&from, balance - value);
        self.balance_of.add(&to, value);
        <Transfer as OdraEvent>::emit(Transfer { from, to, value });
    }
}
