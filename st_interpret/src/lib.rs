//! MIT-licensed IEC 61131-3 Structured Text interpreter library.

mod ast;
mod capi;
pub mod prog_handle;

use std::fs;

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
        .parse(&read_file("tests/test_inputs/st_subset_1/01_Int.st"))
        .is_ok()
}

/// Read in the contents of a file to a String
pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Unable to read file");
    return contents;
}

/// Unit tests for interpreter
#[cfg(test)]
mod tests {
    use crate::ast::{VariableKind, VariableValue};
    use crate::prog_handle::ProgContext;
    use crate::{lib_function_example_add, parser, read_file};
    use std::fs;

    #[test]
    /// Check parser succeeds over subset 1 ST programs.
    fn subset1_parse() {
        parser_batch_test_st_folder("tests/test_inputs/st_subset_1");
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
        let result = read_file("tests/test_inputs/misc/read_test.txt");
        assert_eq!(result.is_empty(), false);
        assert_eq!(result, "Hello World!");
    }

    #[test]
    /// Test adding an int variable to a ProgContext
    fn test_add_var_int() {
        let mut prog_context = ProgContext::new();
        prog_context.add_var(
            String::from("variable"),
            VariableKind::NORMAL,
            VariableValue::INT(10),
        );
    }

    #[test]
    /// Test adding a bool variable to a ProgContext
    fn test_add_var_bool() {
        let mut prog_context = ProgContext::new();
        prog_context.add_var(
            String::from("variable"),
            VariableKind::NORMAL,
            VariableValue::BOOL(false),
        );
    }

    #[test]
    /// Test adding a real variable to a ProgContext
    fn test_add_var_real() {
        let mut prog_context = ProgContext::new();
        prog_context.add_var(
            String::from("variable"),
            VariableKind::NORMAL,
            VariableValue::REAL(1.5),
        );
    }

    #[test]
    /// Test updating a variable works
    fn update_var() {
        let mut prog_context = ProgContext::new();
        prog_context.add_var(
            String::from("myvar"),
            VariableKind::NORMAL,
            VariableValue::INT(4),
        );

        prog_context.update_var("myvar", VariableValue::INT(5));

        // TODO: check that the value is as expected
    }

    #[test]
    #[should_panic(expected = "Cannot change the type of a variable")]
    fn update_change_type_fails() {
        let mut prog_context = ProgContext::new();
        prog_context.add_var(
            String::from("myvar"),
            VariableKind::NORMAL,
            VariableValue::INT(4),
        );

        prog_context.update_var("myvar", VariableValue::REAL(5.0));
    }

    #[test]
    #[should_panic(expected = "A variable already exists with this name")]
    fn add_second_variable_same_name_fails() {
        let mut prog_context = ProgContext::new();
        prog_context.add_var(
            String::from("myvar"),
            VariableKind::NORMAL,
            VariableValue::INT(4),
        );

        prog_context.add_var(
            String::from("MyVAR"),
            VariableKind::NORMAL,
            VariableValue::INT(4),
        );
    }

    /// Tests parser on all st files within a specified folder
    fn parser_batch_test_st_folder(folder_path: &str) {
        let paths = fs::read_dir(folder_path).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let path_name = path.to_str().unwrap();

            println!("Name: {}", path_name);

            let file = read_file(path_name);
            let parse_result = parser::ProgramParser::new().parse(&file);

            println!("{:?}\n", parse_result);
            assert!(parse_result.is_ok());
        }
    }
}
