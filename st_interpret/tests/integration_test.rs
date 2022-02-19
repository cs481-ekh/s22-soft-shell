// Rust integration tests here

use st_interpret;

#[test]
fn it_adds_two_integration() {
    assert_eq!(4, st_interpret::lib_function_example_add(2, 2));
}

#[test]
fn it_adds_15_integration() {
    assert_eq!(17, st_interpret::lib_function_example_add(15, 2));
}
