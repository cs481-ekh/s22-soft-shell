#[no_mangle]
pub extern "C" fn lib_function_example_add_clib(num_one: usize, num_two: usize) -> usize {
    num_one + num_two
}

pub fn lib_function_example_add(num_one: usize, num_two: usize) -> usize {
    num_one + num_two
}

// Unit tests here
#[cfg(test)]
mod tests {
    use crate::{lib_function_example_add, lib_function_example_add_clib};

    #[test]
    fn test_add() {
        let result = lib_function_example_add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_c() {
        let result = lib_function_example_add_clib(2, 22);
        assert_eq!(result, 24);
    }

    #[test]
    #[ignore]
    fn test_add_wrong() {
        let result = 3;
        assert_eq!(result, 4);
    }
}
