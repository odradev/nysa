const DEST_FILE_PATH: &str = "src/owned_token_sol.rs";
const SOURCE_FILE_PATH: &str = "src/owned_token.sol";

fn main() {
    nysa::builder::generate_file(SOURCE_FILE_PATH, DEST_FILE_PATH);
}
