pub mod errors {}
pub mod events {
    #[soroban_sdk::contracttype]
    pub struct Transfer {
        from: Option<soroban_sdk::Address>,
        to: Option<soroban_sdk::Address>,
        value: soroban_sdk::U256,
    }
    impl Transfer {
        pub fn new(
            from: Option<soroban_sdk::Address>,
            to: Option<soroban_sdk::Address>,
            value: soroban_sdk::U256,
        ) -> Self {
            Self { from, to, value }
        }
    }
}
pub mod enums {}
pub mod structs {}
pub mod owned_token {
    use super::errors::*;
    use super::events::*;
    use super::structs::*;
    const OWNER: soroban_sdk::Symbol = soroban_sdk::symbol_short!("OWNER");
    const NAME: soroban_sdk::Symbol = soroban_sdk::symbol_short!("NAME");
    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::symbol_short!("SYMBOL");
    const DECIMALS: soroban_sdk::Symbol = soroban_sdk::symbol_short!("DECIMALS");
    const TOTAL_SUPPLY: soroban_sdk::Symbol = soroban_sdk::symbol_short!("TOTAL_SUP");
    #[soroban_sdk::contracttype]
    pub struct BalanceOf(pub Option<soroban_sdk::Address>);
    #[derive(Clone)]
    struct PathStack {
        path: [ClassName; MAX_PATH_LENGTH],
        stack_pointer: usize,
        path_pointer: usize,
    }
    impl PathStack {
        pub fn push_path_on_stack(&mut self) {
            self.path_pointer = 0;
            if self.stack_pointer < MAX_STACK_SIZE {
                self.stack_pointer += 1;
            }
        }
        pub fn drop_one_from_stack(&mut self) {
            if self.stack_pointer > 0 {
                self.stack_pointer -= 1;
            }
        }
        pub fn pop_from_top_path(&mut self) -> Option<ClassName> {
            if self.path_pointer < MAX_PATH_LENGTH {
                let class = self.path[MAX_PATH_LENGTH - self.path_pointer - 1];
                self.path_pointer += 1;
                Some(class)
            } else {
                None
            }
        }
    }
    static mut STACK: PathStack = PathStack::new();
    const MAX_STACK_SIZE: usize = 8;
    const MAX_PATH_LENGTH: usize = 3usize;
    impl PathStack {
        pub const fn new() -> Self {
            Self {
                path: [ClassName::Owner, ClassName::ERC20, ClassName::OwnedToken],
                stack_pointer: 0,
                path_pointer: 0,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum ClassName {
        OwnedToken,
        ERC20,
        Owner,
    }
    #[soroban_sdk::contract]
    pub struct OwnedToken {}
    #[soroban_sdk::contractimpl]
    impl OwnedToken {
        pub(crate) fn _transfer(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _from: Option<soroban_sdk::Address>,
            _to: Option<soroban_sdk::Address>,
            _value: soroban_sdk::U256,
        ) {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super__transfer(env, caller, _from, _to, _value);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super__transfer(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _from: Option<soroban_sdk::Address>,
            _to: Option<soroban_sdk::Address>,
            _value: soroban_sdk::U256,
        ) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::ERC20) => {
                    if !(_to != None) {
                        panic!("Invalid recipient address.")
                    };
                    if !(env
                        .storage()
                        .persistent()
                        .get::<_, soroban_sdk::U256>(&BalanceOf(_from))
                        .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                        >= _value)
                    {
                        panic!("Insufficient balance.")
                    };
                    env.storage().persistent().set(
                        &BalanceOf(_from),
                        &env.storage()
                            .persistent()
                            .get::<_, soroban_sdk::U256>(&BalanceOf(_from))
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .sub(&_value)
                    );
                    env.storage().persistent().set(
                        &BalanceOf(_to),
                        &env.storage()
                            .persistent()
                            .get::<_, soroban_sdk::U256>(&BalanceOf(_to))
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .add(&_value)
                    );
                    env.events().publish((), Transfer::new(_from, _to, _value));
                }
                #[allow(unreachable_patterns)]
                _ => Self::super__transfer(env, caller, _from, _to, _value),
            }
        }
        pub fn burn(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _amount: soroban_sdk::U256,
        ) {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super_burn(env, caller, _amount);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super_burn(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _amount: soroban_sdk::U256,
        ) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::OwnedToken) => {
                    Self::modifier_before_only_owner(env.clone(), caller.clone());
                    if !(env
                        .storage()
                        .persistent()
                        .get::<_, soroban_sdk::U256>(&BalanceOf(caller))
                        .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                        >= _amount)
                    {
                        panic!("Insufficient balance.")
                    };
                    env.storage().persistent().set(
                        &TOTAL_SUPPLY,
                        &env.storage()
                            .persistent()
                            .get::<_, soroban_sdk::U256>(&TOTAL_SUPPLY)
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .sub(&_amount)
                    );
                    env.storage().persistent().set(
                        &BalanceOf(caller),
                        &env.storage()
                            .persistent()
                            .get::<_, soroban_sdk::U256>(&BalanceOf(caller))
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .sub(&_amount)
                    );
                    env.events()
                        .publish((), Transfer::new(caller, None, _amount));
                    Self::modifier_after_only_owner(env.clone(), caller.clone());
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_burn(env, caller, _amount),
            }
        }
        pub fn get_owner(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
        ) -> Option<soroban_sdk::Address> {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super_get_owner(env, caller);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super_get_owner(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
        ) -> Option<soroban_sdk::Address> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    return env
                        .storage()
                        .persistent()
                        .get::<_, Option<soroban_sdk::Address>>(&OWNER)
                        .unwrap_or(None);
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_get_owner(env, caller),
            }
        }
        fn _owner_init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
            env.storage().persistent().set(&OWNER, &caller);
        }
        fn _erc_20_init(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _name: soroban_sdk::String,
            _symbol: soroban_sdk::String,
            _decimals: u32,
            _initial_supply: soroban_sdk::U256,
        ) {
            env.storage().persistent().set(&NAME, &_name);
            env.storage().persistent().set(&SYMBOL, &_symbol);
            env.storage().persistent().set(&DECIMALS, &_decimals);
            env.storage().persistent().set(
                &TOTAL_SUPPLY,
                &_initial_supply.mul(
                    &soroban_sdk::U256::from_parts(&env, 10u64, 0u64, 0u64, 0u64).pow(
                        env.storage()
                            .persistent()
                            .get::<_, u32>(&DECIMALS)
                            .unwrap_or_default(),
                    ),
                ),
            );
            env.storage().persistent().set(
                &BalanceOf(caller),
                &env.storage()
                    .persistent()
                    .get::<_, soroban_sdk::U256>(&TOTAL_SUPPLY)
                    .unwrap_or(soroban_sdk::U256::from_u32(&env, 0)),
            );
        }
        pub fn init(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _name: soroban_sdk::String,
            _symbol: soroban_sdk::String,
            _decimals: u32,
            _initial_supply: soroban_sdk::U256,
        ) {
            Self::_owner_init(env.clone(), caller.clone());
            Self::_erc_20_init(
                env.clone(),
                caller.clone(),
                _name,
                _symbol,
                _decimals,
                _initial_supply,
            );
        }
        pub fn mint(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _to: Option<soroban_sdk::Address>,
            _amount: soroban_sdk::U256,
        ) {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super_mint(env, caller, _to, _amount);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super_mint(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _to: Option<soroban_sdk::Address>,
            _amount: soroban_sdk::U256,
        ) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::OwnedToken) => {
                    Self::modifier_before_only_owner(env.clone(), caller.clone());
                    if !(_to != None) {
                        panic!("Invalid recipient address.")
                    };
                    env.storage().persistent().set(
                        &TOTAL_SUPPLY,
                        &env.storage()
                            .persistent()
                            .get::<_, soroban_sdk::U256>(&TOTAL_SUPPLY)
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .add(&_amount),
                    );
                    env.storage().persistent().set(
                        &BalanceOf(_to),
                        &env.storage()
                            .persistent()
                            .get::<_, soroban_sdk::U256>(&BalanceOf(_to))
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .add(&_amount),
                    );
                    env.events().publish((), Transfer::new(None, _to, _amount));
                    Self::modifier_after_only_owner(env.clone(), caller.clone());
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_mint(env, caller, _to, _amount),
            }
        }
        fn modifier_before_only_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {
            caller.expect("`caller` must not be `None`").require_auth();
        }
        fn modifier_after_only_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) {}
        pub fn transfer(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _to: Option<soroban_sdk::Address>,
            _value: soroban_sdk::U256,
        ) {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super_transfer(env, caller, _to, _value);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super_transfer(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _to: Option<soroban_sdk::Address>,
            _value: soroban_sdk::U256,
        ) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::ERC20) => {
                    Self::_transfer(env.clone(), caller.clone(), caller, _to, _value);
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_transfer(env, caller, _to, _value),
            }
        }
        pub fn transfer_ownership(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            new_owner: Option<soroban_sdk::Address>,
        ) {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super_transfer_ownership(env, caller, new_owner);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super_transfer_ownership(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            new_owner: Option<soroban_sdk::Address>,
        ) {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    Self::modifier_before_only_owner(env.clone(), caller.clone());
                    env.storage().persistent().set(&OWNER, &new_owner);
                    Self::modifier_after_only_owner(env.clone(), caller.clone());
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_transfer_ownership(env, caller, new_owner),
            }
        }
    }
}
