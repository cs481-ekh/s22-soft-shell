//! MIT-licensed IEC 61131-3 Structured Text interpreter library.

mod ast;
mod capi;
mod prog_handle;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

/// Simple example function used for testing tests and C integration
pub fn lib_function_example_add(num_one: usize, num_two: usize) -> usize {
    num_one + num_two
}

/// Check that the parser accepts a valid file
pub fn parser_test() -> bool {
    parser::ProgramParser::new()
        .parse(&read_file("tests/st_testing_subsets/01_Int.st"))
        .is_ok()
}

/// Read in the contents of a file to a String
pub fn read_file(file_path: &str) -> String {
    let file = File::open(file_path).unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut test = String::new();

    loop {
        if let Some(s) = lines.next() {
            test.push_str(&s.unwrap());
        } else {
            break;
        }
    }
    return test;
}

/// Unit tests for interpreter
#[cfg(test)]
mod tests {
    use crate::{lib_function_example_add, parser, read_file};

    #[test]
    /// Check parser succeeds over subset 1 ST programs.
    fn subset1_lexer_ast() {
        println!(
            "{:?}",
            parser::ProgramParser::new().parse(&read_file("tests/st_testing_subsets/01_Int.st"))
        );
        assert!(parser::ProgramParser::new()
            .parse(&read_file("tests/st_testing_subsets/01_Int.st"))
            .is_ok());

        println!(
            "{:?}",
            parser::ProgramParser::new().parse(&read_file("tests/st_testing_subsets/01_Bool.st"))
        );
        assert!(parser::ProgramParser::new()
            .parse(&read_file("tests/st_testing_subsets/01_Bool.st"))
            .is_ok());

        println!(
            "{:?}",
            parser::ProgramParser::new().parse(&read_file("tests/st_testing_subsets/01_Real.st"))
        );
        assert!(parser::ProgramParser::new()
            .parse(&read_file("tests/st_testing_subsets/01_Real.st"))
            .is_ok());
    }

    #[test]
    /// Test addition sample function.
    fn test_add() {
        let result = lib_function_example_add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    /// An example test that would always fail.
    fn test_add_wrong() {
        let result = 3;
        assert_eq!(result, 4);
    }

    #[test]
    /// Test reading in the contents of a file.
    fn test_open_file() {
        let result = read_file("tests/st_testing_subsets/read_test.txt");
        assert_eq!(result.is_empty(), false);
        assert_eq!(result, "Hello World!");
    }
}
