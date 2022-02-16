mod capi;

pub fn lib_function_example_add(num_one: usize, num_two: usize) -> usize {
    num_one + num_two
}

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub calculator1);

// Unit tests here
#[cfg(test)]
mod tests {
    use crate::{lib_function_example_add, calculator1};

    // Testing larlpop functionality
    #[test]
    fn calculator1() {
        assert!(calculator1::TermParser::new().parse("22").is_ok());
        assert!(calculator1::TermParser::new().parse("(22)").is_ok());
        assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
        assert!(calculator1::TermParser::new().parse("((22)").is_err());
    }

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
