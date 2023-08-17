const DEST_FILE_PATH: &str = "src/plascoin.rs";
const SOURCE_FILE_PATH: &str = "src/plascoin.sol";

fn main() {
    nysa::builder::generate_file::<&str, nysa::OdraParser>(SOURCE_FILE_PATH, DEST_FILE_PATH);
}
