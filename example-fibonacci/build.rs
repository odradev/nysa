const DEST_FILE_PATH: &str = "src/fibonacci_sol.rs";
const SOURCE_FILE_PATH: &str = "src/fibonacci.sol";

fn main() {
    nysa::builder::generate_file(SOURCE_FILE_PATH, DEST_FILE_PATH);
}

