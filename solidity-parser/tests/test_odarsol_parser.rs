use solidity_parser;

#[test]
fn test_parser() {
    let input = r#"
    contract flipper {
        /// Simply returns the current value of our `bool`.
        function get() public view returns (bool) {
            return true == super.get();
        }
    }
    "#;

    let _result = solidity_parser::parse(input, 0).unwrap();

    // dbg!(result);

    // assert!(false);
}
