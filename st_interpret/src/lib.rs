//! MIT-licensed IEC 61131-3 Structured Text interpreter library.

extern crate core;
#[macro_use]
extern crate lalrpop_util;

use std::collections::HashSet;
use std::fs;

use crate::prog_handle::InterpreterResult;
pub use ast::VariableValue;

mod ast;
mod capi;
pub mod prog_handle;

lalrpop_mod!(pub parser);

/// Simple example function used for testing tests and C integration
pub fn lib_function_example_add(num_one: usize, num_two: usize) -> usize {
    num_one + num_two
}

/// Check that the parser accepts a valid file
pub fn parser_test() -> bool {
    parser::ProgramParser::new()
        .parse(
            &mut HashSet::new(),
            &read_file("tests/test_inputs/st_subset_1/01_Int.st").unwrap(),
        )
        .is_ok()
}

/// Read in the contents of a file to a String
pub fn read_file(file_path: &str) -> InterpreterResult<String> {
    let contents = fs::read_to_string(file_path);
    if let Ok(contents) = contents {
        InterpreterResult::Ok(contents)
    } else {
        InterpreterResult::Err(String::from("Unable to read file"))
    }
}

/// Unit tests for interpreter
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs;

    use crate::ast::{Function, VariableKind, VariableValue};
    use crate::prog_handle::{st_program_load, st_program_run, ProgContext, VariableInfo};
    use crate::{lib_function_example_add, parser, read_file};

    #[test]
    /// Check parser succeeds over subset 1 ST programs.
    fn subset1_parse() {
        parser_batch_test_st_folder("tests/test_inputs/st_subset_1");
    }

    #[test]
    /// Check parser succeeds over subset 3 and 4 ST programs.
    fn subset3_4_parse() {
        parser_batch_test_st_folder("tests/test_inputs/st_subset_3-4");
    }

    #[test]
    /// Check parser succeeds over subset 5 and 6 ST programs.
    fn subset5_6_parse() {
        parser_batch_test_st_folder("tests/test_inputs/st_subset_5-6");
    }

    #[test]
    /// Check parser succeeds over subset 7 ST programs.
    fn subset7_parse() {
        parser_batch_test_st_folder("tests/test_inputs/st_subset_7");
    }

    #[test]
    /// Check parser succeeds over subset 8 and 9 ST programs.
    fn subset8_9_parse() {
        parser_batch_test_st_folder("tests/test_inputs/st_subset_8-9");
    }

    #[test]
    /// Check parser succeeds over subset 9 ST functions
    fn subset9_func_parse() {
        parser_test_st_function("tests/test_inputs/st_subset_9_funcs/SimpleAddFunction.st");
        parser_test_st_function("tests/test_inputs/st_subset_9_funcs/ValueAverageFunction.st");
    }

    #[test]
    /// Check parser succeeds over parts of the ST language not currently included in a target subset.
    fn stretch_parse() {
        parser_batch_test_st_folder("tests/test_inputs/st_stretch");
    }

    #[test]
    #[should_panic(expected = "assertion failed: parse_result.is_ok()")]
    /// Check parser correctly finds incorrect st files
    fn fail_parse() {
        parser_batch_test_st_folder("tests/test_inputs/st_failure/parse_fail");
    }

    #[test]
    /// Check subset 1 programs execute correctly
    fn execute_subset_1() {
        interpreter_batch_test_st_folder("tests/test_inputs/execution/st_subset_1");
    }

    #[test]
    /// Check subset 3-4 programs execute correctly
    fn execute_subset_3_4() {
        interpreter_batch_test_st_folder("tests/test_inputs/execution/st_subset_3-4");
    }

    #[test]
    /// Check subset 5-6 programs execute correctly
    fn execute_subset_5_6() {
        interpreter_batch_test_st_folder("tests/test_inputs/execution/st_subset_5-6");
    }

    #[test]
    /// Check subset 7 programs execute correctly
    fn execute_subset_7() {
        interpreter_batch_test_st_folder("tests/test_inputs/execution/st_subset_7");
    }

    #[test]
    /// Check stretch (no assigned subset) programs execute correctly
    fn execute_stretch() {
        interpreter_batch_test_st_folder("tests/test_inputs/execution/st_stretch");
    }

    #[test]
    /// Example of a basic interpretation test
    fn execute_basic() {
        let mut prog_handle = st_program_load(
            "tests/test_inputs/st_subset_1/01_mixed.st",
            ProgContext::new(),
        )
        .unwrap();
        st_program_run(&mut prog_handle).unwrap();
        assert_eq!(
            prog_handle
                .context
                .get_var(String::from("a"))
                .unwrap()
                .var_value,
            VariableValue::REAL(1.2)
        );
        assert_eq!(
            prog_handle
                .context
                .get_var(String::from("b"))
                .unwrap()
                .var_value,
            VariableValue::INT(5)
        );
        assert_eq!(
            prog_handle
                .context
                .get_var(String::from("c"))
                .unwrap()
                .var_value,
            VariableValue::BOOL(true)
        );
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
        let result = read_file("tests/test_inputs/misc/read_test.txt").unwrap();
        assert_eq!(result.is_empty(), false);
        assert_eq!(result, "Hello World!");
    }

    #[test]
    /// Test adding an int variable to a ProgContext
    fn test_add_var_int() {
        let mut prog_context = ProgContext::new();
        prog_context
            .add_var(
                String::from("variable"),
                VariableKind::NORMAL,
                VariableValue::INT(10),
            )
            .unwrap();
        let result = prog_context.get_var(String::from("variable"));
        let value = match result {
            None => VariableValue::INT(0),
            Some(v) => v.var_value.clone(),
        };
        assert_eq!(VariableValue::INT(10), value);
        let kind = match result {
            None => VariableKind::INPUT,
            Some(v) => v.var_kind,
        };
        assert_eq!(VariableKind::NORMAL, kind);
    }

    #[test]
    /// Test adding a bool variable to a ProgContext
    fn test_add_var_bool() {
        let mut prog_context = ProgContext::new();
        prog_context
            .add_var(
                String::from("variable"),
                VariableKind::NORMAL,
                VariableValue::BOOL(false),
            )
            .unwrap();
        let result = prog_context.get_var(String::from("variable"));
        let value = match result {
            None => VariableValue::BOOL(true),
            Some(v) => v.var_value.clone(),
        };
        assert_eq!(VariableValue::BOOL(false), value);
        let kind = match result {
            None => VariableKind::INPUT,
            Some(v) => v.var_kind,
        };
        assert_eq!(VariableKind::NORMAL, kind);
    }

    #[test]
    /// Test adding a real variable to a ProgContext
    fn test_add_var_real() {
        let mut prog_context = ProgContext::new();
        prog_context
            .add_var(
                String::from("variable"),
                VariableKind::NORMAL,
                VariableValue::REAL(1.5),
            )
            .unwrap();
        let result = prog_context.get_var(String::from("variable"));
        let value = match result {
            None => VariableValue::REAL(0.0),
            Some(v) => v.var_value.clone(),
        };
        assert_eq!(VariableValue::REAL(1.5), value);
        let kind = match result {
            None => VariableKind::INPUT,
            Some(v) => v.var_kind,
        };
        assert_eq!(VariableKind::NORMAL, kind);
    }

    #[test]
    /// Test adding multiple variables to a ProgContext
    fn test_get_vars() {
        let mut prog_context = ProgContext::new();
        prog_context
            .add_var(
                String::from("variable0"),
                VariableKind::NORMAL,
                VariableValue::REAL(1.5),
            )
            .unwrap();
        prog_context
            .add_var(
                String::from("variable1"),
                VariableKind::NORMAL,
                VariableValue::BOOL(false),
            )
            .unwrap();
        prog_context
            .add_var(
                String::from("variable2"),
                VariableKind::NORMAL,
                VariableValue::INT(10),
            )
            .unwrap();
        let v0 = VariableInfo {
            var_value: VariableValue::REAL(1.5),
            var_kind: VariableKind::NORMAL,
        };
        let v1 = VariableInfo {
            var_value: VariableValue::BOOL(false),
            var_kind: VariableKind::NORMAL,
        };
        let v2 = VariableInfo {
            var_value: VariableValue::INT(10),
            var_kind: VariableKind::NORMAL,
        };
        let s0 = String::from("variable0");
        let s1 = String::from("variable1");
        let s2 = String::from("variable2");
        let vars = prog_context.get_all_vars();
        for (name, var) in vars {
            println!("{}", name);
            if name.eq(&s0) {
                println!("matched {}", s0);
                assert_eq!(v0, *var);
            } else if name.eq(&s1) {
                println!("matched {}", s1);
                assert_eq!(v1, *var);
            } else if name.eq(&s2) {
                println!("matched {}", s2);
                assert_eq!(v2, *var);
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    /// Test updating a variable works
    fn update_var() {
        let mut prog_context = ProgContext::new();
        prog_context
            .add_var(
                String::from("myvar"),
                VariableKind::NORMAL,
                VariableValue::INT(4),
            )
            .unwrap();

        prog_context
            .update_var("myvar", VariableValue::INT(5))
            .unwrap();

        let result = prog_context.get_var(String::from("myvar"));
        let value = match result {
            None => VariableValue::INT(0),
            Some(v) => v.var_value.clone(),
        };
        assert_eq!(VariableValue::INT(5), value);
        let kind = match result {
            None => VariableKind::INPUT,
            Some(v) => v.var_kind,
        };
        assert_eq!(VariableKind::NORMAL, kind);
    }

    #[test]
    fn update_change_type_fails() {
        let mut prog_context = ProgContext::new();
        prog_context
            .add_var(
                String::from("myvar"),
                VariableKind::NORMAL,
                VariableValue::BOOL(true),
            )
            .unwrap();

        assert!(prog_context
            .update_var("myvar", VariableValue::REAL(5.0))
            .unwrap_err()
            .contains("Cannot change the type of a variable"));
    }

    #[test]
    fn update_change_type_truncate() {
        let mut prog_context = ProgContext::new();
        prog_context
            .add_var(
                String::from("myvar"),
                VariableKind::NORMAL,
                VariableValue::INT(1),
            )
            .unwrap();

        let _result = prog_context.update_var("myvar", VariableValue::REAL(3.9));
        assert!(
            prog_context.get_var("myvar".to_string()).unwrap().var_value == VariableValue::INT(3)
        );
    }

    #[test]
    fn update_change_type_int_real() {
        let mut prog_context = ProgContext::new();
        prog_context
            .add_var(
                String::from("myvar"),
                VariableKind::NORMAL,
                VariableValue::REAL(1.3),
            )
            .unwrap();

        let _result = prog_context.update_var("myvar", VariableValue::INT(5));
        assert!(
            prog_context.get_var("myvar".to_string()).unwrap().var_value
                == VariableValue::REAL(5.0)
        );
    }

    #[test]
    fn run_program() {
        let context = ProgContext::new();
        let mut prog_handle =
            st_program_load("tests/test_inputs/st_subset_1/01_Int.st", context).unwrap();
        st_program_run(&mut prog_handle).unwrap();
    }

    #[test]
    fn add_second_variable_same_name_fails() {
        let mut prog_context = ProgContext::new();
        prog_context
            .add_var(
                String::from("myvar"),
                VariableKind::NORMAL,
                VariableValue::INT(4),
            )
            .unwrap();

        assert!(prog_context
            .add_var(
                String::from("MyVAR"),
                VariableKind::NORMAL,
                VariableValue::INT(4),
            )
            .unwrap_err()
            .contains("A variable already exists with this name"));
    }

    /// Tests parser on all st files within a specified folder
    fn parser_batch_test_st_folder(folder_path: &str) {
        let paths = fs::read_dir(folder_path).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let path_name = path.to_str().unwrap();

            println!("Name: {}", path_name);

            let file = read_file(path_name).unwrap();
            let parse_result = parser::ProgramParser::new().parse(&mut HashSet::new(), &file);

            println!("{:?}\n", parse_result);
            assert!(parse_result.is_ok());
        }
    }

    /// Test execution of all ST files in a folder.
    /// Simple runs the each program and asserts it contains a boolean variable 'ST_TESTING_RESULT'
    /// that is true after execution completes. This allows creating ST example files that
    /// essentially include their own assertions about their functionality.
    fn interpreter_batch_test_st_folder(folder_path: &str) {
        println!("Executing all ST files in folder {}", folder_path);
        let paths = fs::read_dir(folder_path).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let file_name = path.to_str().unwrap();
            println!("Executing file {}", file_name);

            let mut prog_handle = st_program_load(file_name, ProgContext::new()).unwrap();
            st_program_run(&mut prog_handle).unwrap();

            println!("Program handle dump: {:?}", prog_handle);
            assert_eq!(
                prog_handle
                    .context
                    .get_var(String::from("ST_TESTING_RESULT"))
                    .unwrap()
                    .var_value,
                VariableValue::BOOL(true)
            );
        }
    }

    fn parser_test_st_function(file_path: &str) {
        println!("Name: {}", file_path);

        let file = read_file(file_path).unwrap();
        let parse_result = parser::FunctionParser::new().parse(&mut HashSet::new(), &file);

        println!("{:?}\n", parse_result);
        assert!(parse_result.is_ok());
    }
}
