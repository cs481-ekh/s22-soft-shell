use crate::lib_function_example_add;

#[no_mangle]
pub extern "C" fn lib_function_example_add_clib(num_one: usize, num_two: usize) -> usize {
    lib_function_example_add(num_one, num_two)
}