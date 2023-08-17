use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{account::AccountHash, Key};
use odra_casper_shared::key_maker::KeyMaker;

struct PlascoinKeyMaker;

impl KeyMaker for PlascoinKeyMaker {
    fn blake2b(preimage: &[u8]) -> [u8; 32] {
        let mut result = [0; 32];
        let mut hasher = VarBlake2b::new(32).expect("should create hasher");

        hasher.update(preimage);
        hasher.finalize_variable(|slice| {
            result.copy_from_slice(slice);
        });
        result
    }
}

enum StorageKey {
    Cap,
    Balances(String),
    Allowances(String),
    TotalSupply,
    Name,
    Symbol,
    Owner,
}

impl StorageKey {
    fn as_string(&self) -> String {
        let bytes = match self {
            Self::Cap => PlascoinKeyMaker::to_variable_key(b"_cap"),
            Self::Balances(addr_str) => {
                let account = to_option_account(&addr_str);
                PlascoinKeyMaker::to_dictionary_key(b"_balances", &account)
                    .expect("Invalid balances key")
            }
            Self::Allowances(addr_str) => {
                let account = to_option_account(&addr_str);
                PlascoinKeyMaker::to_dictionary_key(b"_allowances", &account)
                    .expect("Invalid allowances key")
            }
            Self::TotalSupply => PlascoinKeyMaker::to_variable_key(b"_total_supply"),
            Self::Name => PlascoinKeyMaker::to_variable_key(b"_name"),
            Self::Symbol => PlascoinKeyMaker::to_variable_key(b"_symbol"),
            Self::Owner => PlascoinKeyMaker::to_variable_key(b"_owner"),
        };
        std::str::from_utf8(&bytes)
            .expect("Invalid key format")
            .to_owned()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pk: String = args.get(1).cloned().expect("key expected but not provided");
    let sk = args.get(2);
    let storage_key = match pk.as_str() {
        "_cap" => Ok(StorageKey::Cap),
        "_balances" => Ok(StorageKey::Balances(sk.cloned().expect("Missing argument"))),
        "_allowances" => Ok(StorageKey::Allowances(
            sk.cloned().expect("Missing argument"),
        )),
        "_total_supply" => Ok(StorageKey::TotalSupply),
        "_name" => Ok(StorageKey::Name),
        "_symbol" => Ok(StorageKey::Symbol),
        "_owner" => Ok(StorageKey::Owner),
        _ => Err(()),
    }
    .expect("Unknown key");
    println!("{}", storage_key.as_string());
}

fn to_option_account(string: &str) -> Option<Key> {
    AccountHash::from_formatted_str(&string)
        .map(Key::Account)
        .ok()
}
