const DEST_FILE_PATH: &str = "src/status_message_sol.rs";
const SOURCE_FILE_PATH: &str = "src/status_message.sol";

fn main() {
    nysa::builder::generate_file(SOURCE_FILE_PATH, DEST_FILE_PATH);
}
