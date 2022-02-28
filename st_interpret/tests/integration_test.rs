// Rust integration tests here

use st_interpret;
use st_interpret::read_file;

/// Test lalrpop functionality
#[test]
fn example_parser() {
    assert!(st_interpret::parser_test());
}

#[test]
/// Test the ability to read in a file
fn test_open_file() {
    let a = read_file("tests/st_testing_subsets/01_Bool.st");
    assert_eq!(a.is_empty(), false);
}

#[test]
/// Example test that does addition
fn it_adds_two_integration() {
    assert_eq!(4, st_interpret::lib_function_example_add(2, 2));
}

#[test]
/// Example test that does addition
fn it_adds_15_integration() {
    assert_eq!(17, st_interpret::lib_function_example_add(15, 2));
}
