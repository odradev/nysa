#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, U256};

#[soroban_sdk::contracttype]
pub struct OwnershipTransferred {
    previous_owner: Option<soroban_sdk::Address>,
    new_owner: soroban_sdk::Address,
}

#[derive(Clone)]
#[contracttype]
pub struct DataKey(pub Address);

#[contract]
pub struct StatusMessage;

#[contractimpl]
impl StatusMessage {
    pub fn set_status(env: Env, account_id: Address, message: String) {
        account_id.require_auth();
        env.storage().instance().set(&DataKey(account_id), &message);
    }

    pub fn get_status(env: Env, account_id: Address) -> String {
        env.storage()
            .instance()
            .get(&DataKey(account_id))
            .unwrap_or_else(|| String::from_str(&env, ""))
    }
}

// pub mod errors {}
// pub mod events {
//     #[soroban_sdk::contracttype]
//     pub struct OwnershipTransferred {
//         previous_owner: Option<soroban_sdk::Address>,
//         new_owner: Option<soroban_sdk::Address>,
//     }
//
//     impl OwnershipTransferred {
//         pub fn new(
//             previous_owner: Option<soroban_sdk::Address>,
//             new_owner: Option<soroban_sdk::Address>,
//         ) -> Self {
//             Self { previous_owner, new_owner }
//         }
//     }
// }
// pub mod enums {}
// pub mod structs {}
// pub mod owner {
//     #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]
//
//     use super::events::*;
//
//     #[derive(Clone)]
//     struct PathStack {
//         path: [ClassName; MAX_PATH_LENGTH],
//         stack_pointer: usize,
//         path_pointer: usize,
//     }
//
//     impl PathStack {
//         pub fn push_path_on_stack(&mut self) {
//             self.path_pointer = 0;
//             if self.stack_pointer < MAX_STACK_SIZE {
//                 self.stack_pointer += 1;
//             }
//         }
//         pub fn drop_one_from_stack(&mut self) {
//             if self.stack_pointer > 0 {
//                 self.stack_pointer -= 1;
//             }
//         }
//         pub fn pop_from_top_path(&mut self) -> Option<ClassName> {
//             if self.path_pointer < MAX_PATH_LENGTH {
//                 let class = self.path[MAX_PATH_LENGTH - self.path_pointer - 1];
//                 self.path_pointer += 1;
//                 Some(class)
//             } else {
//                 None
//             }
//         }
//     }
//
//     static mut STACK: PathStack = PathStack::new();
//
//     const MAX_STACK_SIZE: usize = 8;
//     const MAX_PATH_LENGTH: usize = 1usize;
//
//     impl PathStack {
//         pub const fn new() -> Self {
//             Self {
//                 path: [ClassName::Owner],
//                 stack_pointer: 0,
//                 path_pointer: 0,
//             }
//         }
//     }
//
//     #[derive(Clone, Copy)]
//     enum ClassName {
//         Owner
//     }
//
//     const OWNER: soroban_sdk::Symbol = soroban_sdk::symbol_short!("OWNER");
//
//     #[soroban_sdk::contract]
//     pub struct Owner {
//     }
//
//     #[soroban_sdk::contractimpl]
//     impl Owner {
//         pub fn get_owner(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
//             unsafe { STACK.push_path_on_stack(); }
//             let result = Self::super_get_owner(env);
//             unsafe { STACK.drop_one_from_stack(); }
//             result
//         }
//
//         fn super_get_owner(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
//             let __class = unsafe { STACK.pop_from_top_path() };
//             match __class {
//                 Some(ClassName::Owner) => {
//                     return env.storage().persistent().get::<_, Option<soroban_sdk::Address>>(&OWNER).unwrap_or(None);
//                 }
//                 #[allow(unreachable_patterns)]
//                 _ => Self::super_get_owner(env),
//             }
//         }
//
//         pub fn init(env: soroban_sdk::Env, caller: soroban_sdk::Address) {
//             env.storage().persistent().has(&OWNER).then(|| {
//                 panic!("Owner already set");
//             });
//
//             env.storage().persistent().set(&OWNER, &caller);
//         }
//
//         fn modifier_before_only_owner(env: soroban_sdk::Env) {
//             match env.storage().persistent().get::<_, Option<soroban_sdk::Address>>(&OWNER) {
//                 Some(Some(owner)) => {
//                     owner.require_auth();
//                 }
//                 _ => {
//                     panic!("Owner not set");
//                 }
//             }
//         }
//
//         fn modifier_after_only_owner(env: soroban_sdk::Env) {
//         }
//
//         pub fn transfer_ownership(env: soroban_sdk::Env, new_owner: Option<soroban_sdk::Address>) {
//             unsafe { STACK.push_path_on_stack(); }
//             let result = Self::super_transfer_ownership(env, new_owner);
//             unsafe { STACK.drop_one_from_stack(); }
//             result
//         }
//
//         fn super_transfer_ownership(env: soroban_sdk::Env, new_owner: Option<soroban_sdk::Address>) {
//             let __class = unsafe { STACK.pop_from_top_path() };
//             match __class {
//                 Some(ClassName::Owner) => {
//                     Self::modifier_before_only_owner(env.clone());
//                     let mut old_owner = env.storage().persistent().get(&OWNER).unwrap_or(None);
//                     env.storage().persistent().set(&OWNER, &new_owner);
//                     env.events().publish((), OwnershipTransferred::new(old_owner, new_owner));
//                     Self::modifier_after_only_owner(env);
//                 }
//                 #[allow(unreachable_patterns)]
//                 _ => Self::super_transfer_ownership(env, new_owner),
//             }
//         }
//     }
// }

mod test;
