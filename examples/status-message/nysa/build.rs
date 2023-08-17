const DEST_FILE_PATH: &str = "src/status_message.rs";
const SOURCE_FILE_PATH: &str = "src/status_message.sol";

fn main() {
    nysa::builder::generate_file::<&str, nysa::OdraParser>(SOURCE_FILE_PATH, DEST_FILE_PATH);
}
