// Rust integration tests here

use st_interpret;

#[test]
fn it_adds_two_integration() {
    assert_eq!(4,  st_interpret::lib_function_example_add(2, 2));
}