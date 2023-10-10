const DEST_FILE_PATH: &str = "src/owned_token.rs";
const SOURCE_FILE_PATH: &str = "src/owned_token.sol";

fn main() {
    // println!("cargo:rerun-if-changed=src/owned_token.sol");
    nysa::builder::generate_file::<&str, nysa::OdraParser>(SOURCE_FILE_PATH, DEST_FILE_PATH);
}
