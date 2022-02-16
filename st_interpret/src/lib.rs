mod capi;

pub fn lib_function_example_add(num_one: usize, num_two: usize) -> usize {
    num_one + num_two
}

// Unit tests here
#[cfg(test)]
mod tests {
    use crate::{lib_function_example_add};

    #[test]
    fn test_add() {
        let result = lib_function_example_add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn test_add_wrong() {
        let result = 3;
        assert_eq!(result, 4);
    }
}
