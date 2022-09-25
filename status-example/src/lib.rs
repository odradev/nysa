// c3_lang! {
//     impl BorshDeserialize for PathStack {
//         fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
//             Ok(Default::default())
//         }
//     }

//     impl BorshSerialize for PathStack {
//         fn serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::io::Result<()> {
//             Ok(())
//         }
//     }

//     #[near_bindgen]
//     #[derive(Default, BorshDeserialize, BorshSerialize)]
//     pub struct StatusMessage {
//         records: HashMap<AccountId, String>,
//     }

//     #[near_bindgen]
//     impl StatusMessage {
//         #[payable]
//         pub fn set_status(&mut self, message: String) {
//             let account_id = env::signer_account_id();
//             self.records.insert(account_id, message);
//         }

//         pub fn get_status(&self, account_id: AccountId) -> String {
//             self.records.get(&account_id).cloned().unwrap_or_default()
//         }
//     }
// }

use nysa_macro::nysa_lang;
nysa_lang! {
    contract StatusMessage {
        mapping(address => string) records;

        function set_status(string memory status) public payable {
            address account_id = msg.sender;
            records[account_id] = status;
        }

        function get_status(address account_id) public view returns (string memory) {
            return records[account_id];
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn set_get_message() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        let context = get_context(true);
        testing_env!(context);
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".parse().unwrap())
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(
            String::new(),
            contract.get_status("francis.near".parse().unwrap())
        );
    }
}
