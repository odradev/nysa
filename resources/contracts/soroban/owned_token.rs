pub mod errors {
}
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
    #![allow(
        unused_braces,
        unused_mut,
        unused_parens,
        non_snake_case,
        unused_imports,
        unused_variables
    )]
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
        ) -> Result<(), soroban_sdk::Error> {
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
        ) -> Result<(), soroban_sdk::Error> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::ERC20) => {
                    if !(_to != None) {
                        return Err(soroban_sdk::Error::from_contract_error(1u32));
                    };
                    if !(env
                        .storage()
                        .instance()
                        .get::<_, soroban_sdk::U256>(&BalanceOf(_from.clone()))
                        .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                        >= _value)
                    {
                        return Err(soroban_sdk::Error::from_contract_error(2u32));
                    };
                    env.storage().instance().set(
                        &BalanceOf(_from.clone()),
                        &env.storage()
                            .instance()
                            .get::<_, soroban_sdk::U256>(&BalanceOf(_from.clone()))
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .sub(&_value),
                    );
                    env.storage().instance().set(
                        &BalanceOf(_to.clone()),
                        &env.storage()
                            .instance()
                            .get::<_, soroban_sdk::U256>(&BalanceOf(_to.clone()))
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .add(&_value),
                    );
                    env.events().publish(
                        (),
                        Transfer::new(_from.clone(), _to.clone(), _value.clone()),
                    );
                    Ok(())
                }
                #[allow(unreachable_patterns)]
                _ => Self::super__transfer(env, caller, _from, _to, _value),
            }
        }
        pub fn burn(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _amount: soroban_sdk::U256,
        ) -> Result<(), soroban_sdk::Error> {
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
        ) -> Result<(), soroban_sdk::Error> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::OwnedToken) => {
                    Self::modifier_before_only_owner(env.clone(), caller.clone())?;
                    if !(env
                        .storage()
                        .instance()
                        .get::<_, soroban_sdk::U256>(&BalanceOf(caller.clone()))
                        .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                        >= _amount)
                    {
                        return Err(soroban_sdk::Error::from_contract_error(2u32));
                    };
                    env.storage().instance().set(
                        &TOTAL_SUPPLY,
                        &env.storage()
                            .instance()
                            .get::<_, soroban_sdk::U256>(&TOTAL_SUPPLY)
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .sub(&_amount),
                    );
                    env.storage().instance().set(
                        &BalanceOf(caller.clone()),
                        &env.storage()
                            .instance()
                            .get::<_, soroban_sdk::U256>(&BalanceOf(caller.clone()))
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .sub(&_amount),
                    );
                    env.events().publish(
                        (),
                        Transfer::new(caller.clone(), None.clone(), _amount.clone()),
                    );
                    Self::modifier_after_only_owner(env.clone(), caller.clone())?;
                    Ok(())
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_burn(env, caller, _amount),
            }
        }
        pub fn get_balance_of(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _owner: Option<soroban_sdk::Address>,
        ) -> Result<soroban_sdk::U256, soroban_sdk::Error> {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super_get_balance_of(env, caller, _owner);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super_get_balance_of(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _owner: Option<soroban_sdk::Address>,
        ) -> Result<soroban_sdk::U256, soroban_sdk::Error> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::ERC20) => {
                    return Ok(env
                        .storage()
                        .instance()
                        .get::<_, soroban_sdk::U256>(&BalanceOf(_owner.clone()))
                        .unwrap_or(soroban_sdk::U256::from_u32(&env, 0)));
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_get_balance_of(env, caller, _owner),
            }
        }
        pub fn get_owner(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
        ) -> Result<Option<soroban_sdk::Address>, soroban_sdk::Error> {
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
        ) -> Result<Option<soroban_sdk::Address>, soroban_sdk::Error> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    return Ok(env
                        .storage()
                        .instance()
                        .get::<_, Option<soroban_sdk::Address>>(&OWNER)
                        .unwrap_or(None));
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_get_owner(env, caller),
            }
        }
        pub fn get_total_supply(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
        ) -> Result<soroban_sdk::U256, soroban_sdk::Error> {
            unsafe {
                STACK.push_path_on_stack();
            }
            let result = Self::super_get_total_supply(env, caller);
            unsafe {
                STACK.drop_one_from_stack();
            }
            result
        }
        fn super_get_total_supply(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
        ) -> Result<soroban_sdk::U256, soroban_sdk::Error> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::ERC20) => {
                    return Ok(env
                        .storage()
                        .instance()
                        .get::<_, soroban_sdk::U256>(&TOTAL_SUPPLY)
                        .unwrap_or(soroban_sdk::U256::from_u32(&env, 0)));
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_get_total_supply(env, caller),
            }
        }
        fn _owner_init(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) -> Result<(), soroban_sdk::Error> {
            env.storage().instance().set(&OWNER, &caller);
            Ok(())
        }
        fn _erc_20_init(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _name: soroban_sdk::String,
            _symbol: soroban_sdk::String,
            _decimals: u32,
            _initial_supply: soroban_sdk::U256,
        ) -> Result<(), soroban_sdk::Error> {
            env.storage().instance().set(&NAME, &_name);
            env.storage().instance().set(&SYMBOL, &_symbol);
            env.storage().instance().set(&DECIMALS, &_decimals);
            env.storage().instance().set(
                &TOTAL_SUPPLY,
                &(_initial_supply.mul(
                    &(soroban_sdk::U256::from_parts(&env, 0u64, 0u64, 0u64, 10u64).pow(
                        env.storage()
                            .instance()
                            .get::<_, u32>(&DECIMALS)
                            .unwrap_or_default(),
                    )),
                )),
            );
            env.storage().instance().set(
                &BalanceOf(caller.clone()),
                &env.storage()
                    .instance()
                    .get::<_, soroban_sdk::U256>(&TOTAL_SUPPLY)
                    .unwrap_or(soroban_sdk::U256::from_u32(&env, 0)),
            );
            Ok(())
        }
        pub fn init(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _name: soroban_sdk::String,
            _symbol: soroban_sdk::String,
            _decimals: u32,
            _initial_supply: soroban_sdk::U256,
        ) -> Result<(), soroban_sdk::Error> {
            Self::_owner_init(env.clone(), caller.clone())?;
            Self::_erc_20_init(
                env.clone(),
                caller.clone(),
                _name,
                _symbol,
                _decimals,
                _initial_supply,
            )?;
            Ok(())
        }
        pub fn mint(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _to: Option<soroban_sdk::Address>,
            _amount: soroban_sdk::U256,
        ) -> Result<(), soroban_sdk::Error> {
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
        ) -> Result<(), soroban_sdk::Error> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::OwnedToken) => {
                    Self::modifier_before_only_owner(env.clone(), caller.clone())?;
                    if !(_to != None) {
                        return Err(soroban_sdk::Error::from_contract_error(1u32));
                    };
                    env.storage().instance().set(
                        &TOTAL_SUPPLY,
                        &env.storage()
                            .instance()
                            .get::<_, soroban_sdk::U256>(&TOTAL_SUPPLY)
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .add(&_amount),
                    );
                    env.storage().instance().set(
                        &BalanceOf(_to.clone()),
                        &env.storage()
                            .instance()
                            .get::<_, soroban_sdk::U256>(&BalanceOf(_to.clone()))
                            .unwrap_or(soroban_sdk::U256::from_u32(&env, 0))
                            .add(&_amount),
                    );
                    env.events().publish(
                        (),
                        Transfer::new(None.clone(), _to.clone(), _amount.clone()),
                    );
                    Self::modifier_after_only_owner(env.clone(), caller.clone())?;
                    Ok(())
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_mint(env, caller, _to, _amount),
            }
        }
        fn modifier_before_only_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) -> Result<(), soroban_sdk::Error> {
            caller.expect("`caller` must not be `None`").require_auth();
            Ok(())
        }
        fn modifier_after_only_owner(env: soroban_sdk::Env, caller: Option<soroban_sdk::Address>) -> Result<(), soroban_sdk::Error> {
            Ok(())
        }
        pub fn transfer(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            _to: Option<soroban_sdk::Address>,
            _value: soroban_sdk::U256,
        ) -> Result<(), soroban_sdk::Error> {
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
        ) -> Result<(), soroban_sdk::Error> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::ERC20) => {
                    Self::_transfer(env.clone(), caller.clone(), caller, _to, _value)?;
                    Ok(())
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_transfer(env, caller, _to, _value),
            }
        }
        pub fn transfer_ownership(
            env: soroban_sdk::Env,
            caller: Option<soroban_sdk::Address>,
            new_owner: Option<soroban_sdk::Address>,
        ) -> Result<(), soroban_sdk::Error> {
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
        ) -> Result<(), soroban_sdk::Error> {
            let __class = unsafe { STACK.pop_from_top_path() };
            match __class {
                Some(ClassName::Owner) => {
                    Self::modifier_before_only_owner(env.clone(), caller.clone())?;
                    env.storage().instance().set(&OWNER, &new_owner);
                    Self::modifier_after_only_owner(env.clone(), caller.clone())?;
                    Ok(())
                }
                #[allow(unreachable_patterns)]
                _ => Self::super_transfer_ownership(env, caller, new_owner),
            }
        }
    }
}
